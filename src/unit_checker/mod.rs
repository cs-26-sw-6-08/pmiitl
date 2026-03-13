use std::error::Error;

use crate::program::Program;
mod expressions;
mod types;

impl Program {
    pub fn unit_check(&self) -> Result<(), Box<dyn Error>> {
        for spanned_expr in self.expressions.iter() {
            spanned_expr.expr.unit_check()?;
        }
        Ok(())
    }
}