use std::error::Error;

use crate::program::Program;
mod expressions;
mod types;
#[cfg(test)]
mod expressions_test;

impl Program {
    pub fn unit_check(&self) -> Result<(), Box<dyn Error>> {
        for spanned_expr in self.expressions.iter() {
            spanned_expr.expr.unit_check()?;
        }
        Ok(())
    }
}