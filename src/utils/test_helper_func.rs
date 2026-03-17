use crate::program::{expressions::Expr, function_types::FunctionType, member_types::MemberType, operations::{BinaryOperators, UnaryOperators}, units::Unit};

pub fn binary_expr(lhs: Expr, rhs: Expr, operator: BinaryOperators) -> Expr {
    Expr::BinaryOperations {
        lhs: lhs.into(),
        rhs: rhs.into(),
        operator,
    }
}

pub fn unary_expr(operand: Expr, operator: UnaryOperators) -> Expr {
    Expr::UnaryOperations { operand: operand.into(), operator }
}

pub fn number_expr() -> Expr {
    Expr::Number(5000)
}

pub fn custom_number_expr(n: i128) -> Expr {
    Expr::Number(n)
}

pub fn string_expr() -> Expr {
    Expr::String("christian".into())
}

pub fn bool_expr() -> Expr {
    Expr::Boolean(true)
}

pub fn current_time() -> Expr {
    Expr::CurrentTime
}

pub fn unit_expr(unit: Unit) -> Expr {
    Expr::Unit {
        number: number_expr().into(),
        unit,
    }
}

pub fn custom_unit_expr(number: i128, unit: Unit) -> Expr {
    Expr::Unit {
        number: custom_number_expr(number).into(),
        unit,
    }
}

pub fn member_expr(access_type: MemberType) -> Expr {
    Expr::Member { access_type }
}

pub fn function_expr(aggregate_type: FunctionType, expr: Expr) -> Expr {
    Expr::Function { aggregate_type, expr: expr.into() }
}

pub fn interval_expr(unit1: Expr, unit2: Expr) -> Expr {
    Expr::Interval { start: unit1.into(), end: unit2.into() }
}

pub fn always_expr(expr: Expr) -> Expr {
    Expr::Always { interval: None, not: false, expr: expr.into() }
}

pub fn eventually_expr(expr: Expr) -> Expr {
    Expr::Eventually { interval: Some(interval_expr(unit_expr(Unit::Seconds), unit_expr(Unit::Seconds)).into()), not: false, expr: expr.into() }
}

pub fn until_expr(expr1: Expr, expr2: Expr ) -> Expr {
    Expr::Until { interval: Some(interval_expr(unit_expr(Unit::Seconds), unit_expr(Unit::Seconds)).into()), not: false, lhs: expr1.into(), rhs: expr2.into() }
}