use std::{error::Error, ops::{Add, Div, Mul, Not, Sub}};

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
pub struct StackValue<'a> {
    pub(crate) value: StackContent<'a>, 
    pub(crate) decided: Decidedability 
}


impl<'a> StackValue<'a> {
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
        let value = match (self.get_value(),rhs.get_value()){
            (StackContent::Number(val1), StackContent::Number(val2)) => {
                //println!("{} % {} = {:?}",val1, val2, DerivedOutput::Number(val1 % val2));
                StackContent::Number((val1 % val2)*1000)},
            _ => unreachable!()
        };
        self.value = value;
        self.decided = self.decided.greatest_lower_bound(&rhs.decided);
        self
    }

    pub fn equals(mut self, rhs: Self) -> Self {
        let value = match (self.get_value(),rhs.get_value()){
            (StackContent::Number(val1), StackContent::Number(val2)) => StackContent::Verdict((val1 == val2).into()),
            (StackContent::Verdict(val1), StackContent::Verdict(val2)) => StackContent::Verdict((val1 == val2).into()),
            (StackContent::String(val1), StackContent::String(val2)) => StackContent::Verdict((val1 == val2).into()),
            _ => unreachable!("Died in equals :)")
        };
        
        self.value = value;
        self.decided = self.decided.greatest_lower_bound(&rhs.decided);
        self 
    }

    pub fn not_equals(mut self, rhs: Self) -> Self {
        let value = match (self.get_value(),rhs.get_value()){
            (StackContent::Number(val1), StackContent::Number(val2)) => {
                //println!("{val1}, {val2}"); 
                StackContent::Verdict((val1 != val2).into())},
            (StackContent::Verdict(val1), StackContent::Verdict(val2)) => StackContent::Verdict((val1 != val2).into()),
            (StackContent::String(val1), StackContent::String(val2)) => StackContent::Verdict((val1 != val2).into()),
            (StackContent::Verdict(v1), StackContent::Number(v2)) | 
            (StackContent::Number(v2), StackContent::Verdict(v1))
            //todo: Check logic here
            => StackContent::Number((if *v1 != (*v2 != 0) { 1000 } else { 0 }).into()),
            _ => unreachable!("LOOOK HERE IS THE ERROR SEARCH AFTER THIS NUMBER 128937178219378"),
        };
        
        self.value = value;
        self.decided = self.decided.greatest_lower_bound(&rhs.decided);
        self 
    }

    pub fn and(mut self, rhs: Self) -> Self {
        self.value = match (self.get_value(), rhs.get_value()) {
            (StackContent::Verdict(v1), StackContent::Verdict(v2)) => 
                StackContent::Verdict(*v1 && *v2),
            (StackContent::Number(v1), StackContent::Verdict(v2)) |
            (StackContent::Verdict(v2), StackContent::Number(v1))
             => StackContent::Verdict(*v1 != 0 && *v2),
            _ => unreachable!()
        };
        self.decided = self.decided.greatest_lower_bound(&rhs.decided);
        self
    }

    pub fn or(mut self, rhs: Self) -> Self {
        self.value = match (self.get_value(), rhs.get_value()) {
            (StackContent::Verdict(v1), StackContent::Verdict(v2)) => 
                StackContent::Verdict(*v1 || *v2),
            (StackContent::Number(v1), StackContent::Verdict(v2)) | 
            (StackContent::Verdict(v2), StackContent::Number(v1))
            => StackContent::Verdict((*v1 != 0) || *v2),
            _ => unreachable!()
        };
        self.decided = self.decided.greatest_lower_bound(&rhs.decided);
        self
    }

    pub fn less_than(mut self, rhs: Self) -> Self {
        self.value = match (self.get_value(), rhs.get_value()) {
            (StackContent::Number(v1), StackContent::Number(v2)) => 
                StackContent::Verdict((v1 < v2).into()),
            _ => unreachable!()
        };
        self.decided = self.decided.greatest_lower_bound(&rhs.decided);
        self
    }

    pub fn less_equal(mut self, rhs: Self) -> Self {
         self.value = match (self.get_value(), rhs.get_value()) {
            (StackContent::Number(v1), StackContent::Number(v2)) => 
                StackContent::Verdict((v1 <= v2).into()),
            _ => unreachable!()
        };
        self.decided = self.decided.greatest_lower_bound(&rhs.decided);
        self
    }

    


    pub fn un_op(mut self, un_op: &UnaryOperators) -> Self {
        self.value = match (self.value, un_op) {
            (StackContent::Verdict(verdict), UnaryOperators::Not) => StackContent::Verdict(verdict.not()),
            (StackContent::Number(v), UnaryOperators::Negative) => StackContent::Number(-v),
            _ => unreachable!()
        };
        self
    }

    pub fn mul_op(mut self, other: StackValue) -> Self {
        let StackContent::Number(lhs) = self.get_value() else {todo!()};
        let StackContent::Number(rhs) = other.get_value() else {todo!()};

        todo!()
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
            BinaryOperators::Times => self * rhs,
            BinaryOperators::Divide => self / rhs,
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
    pub fn get_verdict(&self) -> Option<bool> {
        Some(match self {
            StackContent::Verdict(verdict) => *verdict,
            StackContent::Number(v) => *v != 0,
            _ => unreachable!("Fail in return")
        })
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


impl From<i128> for StackValue<'_> {
    fn from(value: i128) -> Self {
        Self { value: StackContent::Number(value), decided: Decidedability::Decided }
    }
}

impl<'a> From<&'a String> for StackValue<'a> {
    fn from(value: &'a String) -> Self {
        Self { value: StackContent::String(value), decided: Decidedability::Decided }
    }
}

impl From<bool> for StackValue<'_> {
    fn from(value: bool) -> Self {
        Self { value: StackContent::Verdict(value), decided: Decidedability::Decided }
    }
}

//todo: Change these
impl<'a> Not for StackValue<'a> {
    type Output = StackValue<'a>;
    fn not(mut self) -> Self::Output {
        self.value = match self.value {
            StackContent::Verdict(verdict) => StackContent::Verdict(!verdict),
            _ => unreachable!()
        };
        self
    }
}


impl<'a> Mul for StackValue<'a> {
    type Output = StackValue<'a>;

    fn mul(mut self, rhs: Self) -> Self::Output {
        let value = match (self.get_value(),rhs.get_value()){
            (StackContent::Number(val1), StackContent::Number(val2)) => StackContent::Number(val1 * val2),
            _ => unreachable!()
        };
        
        self.value = value;
        self.decided = self.decided.greatest_lower_bound(&rhs.decided);
        self
    }
}

impl<'a> Add for StackValue<'a> {
    type Output = StackValue<'a>;

    fn add(mut self, rhs: Self) -> Self::Output {
        let value = match (self.get_value(),rhs.get_value()){
            (StackContent::Number(val1), StackContent::Number(val2)) => StackContent::Number(val1 + val2),
            _ => unreachable!()
        };
        
        self.value = value;
        self.decided = self.decided.greatest_lower_bound(&rhs.decided);
        self
    }
}

impl<'a> Sub for StackValue<'a> {
    type Output = StackValue<'a>;

    fn sub(mut self, rhs: Self) -> Self::Output {
        let value = match (self.get_value(),rhs.get_value()){
            (StackContent::Number(val1), StackContent::Number(val2)) => StackContent::Number(val1 - val2),
            _ => unreachable!()
        };
        
        self.value = value;
        self.decided = self.decided.greatest_lower_bound(&rhs.decided);
        self
    }
}

impl<'a> Div for StackValue<'a> {
    type Output = StackValue<'a>;

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

