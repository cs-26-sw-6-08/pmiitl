use std::error::Error;

use crate::{monitor_setup::streams::Streams, program::expressions::{SpannedExpr, Expr}};



pub fn eval_properties(exprs: &Vec<SpannedExpr>) -> Result<Streams, Box<dyn Error>> {
    let mut k = 0;
    let mut env = Streams::new()?;
   
    for expr in exprs {
        let expr = &expr.expr;
        match expr {
            Expr::Always { interval, expr, ..} => {
                let _ = expr.eval_expression(Vec::new(), 0, &env.devices, &env.time_stream);
                todo!()
            },
            Expr::Eventually { interval, expr, ..} => {
                let Some(interval) = interval else { unreachable!() /*todo: overvej custom error*/ };
                
            },
            _ => unreachable!() //todo: overvej custom error 
        }
    }

    todo!()
}