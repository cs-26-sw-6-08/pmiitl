use std::error::Error;

use crate::program::Program;

mod expressions;
#[cfg(test)]
mod conversion_binary_operator_test;
mod conversion_binary_operator;

impl Program {
    pub fn convert(&mut self) -> Result<(), Box<dyn Error>> {
        for spanned_expr in self.expressions.iter_mut() {
            spanned_expr.expr = spanned_expr.expr.convert()?;
        }
        Ok(())
    }
}