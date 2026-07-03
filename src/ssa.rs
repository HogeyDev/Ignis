pub type BasicBlockId = usize;
pub struct BasicBlock {
    insts: Vec<Instruction>,

    preds: Vec<BasicBlockId>,
    succs: Vec<BasicBlockId>,
}

pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Xor,
    LShift,
    RShift,
    LNot,
    GT,
    GTE,
    LT,
    LTE,
    Eq,
    NotEq,
}
pub enum UnOp {
    Add
}

pub enum Instruction {
    BinExp {
        dest: String,
        op: BinOp,
    }
}
