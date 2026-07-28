use crate::parser::ExpressionId;

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

pub struct IrConstructor {}
