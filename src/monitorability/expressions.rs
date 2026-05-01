use std::error::Error;

use crate::program::expressions::Expr;
use crate::errors;

impl Expr {
    pub fn monitorability_check(&self) -> Result<(), Box<dyn Error>> {
        match self {
            Expr::Number(_) | Expr::String(_) | Expr::CurrentTime | Expr::Member { access_type:_ } => Ok(()),
            Expr::Interval { start: _, end: _ } => Ok(()),
            Expr::Always { interval, not: false, expr } => {
                if interval.is_some() {
                    return Ok(());
                }
                expr.monitorability_check()?;
                Ok(())
            },
            Expr::Eventually { interval, .. } => {
                if interval.is_some() {
                    return Ok(());
                }
                Err(errors::Error::Unmonitorable(self.clone()).into())
            },
            Expr::BinaryOperations { lhs, rhs, operator: _ } => {
                lhs.monitorability_check()?;
                rhs.monitorability_check()?;
                Ok(())
            },
            Expr::UnaryOperations { operand, operator:_ } => {
                operand.monitorability_check()?;
                Ok(())
            },
            Expr::Function { aggregate_type:_, expr, bound: _ } => {
                expr.monitorability_check()?;
                Ok(())
            },
            _ => Err(errors::Error::Unmonitorable(self.clone()).into())
        }
    }
}