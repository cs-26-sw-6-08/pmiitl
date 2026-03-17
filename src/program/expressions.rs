use std::error::Error;

use hime_redist::{ast::AstNode, symbols::SemanticElementTrait};

use crate::{
    errors,
    program::{
        function_types::FunctionType, member_types::MemberType, operations::{BinaryOperators, UnaryOperators}, units::Unit
    },
};

#[derive(Debug, PartialEq)]
pub struct SpannedExpr {
    pub expr: Expr,
    pub line: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Number(i128),
    String(String),
    Boolean(bool),
    CurrentTime,
    Unit {
        number: Box<Expr>,
        unit: Unit,
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
        operator: BinaryOperators,
    },
    UnaryOperations {
        operand: Box<Expr>,
        operator: UnaryOperators,
    },
    Member {
        access_type: MemberType,
    },
    Function {
        aggregate_type: FunctionType,
        expr: Box<Expr>,
    },
}

impl SpannedExpr {
    pub fn new(node: AstNode) -> Result<Self, Box<dyn Error>> {
        let position = node
            .get_position()
            .unwrap_or(hime_redist::text::TextPosition { line: 0, column: 0 });
        let expr = Expr::new(node)?;

        Ok(SpannedExpr {
            expr,
            line: position.line,
        })
    }
}

impl Expr {
    pub fn new(node: AstNode) -> Result<Self, Box<dyn Error>> {
        let expr = match node.get_symbol().name {
            "NUMBER" => Expr::Number(
                (node
                    .get_value()
                    .ok_or_else(|| {
                        errors::Error::ASTNodeValueInvalid(node.get_symbol().name.into())
                    })?
                    .parse::<f64>()?
                    * 1000.0).round() as i128, // Numbers are stored as integers, decimals is represented as last 3 digits.
            ),
            "BOOL" => Expr::Boolean(
                node.get_value()
                    .ok_or_else(|| {
                        errors::Error::ASTNodeValueInvalid(node.get_symbol().name.into())
                    })?
                    .parse()?,
            ),
            "STRING" => Expr::String(
                node.get_value()
                    .ok_or_else(|| {
                        errors::Error::ASTNodeValueInvalid(node.get_symbol().name.into())
                    })?
                    .into(),
            ),
            "TIME" => Expr::CurrentTime,
            "TIMEUNIT" | "POWERUNIT" => {
                let number = Expr::new(node.child(0))?.into();
                let unit = Unit::new(node.get_value().ok_or_else(|| {
                    errors::Error::ASTNodeValueInvalid(node.get_symbol().name.into())
                })?)?;
                Expr::Unit { number, unit }
            }

            "Interval" => {
                let start = Expr::new(node.child(0))?.into();
                let end = Expr::new(node.child(1))?.into();
                Expr::Interval { start, end }
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
                let not = node
                    .children()
                    .iter()
                    .any(|node| node.get_symbol().name.eq("!") && node.children_count()==0);
                let expr = Expr::new(node.children().iter().next_back().unwrap())?.into();

                Expr::Always {
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
                let not = node
                .children()
                .iter()
                .any(|node| node.get_symbol().name.eq("!") && node.children_count()==0);
                
    
                let expr = Expr::new(node.children().iter().next_back().unwrap())?.into();

                Expr::Eventually {
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
                let not = node
                    .children()
                    .iter()
                    .any(|node| node.get_symbol().name.eq("!") && node.children_count()==0);

                let mut iter = node
                    .children()
                    .iter()
                    .rev()
                    .take(2)
                    .map(|node| Box::new(Expr::new(node).unwrap()));

                let (rhs, lhs) = (iter.next().unwrap(), iter.next().unwrap());

                Expr::Until {
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
                    let lhs = Expr::new(node.child(0))?.into();
                    let rhs = Expr::new(node.child(1))?.into();
                    let operator = BinaryOperators::new(node.get_value().ok_or_else(|| {
                        errors::Error::ASTNodeValueInvalid(node.get_symbol().name.into())
                    })?)?;
                    Expr::BinaryOperations { lhs, rhs, operator }
                } else {
                    let operand = Expr::new(node.child(0))?.into();
                    let operator = UnaryOperators::new(node.get_value().ok_or_else(|| {
                        errors::Error::ASTNodeValueInvalid(node.get_symbol().name.into())
                    })?)?;
                    Expr::UnaryOperations { operand, operator }
                }
            }
            "active" | "power" | "name" => {
                let access_type = MemberType::new(node.get_value().ok_or_else(|| {
                    errors::Error::ASTNodeValueInvalid(node.get_symbol().name.into())
                })?)?;
                Expr::Member { access_type }
            }
            "sum" | "avg" | "count" | "sumtime" | "avgtime" | "counttime" | "foreach" => {
                let aggregate_type = FunctionType::new(node.get_value().ok_or_else(|| {
                    errors::Error::ASTNodeValueInvalid(node.get_symbol().name.into())
                })?)?;
                let expr = Expr::new(node.child(0))?.into();
                Expr::Function {
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

    
}
