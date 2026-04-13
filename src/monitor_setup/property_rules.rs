use std::error::Error;

use crate::{monitor_setup::streams::{LTL, OutputStream, Streams}, program::expressions::{Expr, SpannedExpr}};



pub fn compile_properties(exprs: &Vec<SpannedExpr>) -> Result<Vec<OutputStream>, Box<dyn Error>> {

    Ok(exprs
        .iter()
        .map(|span_expr| &span_expr.expr)
        .map(|expr| {
            match expr {
                Expr::Always { interval, expr, ..} | 
                Expr::Eventually { interval, expr, ..} => {
                    let bound = interval.as_ref().map(|expr| {
                        let Expr::Interval{ start, end} = expr.as_ref() else { unreachable!() };
                        let (Expr::Number(start), Expr::Number(end)) = (start.as_ref(), end.as_ref()) else { unreachable!() };

                        (*start, *end)
                    });
                    let ltl = if let Expr::Always{..} = expr.as_ref() { LTL::Always } else { LTL::Eventually };
                    
                    let operations = expr.compile_expression();
                    
                    (ltl, operations, bound).into()
                    
                },
                _ => unreachable!() //todo: overvej custom error 
            }
        }).collect::<Vec<OutputStream>>()
    )
}