use std::error::Error;

use hime_redist::ast::AstNode;

use crate::{errors, grammar::cfg, program::expressions::SpannedExpr};
pub mod expressions;
pub mod units;
pub mod operations;
pub mod function_types;
pub mod member_types;
#[cfg(test)]
mod program_test;
#[cfg(test)]
mod conversion_test;
mod conversion_binary_operator;
mod conversion_units;

#[derive(Debug)]
pub struct Program {
    pub expressions: Vec<SpannedExpr>
}

impl Program {

    pub fn new(programstr: &str) -> Result<Self, Box<dyn Error>>{
        let parsed = cfg::parse_string(programstr.to_lowercase());
        let mut exprs : Vec<SpannedExpr> = Vec::new();
        if !parsed.is_success() {
            return Err(errors::Error::HimeParse.into())
        }
        let ast = parsed.get_ast();
        let root = ast.get_root();

        print(root, Vec::<bool>::new());

        for node in root.children(){
            exprs.push(SpannedExpr::new(node)?);

        }

        let program = Program { expressions: exprs};
        Ok(program)
    }

    pub fn convert(&mut self) -> Result<(), Box<dyn Error>> {
        for spanned_expr in self.expressions.iter_mut() {
            spanned_expr.expr = spanned_expr.expr.convert()?;
        }
        Ok(())
    }

}

fn print<'a>(node: AstNode<'_,'_,'a>, crossings: Vec<bool>) {
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
