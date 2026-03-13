use std::error::Error;

use hime_redist::{ast::AstNode, symbols::SemanticElementTrait};

use crate::{
    errors,
    program::{
        conversion_binary_operator::conversion_binary_operations, conversion_units, function_types::FunctionType, member_types::MemberType, operations::{BinaryOperators, UnaryOperators}, units::Unit
    },
};

#[derive(Debug, PartialEq)]
pub struct SpannedExpr {
    pub expr: ExprKind,
    pub line: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExprKind {
    Number(i64),
    String(String),
    Boolean(bool),
    CurrentTime,
    Unit {
        number: Box<ExprKind>,
        unit: Unit,
    },
    Interval {
        start: Box<ExprKind>,
        end: Box<ExprKind>,
    },
    Always {
        interval: Option<Box<ExprKind>>,
        not: bool,
        expr: Box<ExprKind>,
    },
    Eventually {
        interval: Option<Box<ExprKind>>,
        not: bool,
        expr: Box<ExprKind>,
    },
    Until {
        interval: Option<Box<ExprKind>>,
        not: bool,
        lhs: Box<ExprKind>,
        rhs: Box<ExprKind>,
    },
    BinaryOperations {
        lhs: Box<ExprKind>,
        rhs: Box<ExprKind>,
        operator: BinaryOperators,
    },
    UnaryOperations {
        operand: Box<ExprKind>,
        operator: UnaryOperators,
    },
    Member {
        access_type: MemberType,
    },
    Function {
        aggregate_type: FunctionType,
        expr: Box<ExprKind>,
    },
}

impl SpannedExpr {
    pub fn new(node: AstNode) -> Result<Self, Box<dyn Error>> {
        let position = node
            .get_position()
            .unwrap_or(hime_redist::text::TextPosition { line: 0, column: 0 });
        let expr = ExprKind::new(node)?;

        Ok(SpannedExpr {
            expr,
            line: position.line,
        })
    }
}

impl ExprKind {
    pub fn new(node: AstNode) -> Result<Self, Box<dyn Error>> {
        let expr = match node.get_symbol().name {
            "NUMBER" => ExprKind::Number(
                (node
                    .get_value()
                    .ok_or_else(|| {
                        errors::Error::ASTNodeValueInvalid(node.get_symbol().name.into())
                    })?
                    .parse::<f64>()?
                    * 1000f64) as i64, // Numbers are stored as integers, decimals is represented as last 3 digits.
            ),
            "BOOL" => ExprKind::Boolean(
                node.get_value()
                    .ok_or_else(|| {
                        errors::Error::ASTNodeValueInvalid(node.get_symbol().name.into())
                    })?
                    .parse()?,
            ),
            "STRING" => ExprKind::String(
                node.get_value()
                    .ok_or_else(|| {
                        errors::Error::ASTNodeValueInvalid(node.get_symbol().name.into())
                    })?
                    .into(),
            ),
            "TIME" => ExprKind::CurrentTime,
            "TIMEUNIT" | "POWERUNIT" => {
                let number = ExprKind::new(node.child(0))?.into();
                let unit = Unit::new(node.get_value().ok_or_else(|| {
                    errors::Error::ASTNodeValueInvalid(node.get_symbol().name.into())
                })?)?;
                ExprKind::Unit { number, unit }
            }

            "Interval" => {
                let start = ExprKind::new(node.child(0))?.into();
                let end = ExprKind::new(node.child(1))?.into();
                ExprKind::Interval { start, end }
            }
            "always" => {
                let interval = match node
                    .children()
                    .iter()
                    .find(|node| node.get_symbol().name.eq("Interval"))
                {
                    Some(interval) => Some(ExprKind::new(interval)?.into()),
                    None => None,
                };
                let not = node
                    .children()
                    .iter()
                    .any(|node| node.get_symbol().name.eq("not"));
                let expr = ExprKind::new(node.children().iter().next_back().unwrap())?.into();

                ExprKind::Always {
                    interval,
                    not,
                    expr,
                }
            }
            "eventually" => {
                let interval = match node
                    .children()
                    .iter()
                    .find(|node| node.get_symbol().name.eq("Interval"))
                {
                    Some(interval) => Some(ExprKind::new(interval)?.into()),
                    None => None,
                };
                let not = node
                    .children()
                    .iter()
                    .any(|node| node.get_symbol().name.eq("not"));
                let expr = ExprKind::new(node.children().iter().next_back().unwrap())?.into();

                ExprKind::Eventually {
                    interval,
                    not,
                    expr,
                }
            }
            "until" => {
                let interval = match node
                    .children()
                    .iter()
                    .find(|node| node.get_symbol().name.eq("Interval"))
                {
                    Some(interval) => Some(ExprKind::new(interval)?.into()),
                    None => None,
                };
                let not = node
                    .children()
                    .iter()
                    .any(|node| node.get_symbol().name.eq("not"));

                let mut iter = node
                    .children()
                    .iter()
                    .rev()
                    .take(2)
                    .map(|node| Box::new(ExprKind::new(node).unwrap()));

                let (rhs, lhs) = (iter.next().unwrap(), iter.next().unwrap());

                ExprKind::Until {
                    interval,
                    not,
                    lhs,
                    rhs,
                }
            }
            "->" | "|" | "&" | "=" | "<=" | ">=" | "!=" | "<" | ">" | "+" | "-" | "*" | "/"
            | "%" | "!" => {
                //unary and binary operations are in one match due to "-" acting as both depending on number of children
                if node.children_count() == 2 {
                    let lhs = ExprKind::new(node.child(0))?.into();
                    let rhs = ExprKind::new(node.child(1))?.into();
                    let operator = BinaryOperators::new(node.get_value().ok_or_else(|| {
                        errors::Error::ASTNodeValueInvalid(node.get_symbol().name.into())
                    })?)?;
                    ExprKind::BinaryOperations { lhs, rhs, operator }
                } else {
                    let operand = ExprKind::new(node.child(0))?.into();
                    let operator = UnaryOperators::new(node.get_value().ok_or_else(|| {
                        errors::Error::ASTNodeValueInvalid(node.get_symbol().name.into())
                    })?)?;
                    ExprKind::UnaryOperations { operand, operator }
                }
            }
            "active" | "power" | "name" => {
                let access_type = MemberType::new(node.get_value().ok_or_else(|| {
                    errors::Error::ASTNodeValueInvalid(node.get_symbol().name.into())
                })?)?;
                ExprKind::Member { access_type }
            }
            "sum" | "avg" | "count" | "sumtime" | "avgtime" | "counttime" | "foreach" => {
                let aggregate_type = FunctionType::new(node.get_value().ok_or_else(|| {
                    errors::Error::ASTNodeValueInvalid(node.get_symbol().name.into())
                })?)?;
                let expr = ExprKind::new(node.child(0))?.into();
                ExprKind::Function {
                    aggregate_type,
                    expr,
                }
            }
            _ => {
                let position = node
                    .get_position()
                    .unwrap_or(hime_redist::text::TextPosition { line: 0, column: 0 });
                return Err(errors::Error::ProgramParse(
                    node.get_symbol().name.into(),
                    position.line,
                    position.column,
                )
                .into());
            }
        };

        Ok(expr)
    }

    pub fn convert(&self) -> Result<ExprKind, Box<dyn Error>> {
        match self {
            ExprKind::BinaryOperations { lhs, rhs, operator } => {
                let lhs = lhs.convert_units().convert()?;
                let rhs = rhs.convert_units().convert()?;

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

    fn convert_units(&self) -> ExprKind {
        conversion_units::conversion_unit(self)
    }

    
}
