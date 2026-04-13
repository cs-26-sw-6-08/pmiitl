mod rules;
pub mod streams;
mod types;

use crate::{monitor_setup::streams::{LTL}, program::expressions::{Expr}};

use std::{error::Error};
use crate::{program::Program};

impl Program {
    fn compile_properties(&mut self) -> Result<(), Box<dyn Error>> {
        self.environment = Some(
            self.expressions
            .iter()
            .map(|span_expr| &span_expr.expr)
            .map(|expr|
                match expr {
                    Expr::Always { interval, expr, ..} => {                        
                        (
                            LTL::Always, 
                            expr.compile_expression(), 
                            interval.as_ref().and_then(get_bound)
                        ).into()
                    },
                    Expr::Eventually { interval, expr, ..} => {
                        (
                            LTL::Eventually, 
                            expr.compile_expression(), 
                            interval.as_ref().and_then(get_bound)
                        ).into()
                    },
                    _ => unreachable!() //todo: overvej custom error 
                }
            ).collect()
        );
        Ok(())
    }
}

fn get_bound(bound: &Box<Expr>) -> Option<(i128,i128)> {
    let Expr::Interval{ start, end} = bound.as_ref() else { return None };
    let (Expr::Number(start), Expr::Number(end)) = (start.as_ref(), end.as_ref()) else { return None };

    Some((*start, *end))
}