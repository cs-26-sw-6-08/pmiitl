use std::{error::Error, ops::{Add, Div, Mul, Neg, Not, Sub}};

use crate::{errors, program::operations::{BinaryOperators, UnaryOperators}};

#[derive(Debug, PartialEq, Clone)]
pub enum Verdict { True, False, Undecided }

impl Verdict {
    pub fn to_bool(&self) -> bool{
        match self {
            Verdict::True => true,
            Verdict::False | 
            Verdict::Undecided => false,
        }
    }
    
    pub fn and(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Verdict::True, Verdict::True) => Verdict::True,
            (_, Verdict::False) => Verdict::False,
            (Verdict::False, _) => Verdict::False,
            (Verdict::Undecided, _) => Verdict::Undecided,
            (_, Verdict::Undecided) => Verdict::Undecided,
        }
    } 

    pub fn or(self, rhs: Self) -> Self {
         match (self, rhs) {
            (Verdict::False, Verdict::False) => Verdict::False,
            (_, Verdict::True) => Verdict::True,
            (Verdict::True, _) => Verdict::True,
            (Verdict::Undecided, _) => Verdict::Undecided,
            (_, Verdict::Undecided) => Verdict::Undecided,
        }
    }
}

impl Not for Verdict {
    type Output = Verdict;

    fn not(self) -> Self::Output {
        use Verdict::*;
        match self {
            True => False,
            False => True,
            Undecided => Undecided,
        }
    }
}

