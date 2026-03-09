use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("hime parse error")]
    HimeParseError,
    #[error("ast node invalid: {0}")]
    ASTNodeValueInvalid(String),
    #[error("program parse error line {1} column {2}: {0}")]
    ProgramParseError(String, usize, usize)
}