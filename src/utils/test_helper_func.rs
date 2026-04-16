use crate::{monitor_setup::operation_types::{AggregateType, LTL, Operation}, program::{expressions::Expr, function_types::FunctionType, member_types::MemberType, operations::{BinaryOperators, UnaryOperators}, units::Unit}};

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

pub fn always_negated_expr(expr: Expr) -> Expr {
    Expr::Always { interval: None, not: true, expr: expr.into() }
}

pub fn always_interval_expr(interval: Expr, expr: Expr) -> Expr {
    Expr::Always { interval: Some(interval.into()), not: false, expr: expr.into() }
}

pub fn eventually_expr(expr: Expr) -> Expr {
    Expr::Eventually { interval: None, not: false, expr: expr.into() }
}

pub fn eventually_negated_expr(expr: Expr) -> Expr {
    Expr::Eventually { interval: None, not: true, expr: expr.into() }
}

pub fn eventually_interval_expr(interval: Expr, expr: Expr) -> Expr {
    Expr::Eventually { interval: Some(interval.into()), not: false, expr: expr.into() }
}


///always[25s, 40s] sumtime(power) < always[500, 1000] sumtime (1)
pub fn operations_vec_with_sumtime() -> Vec<Operation> {
    [
        Operation::Binary { bin_op: BinaryOperators::Less, idx_lhs: 1, idx_rhs: 5 },
        Operation::LTLBounded { bound: (25, 40), idx: 2, not: false, ltl_type: LTL::Always },
        Operation::TimeFunction { idx: 3, function_type: AggregateType::Sum, history: Vec::new(), max_bound: None },
        Operation::AggregateFunction { idx: 4, function_type: AggregateType::Sum },
        Operation::Member(MemberType::Power),
        Operation::LTLBounded { bound: (500, 1000), idx: 6, not: false, ltl_type: LTL::Always },
        Operation::TimeFunction { idx: 7, function_type: AggregateType::Sum, history: Vec::new(), max_bound: None },
        Operation::AggregateFunction { idx: 8, function_type: AggregateType::Sum },
        Operation::Number(1)
    ].into()
}

/*
1 < 5
    always [25,40]
        sumtime(3)
            sum(4)
                member
    always[500,1000]
        sumtime(7)
            sum(8)
                1
*/