impl From<bool> for Verdict {
    fn from(value: bool) -> Self {
        match value {
            true => Verdict::True,
            false => Verdict::False,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Device {
    name: String,
    power: i128,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StreamOutput<'a> {
    pub(crate) value: StackContent<'a>, 
    pub(crate) decided: Decidedability 
}


impl<'a> StreamOutput<'a> {
    pub fn get_value(&self) -> &StackContent<'a> {
        &self.value
    }

    pub fn get_taint(&self) -> &Decidedability {
        &self.decided
    }

    pub fn is_undecided(&self) -> bool {
        self.decided == Decidedability::Undecided
    }
    
    pub fn is_decided(&self) -> bool {
        self.decided == Decidedability::Decided
    }

    pub fn to_undecided(mut self) -> Self {
        self.decided = Decidedability::Undecided;
        self
    }


    pub fn modulo(mut self, rhs: Self) -> Self {
        self.value = match (self.get_value().to_num(), rhs.get_value().to_num()){
            (StackContent::Number(val1), StackContent::Number(val2)) => 
                // Rust does not have a correct mathmatical mod function, therefore this calculation is used instead:
                // https://stackoverflow.com/a/31210691
                StackContent::Number(((val1.checked_rem(val2).unwrap_or(0)) + val2).checked_rem(val2).unwrap_or(0)),
            _ => unreachable!()
        };

        self.decided = self.decided.greatest_lower_bound(&rhs.decided);
        self
    }

    pub fn equals(mut self, rhs: Self) -> Self {
        let value = match (self.get_value().to_num(),rhs.get_value().to_num()){
            (StackContent::Number(val1), StackContent::Number(val2)) => StackContent::Verdict(val1 == val2),
            _ => unreachable!()
        };
        
        self.value = value;
        self.decided = self.decided.greatest_lower_bound(&rhs.decided);
        self 
    }

    pub fn not_equals(mut self, rhs: Self) -> Self {
        let value = match (self.get_value().to_num(),rhs.get_value().to_num()){
            (StackContent::Number(val1), StackContent::Number(val2)) => StackContent::Verdict(val1 != val2),
            _ => unreachable!(),
        };
        
        self.value = value;
        self.decided = self.decided.greatest_lower_bound(&rhs.decided);
        self 
    }

    pub fn and(mut self, rhs: Self) -> Self {
        self.value = match (self.get_value().to_ver(), rhs.get_value().to_ver()) {
            (StackContent::Verdict(v1), StackContent::Verdict(v2)) => 
                StackContent::Verdict(v1 && v2),
             _ => unreachable!()
                     };
        self.decided = self.decided.greatest_lower_bound(&rhs.decided);
        self
    }

    pub fn or(mut self, rhs: Self) -> Self {
        self.value = match (self.get_value().to_ver(), rhs.get_value().to_ver()) {
            (StackContent::Verdict(v1), StackContent::Verdict(v2)) => 
                StackContent::Verdict(v1 || v2),
            _ => unreachable!()
        };
        self.decided = self.decided.greatest_lower_bound(&rhs.decided);
        self
    }

    pub fn less_than(mut self, rhs: Self) -> Self {
        self.value = match (self.get_value().to_num(), rhs.get_value().to_num()) {
            (StackContent::Number(v1), StackContent::Number(v2)) => StackContent::Verdict(v1 < v2),
            _ => unreachable!()
        };
        self.decided = self.decided.greatest_lower_bound(&rhs.decided);
        self
    }

    pub fn less_equal(mut self, rhs: Self) -> Self {
         self.value = match (self.get_value().to_num(), rhs.get_value().to_num()) {
            (StackContent::Number(v1), StackContent::Number(v2)) => 
                StackContent::Verdict(v1 <= v2),
                _ => unreachable!()
        };
        self.decided = self.decided.greatest_lower_bound(&rhs.decided);
        self
    }

    pub fn un_op(self, un_op: &UnaryOperators) -> Self {
        match un_op {
            UnaryOperators::Not => !self,
            UnaryOperators::Negative => -self,
        }
    }

    pub fn mul_op(mut self, other: StreamOutput) -> Self {
        let m = match self.get_value().to_num() {
            StackContent::Number(m) => m,
            _ => unreachable!()
        };

        let n = match other.get_value().to_num() {
            StackContent::Number(m) => m,
            _ => unreachable!()
        };

        let m_int = m / 1000;
        let m_frac = m % 1000;

        let int = n*m_int;
        let frac = (n*m_frac) / 1000;
        self.value = StackContent::Number(int + frac);

        self.decided = self.decided.greatest_lower_bound(&other.decided);
        self
    }

    pub fn div_op(mut self, other: StreamOutput) -> Self {    
        let m = match self.get_value().to_num() {
            StackContent::Number(m) => m,
            _ => unreachable!()
        };

        let n = match other.get_value().to_num() {
            StackContent::Number(m) => m,
            _ => unreachable!()
        };

        let m_int = m / 1000;
        let m_frac = m % 1000;

        let int = n.checked_div(m_int).unwrap_or(0);
        let frac = n.checked_div(m_frac).unwrap_or(0);
        self.value = StackContent::Number(int + frac);

        self.decided = self.decided.greatest_lower_bound(&other.decided);
        self
    }

    pub fn bin_op(self, rhs: Self, bin_op: &BinaryOperators) -> Self {
        match bin_op {
            BinaryOperators::Equal => self.equals(rhs),
            BinaryOperators::NotEqual => self.not_equals(rhs),
            BinaryOperators::Less => self.less_than(rhs),
            BinaryOperators::Greater => rhs.less_than(self),
            BinaryOperators::LessEqual => self.less_equal(rhs),
            BinaryOperators::GreaterEqual => rhs.less_equal(self),
            BinaryOperators::Plus => self + rhs,
            BinaryOperators::Minus => self - rhs,
            BinaryOperators::Times => self.mul_op(rhs),
            BinaryOperators::Divide => rhs.div_op(self),
            BinaryOperators::Mod => self.modulo(rhs),
            BinaryOperators::Or => self.or(rhs),
            _ => unreachable!()
        }
    }

    
}

#[derive(Debug, PartialEq, Clone)]
pub enum StackContent<'a> {
    Verdict(bool),
    Number(i128),
    String(&'a String)
}

impl StackContent<'_> {
    pub fn get_verdict(&self) -> Result<bool, Box<dyn Error>> {
        match self {
            StackContent::Verdict(verdict) => Ok(*verdict),
            StackContent::Number(v) => Ok(*v != 0),
            _ => Err(errors::Error::ValueStackVal.into())
        }
    }

    pub fn to_ver(&self) -> Self {
        match self {
            StackContent::Verdict(_) => self.clone(),
            StackContent::Number(v) => Self::Verdict(*v != 0),
            StackContent::String(_) => panic!("Got a String, Expected Number or Boolean"),
        }
    }

    pub fn to_num(&self) -> Self {
        match self {
            StackContent::Verdict(v1) => Self::Number(if *v1 { 1_000 } else { 0 }),
            StackContent::Number(_) => self.clone(),
            StackContent::String(_) => panic!("Got a String, Expected Number or Boolean"),
        }
    }

    pub fn get_num(&self) -> Result<i128, Box<dyn Error>> {
        match self {
            StackContent::Number(v) => Ok(*v),
            StackContent::Verdict(b) => Ok(if *b { 1000 } else { 0 }),
           _ => Err(errors::Error::ValueStackVal.into())
        }
    }
}



#[derive(Debug, PartialEq, Clone)]
pub enum Decidedability { Decided, Undecided }

impl Decidedability {
    fn greatest_lower_bound(&self, rhs: &Self) -> Self {
        match (self, rhs) {
            (Decidedability::Decided, Decidedability::Decided) => Decidedability::Decided,
            _ => Decidedability::Undecided,
        }
    }
}



impl From<i128> for StreamOutput<'_> {
    fn from(value: i128) -> Self {
        Self { value: StackContent::Number(value), decided: Decidedability::Decided }
    }
}

