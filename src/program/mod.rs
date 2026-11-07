use std::error::Error;

use hime_redist::{ast::AstNode, errors::ParseErrorDataTrait};

use crate::{errors, grammar::cfg, monitor::streams::OutputStream, program::expressions::SpannedExpr};
pub mod expressions;
pub mod units;
pub mod operations;
pub mod function_types;
pub mod member_types;
#[cfg(test)]
mod program_test;

#[derive(PartialEq, Debug)]
pub struct Program {
    pub expressions: Vec<SpannedExpr>,
    pub environment: Option<Vec<OutputStream>>,
    pub iotstream_len: Option<usize>
}

impl Program {
    pub fn new(programstr: &str) -> Result<Self, Box<dyn Error>>{
        let parsed = cfg::parse_string(programstr.to_lowercase());
        let mut exprs : Vec<SpannedExpr> = Vec::new();
        if !parsed.is_success() {
            // Report the first parse error with its position, so callers (CLI or frontend) can show it
            return Err(match parsed.errors.errors.first() {
                Some(error) => errors::Error::ProgramParse(
                    error.to_string(),
                    error.get_position().line,
                    error.get_position().column,
                ).into(),
                None => errors::Error::HimeParse.into(),
            });
        }
        let ast = parsed.get_ast();
        let root = ast.get_root();

        for node in root.children(){
            exprs.push(SpannedExpr::new(node)?);

        }

        let program = Program { expressions: exprs, environment: None, iotstream_len: None };
        Ok(program)
    }
}

#[allow(dead_code)]
fn print(node: AstNode<'_,'_,'_>, crossings: Vec<bool>) {
    let mut i = 0;
    if !crossings.is_empty() {
        while i < crossings.len() - 1 {
            print!("{:}", if crossings[i] { "|   " } else { "    " });
            i += 1;
        }
        print!("+-> ");
    }
    println!("{:}", node);
    i = 0;
    let children = node.children();
    while i < children.len() {
        let mut child_crossings = crossings.clone();
        child_crossings.push(i < children.len() - 1);
        print(children.at(i), child_crossings);
        i += 1;
    }
}
