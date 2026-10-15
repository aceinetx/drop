#include "ir1.h"
#include "string_format_arg.h"

#define X(ident) drop::strlit(#ident),
drop::string drop::instruction_type_names[] = {XInstructionTypes};
#undef X

drop::IR1 drop::IR1::init() {
  return {
      List<Instruction>::init(),
      List<string>::init(),
  };
}

void drop::IR1::deinit() {
  for (usize i = 0; i < strings.items.len; i++) {
    free(strings.items[i]);
  }
  strings.deinit();

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
  };
}

static bool gen_node(drop::IR1Gen *gen, drop::ASTNode *node,
                     drop::IR1GenDiagnostics *diag) {
  return true;
}

bool drop::generate_ir1(AST *ast, IR1 *ir1, IR1GenDiagnostics *diag) {
  IR1Gen gen = IR1Gen::init(ast);
  if (!gen_node(&gen, &gen.ast->nodes.items[gen.ast->root], diag)) {
    return false;
  }
  *ir1 = gen.ir;
  return true;
}
