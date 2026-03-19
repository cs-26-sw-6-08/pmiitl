use std::error::Error;

use crate::program::Program;
mod expressions;
#[cfg(test)]
mod expressions_test;

impl Program {
    pub fn unit_convert(&mut self) -> Result<(), Box<dyn Error>> {
        for spanned_expr in self.expressions.iter_mut() {
            spanned_expr.expr = spanned_expr.expr.unit_convert();
        }
        Ok(())
    }
}