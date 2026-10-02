use drop_util::table::Table;

use crate::{tdir::Instruction, udir::UDIR};

pub type TDIR = Table<Instruction>;

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

    pub fn generate(&mut self) {}
}

pub fn generate(udir: &UDIR) -> TDIR {
    let mut tdirgen = TDIRGen::new(udir);
    tdirgen.generate();
    tdirgen.tdir
}
