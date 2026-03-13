use std::error::Error;

use crate::{equivalence_converter::conversion_binary_operator::conversion_binary_operations, program::expressions::ExprKind};

impl ExprKind {
    pub fn convert(&self) -> Result<ExprKind, Box<dyn Error>> {
        match self {
            ExprKind::BinaryOperations { lhs, rhs, operator } => {
                let lhs = lhs.convert()?;
                let rhs = rhs.convert()?;

                Ok(conversion_binary_operations(lhs, rhs, operator)?)
            },
            ExprKind::Always { interval, not, expr } => {
                Ok(ExprKind::Always { interval: interval.clone().and_then(|e| Some(e.convert().ok()?.into())), not: *not, expr: expr.convert()?.into() })
            }

                /*
                let expr = match operator {
                    /* p && q => !(!p || !q) */
                    
                   
                    BinaryOperators::Implies => ExprKind::BinaryOperations {
                        lhs: ExprKind::UnaryOperations {
                            operand: lhs.convert()?.into(),
                            operator: UnaryOperators::Not,
                        }
                        .into(),
                        rhs: rhs.convert()?.into(),
                        operator: BinaryOperators::Or,
                    },
                    _ => self.clone(),
                };
                Ok(expr)
            },
            ExprKind::Function { aggregate_type, expr } => match aggregate_type {
                FunctionType::Count => {
                }
                _ => self.clone(),
            },*/
            _ => Ok(self.clone()),
        }
    }
}