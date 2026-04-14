mod rules;

pub mod operation_types;

use std::{error::Error};
use crate::{errors, monitor::streams::OutputStream, monitor_setup::operation_types::LTL, program::{Program, expressions::Expr}};

#[cfg(test)]
mod rules_test;

impl Program {
    pub fn compile_properties(&mut self) -> Result<(), Box<dyn Error>> {
        self.environment = Some(
            self.expressions
            .iter()
            .map(|span_expr| &span_expr.expr)
            .map(|ltl_expr|
                match ltl_expr {
                    Expr::Always { interval, expr, not: false } | 
                    Expr::Eventually { interval, expr, not: false } => 
                        Ok((
                            match ltl_expr {
                                Expr::Always { .. } => LTL::Always,
                                _ =>                   LTL::Eventually,
                            },
                            expr.compile_expression()?,
                            interval.as_deref().map(Expr::get_bound).transpose()?
                        ).into()),
                    _ => Err(errors::Error::InvalidCompileExpr.into()) 
                }
            ).collect::<Result<Vec<OutputStream>, Box<dyn Error>>>()?
        );
        Ok(())
    }
}






