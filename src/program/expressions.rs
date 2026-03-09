use std::error::Error;

use hime_redist::{ast::AstNode, symbols::SemanticElementTrait};

use crate::{errors, program::{operations::{BinaryOperators, UnaryOperators}, units::Unit}};

pub type Expr = SpannedExpr;

#[derive(Debug)]
pub struct SpannedExpr {
    expr: ExprKind,
    line: usize,
    column: usize,
}

#[derive(Debug)]
pub enum ExprKind {
    Number(i64),
    String(String),
    Boolean(bool),
    Timeunit{
        number: Box<Expr>,
        unit : Unit
    },
    Interval {
        start: Box<Expr>,
        end: Box<Expr>,
    },
    Always {
        interval: Option<Box<Expr>>,
        not: bool,
        expr: Box<Expr>,
    },
    Eventually {
        interval: Option<Box<Expr>>,
        not: bool,
        expr: Box<Expr>,
    },
    Until {
        interval: Option<Box<Expr>>,
        not: bool,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    BinaryOperations {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
        operator: BinaryOperators 
    },
    UnaryOperations {
        operand: Box<Expr>,
        operator: UnaryOperators 
    }
}

impl Expr {
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
            "TIMEUNIT" => {
                        let number = Expr::new(node.child(0))?.into();
                        let unit = Unit::new(node.get_value()
                    .ok_or_else(|| {
                        errors::Error::ASTNodeValueInvalid(node.get_symbol().name.into())
                    })?)?;
                        ExprKind::Timeunit { number, unit }
                    }

            "Interval" => {
                let start = Expr::new(node.child(0))?.into();
                let end = Expr::new(node.child(1))?.into();
                ExprKind::Interval { start, end }
            }
            "always" => {
                let interval = match node
                    .children()
                    .iter()
                    .find(|node| node.get_symbol().name.eq("Interval"))
                {
                    Some(interval) => Some(Expr::new(interval)?.into()),
                    None => None,
                };
                let not = match node
                    .children()
                    .iter()
                    .find(|node| node.get_symbol().name.eq("not"))
                {
                    Some(_) => true,
                    None => false,
                };
                let expr = Expr::new(node.children().iter().last().unwrap())?.into();

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
                    Some(interval) => Some(Expr::new(interval)?.into()),
                    None => None,
                };
                let not = match node
                    .children()
                    .iter()
                    .find(|node| node.get_symbol().name.eq("not"))
                {
                    Some(_) => true,
                    None => false,
                };
                let expr = Expr::new(node.children().iter().last().unwrap())?.into();

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
                    Some(interval) => Some(Expr::new(interval)?.into()),
                    None => None,
                };
                let not = match node
                    .children()
                    .iter()
                    .find(|node| node.get_symbol().name.eq("not"))
                {
                    Some(_) => true,
                    None => false,
                };
                let mut iter = node
                    .children()
                    .iter()
                    .rev()
                    .take(2)
                    .map(|node| Box::new(Expr::new(node).unwrap()));

                let (rhs, lhs) = (iter.next().unwrap().into(), iter.next().unwrap().into());

                ExprKind::Until {
                    interval,
                    not,
                    lhs,
                    rhs,
                }
            },
            "->" |  "|" | "&" | "=" | "<=" | ">=" | "!=" | "<" | ">" | "+" | "-" | "*" | "/" | "%" =>{
                if node.children_count() == 2 {
                    let lhs = Expr::new(node.child(0))?.into();
                    let rhs =  Expr::new(node.child(1))?.into();
                    let operator = BinaryOperators::new(node.get_value()
                        .ok_or_else(|| {
                            errors::Error::ASTNodeValueInvalid(node.get_symbol().name.into())
                        })?)?;
                    ExprKind::BinaryOperations { lhs, rhs, operator }
                } else {
                    todo!()
                }
                
            }

            _ => {
                let position = node.get_position().unwrap();
                return Err(errors::Error::ProgramParseError(node.get_symbol().name.into(), position.line, position.column).into());
            },
        };

        Ok(SpannedExpr { expr, line: node.get_position().unwrap().line, column: node.get_position().unwrap().column })
    }
}
