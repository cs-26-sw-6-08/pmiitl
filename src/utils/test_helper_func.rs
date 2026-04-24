use crate::{monitor::streams::IoTDevice, monitor_setup::operation_types::{AggregateType, ExprLTL, Operation}, program::{expressions::Expr, function_types::FunctionType, member_types::MemberType, operations::{BinaryOperators, UnaryOperators}, units::Unit}};

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
        Operation::LTLBounded { bound: (25, 40), idx: 2, not: false, ltl_type: ExprLTL::Always },
        Operation::TimeFunction { idx: 3, function_type: AggregateType::Sum, history: Vec::new(), bound: None },
        Operation::AggregateFunction { idx: 4, function_type: AggregateType::Sum },
        Operation::Member(MemberType::Power),
        Operation::LTLBounded { bound: (500, 1000), idx: 6, not: false, ltl_type: ExprLTL::Always },
        Operation::TimeFunction { idx: 7, function_type: AggregateType::Sum, history: Vec::new(), bound: None },
        Operation::AggregateFunction { idx: 8, function_type: AggregateType::Sum },
        Operation::Number(1)
    ].into()
}

pub fn mock_devices(amt: usize) -> Vec<IoTDevice> {
    [
        ("Roomba".into(), 5).into(),
        ("christian".into(), 15).into(),
        ("Fridge".into(), 10).into(),
        ("christian0".into(), 10).into(),
        ("christian1".into(), 20).into(),
        ("christian2".into(), 30).into(),
        ("christian3".into(), 40).into(),
        ("christian4".into(), 50).into(),
        ("christian5".into(), 60).into(),
        ("christian6".into(), 70).into(),
        ("christian7".into(), 80).into(),
        ("christian8".into(), 90).into(),
        ("christian9".into(), 100).into(),
    ].into_iter().take(amt).collect()
}