use drop_util::table::Table;

use crate::{
    tdir::{Func, Instruction, InstructionKind, Ref},
    udir::{self, UDIR},
};

pub type TDIR = Table<Instruction>;

#[derive(Debug)]
pub enum GenerateError {
    TypeFuncRetNotInFunction(),
}

type GenerateResult<T> = Result<T, GenerateError>;

pub struct TDIRGen<'a> {
    udir: &'a UDIR,
    tdir: TDIR,
}

impl<'a> TDIRGen<'a> {
    pub fn new(udir: &'a UDIR) -> Self {
        Self {
            udir,
            tdir: TDIR::new(),
        }
    }

    fn gen_inst(&mut self, inst: &udir::Instruction, func: Option<&Func>) -> GenerateResult<Ref> {
        match inst {
            udir::Instruction::Func(func) => {
                let inst = InstructionKind::Func(self.gen_func(func)?).into();
                let inst = self.tdir.insert(inst).into();
                Ok(Ref::Index(inst))
            }
            udir::Instruction::Number(number) => {
                let mut inst: Instruction = InstructionKind::Number(*number).into();
                inst.ty = Some(Ref::TypeI32());

                let inst = self.tdir.insert(inst).into();
                Ok(Ref::Index(inst))
            }
            udir::Instruction::Coerce(coerce) => {
                let Ref::Index(index) = self.gen_ref(&coerce.val, func)? else {
                    panic!();
                };

                let ty = self.tdir[index].ty.unwrap();
                let into_ty = self.gen_ref(&coerce.ty, func)?;

                assert!(ty == into_ty, "not implemented for now");

                Ok(Ref::Index(index))
            }
            udir::Instruction::Block(block) => {
                let mut new_block = Vec::<Ref>::new();
                for inst in block.iter() {
                    new_block.push(self.gen_ref(inst, func)?);
                }

                let inst: Instruction = InstructionKind::Block(new_block).into();
                let inst = self.tdir.insert(inst);
                Ok(Ref::Index(inst))
            }
            udir::Instruction::Ret(val) => {
                let inst: Instruction = InstructionKind::Return(self.gen_ref(val, func)?).into();
                let inst = self.tdir.insert(inst);
                Ok(Ref::Index(inst))
            }
            _ => unimplemented!("{:?}", inst),
        }
    }

    fn gen_ref(&mut self, r#ref: &udir::Ref, func: Option<&Func>) -> GenerateResult<Ref> {
        match r#ref {
            udir::Ref::TypeI32() => Ok(Ref::TypeI32()),
            udir::Ref::TypeFuncRet() => match func {
                Some(func) => Ok(func.return_type),
                None => Err(GenerateError::TypeFuncRetNotInFunction()),
            },
            udir::Ref::Index(i) => self.gen_inst(&self.udir[*i], func),
            _ => unimplemented!("{:?}", r#ref),
        }
    }

    fn gen_func(&mut self, func: &udir::Func) -> GenerateResult<Func> {
        let mut gen_args = Vec::<(String, Ref)>::new();
        for arg in func.args.iter() {
            gen_args.push((arg.0.clone(), self.gen_ref(&arg.1, None)?));
        }
        let return_type = self.gen_ref(&func.return_type, None)?;

        let mut translated_func = Func {
            is_extern: func.is_extern,
            name: func.name.clone(),
            args: gen_args,
            return_type: return_type,
            body: None,
        };

        if let Some(body) = &func.body {
            translated_func.body = Some(self.gen_ref(body, Some(&translated_func))?);
        }

        Ok(translated_func)
    }

    pub fn generate(&mut self) -> GenerateResult<()> {
        for inst in self.udir.iter() {
            if matches!(inst, udir::Instruction::Func(_)) {
                self.gen_inst(inst, None)?;
            }
        }

        Ok(())
    }
}

pub fn generate(udir: &UDIR) -> GenerateResult<TDIR> {
    let mut tdirgen = TDIRGen::new(udir);
    tdirgen.generate()?;
    Ok(tdirgen.tdir)
}
