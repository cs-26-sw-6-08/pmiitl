mod grammar;// default namespace for the parser is the grammar's name
pub mod program;
pub mod unit_check;
pub mod equiv_convert;
pub mod unit_convert;
pub mod monitorability;
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
   
    let mut program = match Program::new(program_str.as_str()) {
        Ok(program) => program,
        Err(err) => return println!("Error: {}", err),
    };

    if let Err(err) = program.unit_convert() {
        return println!("Error: {}", err);
    };

    if let Err(err) = program.unit_check() {
        return println!("Error: {}", err);
    }

    if let Err(err) = program.equiv_convert() {
        return println!("Error: {}", err);
    };

    if let Err(err) = program.monitorability_check() {
        return println!("Error: {}", err);
    }

    println!("{:?}", program);  

}
