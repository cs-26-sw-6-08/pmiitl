mod rules;
pub mod streams;
mod types;

use crate::{errors, monitor_setup::streams::{LTL, OutputStream}, program::expressions::Expr};
use std::{error::Error};
use crate::{program::Program};

impl Program {
    pub fn compile_properties(&mut self) -> Result<(), Box<dyn Error>> {
        self.environment = Some(
            self.expressions
            .iter()
            .map(|span_expr| &span_expr.expr)
            .map(|ltl_expr|
                match ltl_expr {
                    Expr::Always { interval, expr, .. } | 
                    Expr::Eventually { interval, expr, .. } => 
                        Ok((
                            match ltl_expr {
                                Expr::Always { .. } => LTL::Always,
                                _ =>                   LTL::Eventually,
                            },
                            expr.compile_expression()?,
                            interval.as_ref().map(|i| i.get_bound()).transpose()?
                        ).into()),
                    _ => Err(errors::Error::InvalidCompileExpr.into()) 
                }
            ).collect::<Result<Vec<OutputStream>, Box<dyn Error>>>()?
        );
        Ok(())
    }
}