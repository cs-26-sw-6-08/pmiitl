use crate::program::{expressions::ExprKind, operations::BinaryOperators};
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
    ConversionBinaryOperation(BinaryOperators, ExprKind, ExprKind),
    #[error("type error")]
    Typechecking,
}