impl<'a> From<&'a String> for StreamOutput<'a> {
    fn from(value: &'a String) -> Self {
        Self { value: StackContent::String(value), decided: Decidedability::Decided }
    }
}

impl From<bool> for StreamOutput<'_> {
    fn from(value: bool) -> Self {
        Self { value: StackContent::Verdict(value), decided: Decidedability::Decided }
    }
}

impl<'a> Not for StreamOutput<'a> {
    type Output = StreamOutput<'a>;
    fn not(mut self) -> Self::Output {
        self.value = match self.value.to_ver() {
            StackContent::Verdict(verdict) => StackContent::Verdict(!verdict),
            _ => unreachable!()
        };
        self
    }
}

impl<'a> Neg for StreamOutput<'a> {
    type Output = StreamOutput<'a>;
 
    fn neg(mut self) -> Self::Output {
        self.value = match self.value.to_num() {
            StackContent::Number(num) => StackContent::Number(-num),
            _ => unreachable!()
        };
        self
    }
}



impl<'a> Add for StreamOutput<'a> {
    type Output = StreamOutput<'a>;

    fn add(mut self, rhs: Self) -> Self::Output {
        let value = match (self.get_value(),rhs.get_value()){
            (StackContent::Number(val1), StackContent::Number(val2)) => StackContent::Number(val1 + val2),
            (StackContent::Verdict(v1), StackContent::Verdict(v2)) => StackContent::Number(*v1 as i128 * 1_000 + *v2 as i128 * 1_000),
            (StackContent::Verdict(v1), StackContent::Number(v2)) | (StackContent::Number(v2), StackContent::Verdict(v1)) 
            => StackContent::Number(*v1 as i128 * 1_000 + *v2),
            _ => unreachable!()
        };
        
        self.value = value;
        self.decided = self.decided.greatest_lower_bound(&rhs.decided);
        self
    }
}

impl<'a> Sub for StreamOutput<'a> {
    type Output = StreamOutput<'a>;

    fn sub(mut self, rhs: Self) -> Self::Output {
        let value = match (self.get_value(),rhs.get_value()){
            (StackContent::Number(val1), StackContent::Number(val2)) => StackContent::Number(val1 - val2),
            (StackContent::Verdict(v1), StackContent::Verdict(v2)) => StackContent::Number(*v1 as i128 * 1_000 - *v2 as i128 * 1_000),
            (StackContent::Verdict(v1), StackContent::Number(v2)) => StackContent::Number(*v1 as i128 * 1_000 - *v2),
            (StackContent::Number(v1), StackContent::Verdict(v2)) => StackContent::Number(*v1 - *v2 as i128 * 1_000),
            _ => unreachable!()
        };
        
        self.value = value;
        self.decided = self.decided.greatest_lower_bound(&rhs.decided);
        self
    }
}

impl<'a> Div for StreamOutput<'a> {
    type Output = StreamOutput<'a>;

    fn div(mut self, rhs: Self) -> Self::Output {
        let value = match (self.get_value(),rhs.get_value()){
            (StackContent::Number(val1), StackContent::Number(val2)) => StackContent::Number(val1 / val2),
            _ => unreachable!()
        };
        
        self.value = value;
        self.decided = self.decided.greatest_lower_bound(&rhs.decided);
        self
    }
}