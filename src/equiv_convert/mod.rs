use std::error::Error;

use crate::program::Program;

mod expressions;
#[cfg(test)]
mod binary_operations_test;
mod binary_operations;

impl Program {
    pub fn equiv_convert(&mut self) -> Result<(), Box<dyn Error>> {
        for spanned_expr in self.expressions.iter_mut() {
            spanned_expr.expr.equiv_convert()?;
        }
        Ok(())
    }
}