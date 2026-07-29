use crate::parser::{ExpressionId, Parser, RootAst};

enum Value {
    Unary {
        child: ExpressionId,
        op: Operation,
    },
    Binary {
        lhs: ExpressionId,
        rhs: ExpressionId,
        op: Operation,
    },
    FuncCall {
        name: ExpressionId,
        args: Vec<ExpressionId>,
    },
    ArrAcc {
        name: ExpressionId,
        index: ExpressionId,
    },
    MemAcc {
        name: ExpressionId,
        member: String,
    },

    UNDEF,
}

enum Operation {
    Eq,

    LogNot, // unary
    LogOr,
    LogAnd,
    LogEq,
    NotEq,
    LessThan,
    MoreThan,
    LessEq,
    MoreEq,

    BitOr,
    BitAnd,
    BitXor,
    BitNeg, // unary

    LShift,
    RShift,

    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug)]
pub enum Instruction {

}

pub struct IrConstructor {}

impl IrConstructor {
    pub fn new(parser: Parser, root: &RootAst) -> Vec<Instruction> {
        for decl_id in root {
            eprintln!("{:#?}", parser.declaration_arena[*decl_id]);
        }
        Vec::new()
    }

    // fn 
}
