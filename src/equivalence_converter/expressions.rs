use std::error::Error;

use crate::{
    equivalence_converter::conversion_binary_operator::conversion_binary_operations,
    program::{expressions::Expr, function_types::FunctionType},
};

impl Expr {
    //minimize tree by calculating simple arithmetic and boolean expressions and making cartain equivalence conversions
    pub fn convert(&self) -> Result<Expr, Box<dyn Error>> {
        match self {
            Expr::BinaryOperations { lhs, rhs, operator } => {
                let lhs = lhs.convert()?;
                let rhs = rhs.convert()?;

                Ok(conversion_binary_operations(lhs, rhs, operator)?)
            }
            Expr::Always {
                interval,
                not,
                expr,
            } => Ok(Expr::Always {
                interval: interval
                    .clone()
                    .and_then(|e| Some(e.convert().ok()?.into())),
                not: *not,
                expr: expr.convert()?.into(),
            }),
            Expr::Unit { number, unit: _ } => Ok(number.convert()?),
            Expr::Function {
                aggregate_type,
                expr,
            } => match aggregate_type {
                FunctionType::Count => {
                    let expr = expr.convert()?;
                    Ok(Expr::Function {
                        aggregate_type: FunctionType::Sum,
                        expr: match expr {
                            Expr::Number(n) => Expr::Boolean(n != 0).into(),
                            Expr::Boolean(n) => Expr::Boolean(n).into(),
                            _ => unreachable!(),
                        },
                    })
                }
                FunctionType::Counttime => {
                    let expr = expr.convert()?;
                    Ok(Expr::Function {
                        aggregate_type: FunctionType::Sumtime,
                        expr: match expr {
                            Expr::Number(n) => Expr::Boolean(n != 0).into(),
                            Expr::Boolean(n) => Expr::Boolean(n).into(),
                            _ => unreachable!(),
                        },
                    })
                }
                _ => Ok(Expr::Function {
                    aggregate_type: aggregate_type.clone(),
                    expr: expr.convert()?.into(),
                }),
            },
            Expr::Interval { start, end } => Ok(Expr::Interval {
                start: start.convert()?.into(),
                end: end.convert()?.into(),
            }),
            Expr::Eventually {
                interval,
                not,
                expr,
            } => Ok(Expr::Eventually {
                interval: interval
                    .clone()
                    .and_then(|e| Some(e.convert().ok()?.into())),
                not: *not,
                expr: expr.convert()?.into(),
            }),
            Expr::UnaryOperations { operand, operator } => Ok(Expr::UnaryOperations {
                operand: operand.convert()?.into(),
                operator: operator.clone(),
            }),
            _ => Ok(self.clone()),
        }
    }
}
