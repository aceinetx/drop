#include "ir1.h"
#include "string_format_arg.h"

#define X(ident) drop::strlit(#ident),
drop::string drop::instruction_type_names[] = {XInstructionTypes};
#undef X

drop::IR1 drop::IR1::init() {
  return {
      List<Instruction>::init(),
      List<List<usize>>::init(),
      List<IR1FuncDef>::init(),
      List<string>::init(),
  };
}

void drop::IR1::deinit() {
  for (usize i = 0; i < strings.items.len; i++) {
    free(strings.items[i]);
  }
  strings.deinit();

  for (usize i = 0; i < lists_of_insts.items.len; i++) {
    lists_of_insts.items[i].deinit();
  }
  lists_of_insts.deinit();

  for (usize i = 0; i < funcdefs.items.len; i++) {
    funcdefs.items[i].args.deinit();
  }
  funcdefs.deinit();

  insts.deinit();
}

void drop::IR1::dump() {
  puts("IR1 dump");

  insts.append({InstructionType::Number, {69}});

  for (usize i = 0; i < insts.items.len; i++) {
    const auto inst = &insts.items[i];
    const auto name = instruction_type_names[(usize)inst->tag];

    printf("- " sv_fmt " ", sv_arg(name));

    switch (inst->tag) {
    case InstructionType::Number:
      printf("%ld", inst->data.number);
      break;
    default:
      break;
    }

    putchar('\n');
  }
}

drop::IR1Gen drop::IR1Gen::init(AST *ast) {
  return {
      IR1::init(),
      ast,
      List<Scope>::init(),
  };
}

static void push_scope(drop::IR1Gen *gen) {
  gen->scopes.items[gen->scopes.append({})].init(128);
  //
}

static void pop_scope(drop::IR1Gen *gen) {
  gen->scopes.pop().deinit();
  //
}

static bool gen_node(drop::IR1Gen *gen, drop::ASTNode *node,
                     drop::IR1GenDiagnostics *diag,
                     drop::IR1FunctionContext *functx) {
  switch (node->type) {
  case drop::ASTNodeType::Root: {
    auto block = gen->ast->node_arrays.items[node->data.node_array].items;

    for (size_t i = 0; i < block.len; i++) {
      auto node = &gen->ast->nodes.items[block[i]];

      if (!gen_node(gen, node, diag, functx))
        return false;
    }
  } break;
  case drop::ASTNodeType::FuncDef: {
    auto funcdef = &gen->ast->funcdefs.items[node->data.funcdef];

  } break;
  default:
    drop_trap();
  }
  return true;
}

bool drop::generate_ir1(AST *ast, IR1 *ir1, IR1GenDiagnostics *diag) {
  IR1Gen gen = IR1Gen::init(ast);
  if (!gen_node(&gen, &gen.ast->nodes.items[gen.ast->root], diag, nullptr)) {
    return false;
  }
  *ir1 = gen.ir;
  return true;
}
