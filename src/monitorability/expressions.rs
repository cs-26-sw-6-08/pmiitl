use std::error::Error;

use crate::program::expressions::Expr;
use crate::errors;
use crate::program::function_types::FunctionType;

impl Expr {
    pub fn monitorability_check(&self) -> Result<(), Box<dyn Error>> {
        self.internal_monitorability_check(false)
    }

    fn internal_monitorability_check(&self, inside_temporal_aggregate: bool) -> Result<(), Box<dyn Error>> {
        match self {
            Expr::Number(_) | Expr::String(_) | Expr::CurrentTime | Expr::Member { access_type:_ } => Ok(()),
            Expr::Interval { start: _, end: _ } => Ok(()),
            Expr::Always { not: false, expr, .. } => {
                if inside_temporal_aggregate {
                    return Err(errors::Error::OnlyForeachTemporalExpressionAllowed.into())
                }
                expr.internal_monitorability_check(inside_temporal_aggregate)?;
                Ok(())
            },
            Expr::Eventually { interval, expr, .. } => {
                 if inside_temporal_aggregate {
                    return Err(errors::Error::OnlyForeachTemporalExpressionAllowed.into())
                } else if interval.is_some() {
                    expr.internal_monitorability_check(inside_temporal_aggregate)?;
                    return Ok(())
                }
                Err(errors::Error::Unmonitorable(self.clone()).into())
            },
            Expr::BinaryOperations { lhs, rhs, .. } => {
                lhs.internal_monitorability_check(inside_temporal_aggregate)?;
                rhs.internal_monitorability_check(inside_temporal_aggregate)?;
                Ok(())
            },
            Expr::UnaryOperations { operand, operator:_ } => {
                operand.internal_monitorability_check(inside_temporal_aggregate)?;
                Ok(())
            },
            Expr::Function { aggregate_type, expr, bound: _ } => {
                if (aggregate_type.eq(&FunctionType::Sumtime) || aggregate_type.eq(&FunctionType::Avgtime) || aggregate_type.eq(&FunctionType::Counttime)) && inside_temporal_aggregate {
                        return Err(errors::Error::OnlyForeachTemporalExpressionAllowed.into())
                } else if aggregate_type.ne(&FunctionType::Foreach) {
                    expr.internal_monitorability_check(true)?;
                } else {
                    expr.internal_monitorability_check(inside_temporal_aggregate)?;
                }
                Ok(())
            },
            _ => Err(errors::Error::Unmonitorable(self.clone()).into())
        }
    }
}