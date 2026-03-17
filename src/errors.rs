use crate::program::{expressions::Expr, operations::BinaryOperators};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("hime parse error")]
    HimeParse,
    #[error("ast node invalid: {0}")]
    ASTNodeValueInvalid(String),
    #[error("program parse error line {1} column {2}: {0}")]
    ProgramParse(String, usize, usize),
    #[error("could not convert operation {0:?} lhs: {1:?} rhs: {2:?}")]
    ConversionBinaryOperation(BinaryOperators, Expr, Expr),
    #[error("type error")]
    Typechecking,
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::ASTNodeValueInvalid(l0), Self::ASTNodeValueInvalid(r0)) => l0 == r0,
            (Self::ProgramParse(l0, l1, l2), Self::ProgramParse(r0, r1, r2)) => l0 == r0 && l1 == r1 && l2 == r2,
            (Self::ConversionBinaryOperation(l0, l1, l2), Self::ConversionBinaryOperation(r0, r1, r2)) => l0 == r0 && l1 == r1 && l2 == r2,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}
