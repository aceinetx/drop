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

    printf("- " sv_fmt "\n", sv_arg(name));
  }
  //
}

drop::IR1Gen drop::IR1Gen::init(AST *ast) {
  return {
      IR1::init(),
      ast,
  };
}

drop::IR1 drop::generate_ir1(AST *ast) {
  IR1Gen gen = IR1Gen::init(ast);
  return gen.ir;
}
