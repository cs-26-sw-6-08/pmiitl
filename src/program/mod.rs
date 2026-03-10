use std::error::Error;

use hime_redist::ast::AstNode;

use crate::{grammar::cfg, program::expressions::Expr, errors};
mod expressions;
mod units;
mod operations;
#[cfg(test)]
mod expressions_test;

#[derive(Debug)]
pub struct Program {
    pub expressions: Vec<Expr>
}

impl Program {

    pub fn new(programstr: &str) -> Result<Self, Box<dyn Error>>{
        let parsed = cfg::parse_str(programstr);
        let mut exprs : Vec<Expr> = Vec::new();
        if !parsed.is_success() {
            return Err(errors::Error::HimeParseError.into())
        }
        let ast = parsed.get_ast();
        let root = ast.get_root();

        print(root, Vec::<bool>::new());

        for node in root.children(){
            exprs.push(Expr::new(node)?);

        }
        Ok(Program { expressions: exprs})
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
