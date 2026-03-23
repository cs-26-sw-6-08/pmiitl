use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperators {
    Equal,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    NotEqual,
    Plus,
    Minus,
    Times,
    Divide,
    Mod,
    And,
    Or,
    Implies
}

impl BinaryOperators {
    pub fn new(operator: &str) -> Result<Self, Box<dyn Error>> {
        Ok(match operator {
            "=" => Self::Equal,
            "<" => Self::Less,
            ">" => Self::Greater,
            "<=" => Self::LessEqual,
            ">=" => Self::GreaterEqual,
            "!=" => Self::NotEqual,
            "+" => Self::Plus,
            "-" => Self::Minus,
            "*" => Self::Times,
            "/" => Self::Divide,
            "%" => Self::Mod,
            "&" => Self::And,
            "|" => Self::Or,
            "->" => Self::Implies,
            _ => unreachable!()
        })
    }
}

impl Display for BinaryOperators {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOperators::Equal => write!(f, "="),
            BinaryOperators::Less => write!(f, "<"),
            BinaryOperators::Greater => write!(f, ">"),
            BinaryOperators::LessEqual => write!(f, "<="),
            BinaryOperators::GreaterEqual => write!(f, ">="),
            BinaryOperators::NotEqual => write!(f, "!="),
            BinaryOperators::Plus => write!(f, "+"),
            BinaryOperators::Minus => write!(f, "-"),
            BinaryOperators::Times => write!(f, "*"),
            BinaryOperators::Divide => write!(f, "/"),
            BinaryOperators::Mod => write!(f, "%"),
            BinaryOperators::And => write!(f, "&"),
            BinaryOperators::Or => write!(f, "|"),
            BinaryOperators::Implies => write!(f, "->"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperators {
    Not,
    Negative
}

impl UnaryOperators {
    pub fn new(operator: &str) -> Result<Self, Box<dyn Error>> {
        Ok(match operator {
            "!" => Self::Not,
            "-" => Self::Negative,
            _ => unreachable!()
        })
    }
}

impl Display for UnaryOperators {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOperators::Not => write!(f, "!"),
            UnaryOperators::Negative => write!(f, "-"),
        }
    }
}