mod grammar;// default namespace for the parser is the grammar's name
pub mod program;
mod errors;

extern crate hime_redist;

use std::fs;

use crate::program::Program;

fn main() {
    let program_str = match fs::read_to_string("program.txt") {
        Ok(program_str) => program_str,
        Err(err) => return println!("Error: {}", err)
    };
    /*let result = grammar::cfg::parse_string("
    always true; eventually 7;
".to_string());
    let ast = result.get_ast();
    let root = ast.get_root();*/
    let program = match Program::new(program_str.as_str()) {
        Ok(program) => program,
        Err(err) => return println!("Error: {}", err),
    };

    println!("{:?}", program);

    //print(root, Vec::<bool>::new());
}
