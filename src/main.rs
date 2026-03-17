mod grammar;// default namespace for the parser is the grammar's name
pub mod program;
pub mod unit_checker;
pub mod equivalence_converter;
pub mod unit_converter;
pub mod utils;
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
    let mut program = match Program::new(program_str.as_str()) {
        Ok(program) => program,
        Err(err) => return println!("Error: {}", err),
    };

    if let Err(err) = program.unit_converter() {
        return println!("Error: {}", err);
    };

    if let Err(err) = program.unit_check() {
        return println!("Error: {}", err);
    }

    if let Err(err) = program.convert() {
        return println!("Error: {}", err);
    };

    println!("{:?}", program);

    //print(root, Vec::<bool>::new());
}
