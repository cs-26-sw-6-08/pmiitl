use std::error::Error;

#[derive(Debug)]
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
#[derive(Debug)]
pub enum UnaryOperators {}