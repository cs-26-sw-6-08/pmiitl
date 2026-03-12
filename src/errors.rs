use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("hime parse error")]
    HimeParse,
    #[error("ast node invalid: {0}")]
    ASTNodeValueInvalid(String),
    #[error("program parse error line {1} column {2}: {0}")]
    ProgramParse(String, usize, usize),
    #[error("conversion error")]
    Conversion,
}