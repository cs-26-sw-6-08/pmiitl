mod grammar;// default namespace for the parser is the grammar's name
pub mod program;
pub mod unit_check;
pub mod equiv_convert;
pub mod unit_convert;
pub mod monitorability;
pub mod utils;
pub mod monitor_setup;
pub mod monitor;
mod errors;
extern crate hime_redist;
use std::fs;
use colored::Colorize;
use crate::program::Program;


#[tokio::main]
async fn main() {
    let program_str = match fs::read_to_string("program.txt") {
        Ok(program_str) => program_str,
        Err(err) => return error_print(format!("{}",err))
    };
   
    let mut program = match Program::new(program_str.as_str()) {
        Ok(program) => program,
        Err(err) => return error_print(format!("{}",err))
    };

    if let Err(err) = program.unit_convert() {
        return error_print(format!("{}",err));
    };

    if let Err(err) = program.unit_check() {
        return error_print(format!("{}",err));
    }

    if let Err(err) = program.equiv_convert() {
        return error_print(format!("{}",err));
    };
    
    if let Err(err) = program.monitorability_check() {
        return error_print(format!("{}",err));
    }

    if let Err(err) = program.compile_properties() {
        return error_print(format!("{}",err));
    }
    
    if let Err(err) = program.monitor(1_000, true).await {
        return error_print(format!("{}",err));
    }
}

fn error_print(err: String) {
    println!("{}{}",String::from("[Error]: ").bright_red().bold(), err)
}