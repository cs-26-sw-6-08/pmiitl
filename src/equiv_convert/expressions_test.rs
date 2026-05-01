use crate::program::function_types::FunctionType;
use crate::program::operations::{BinaryOperators, UnaryOperators};
use crate::program::units::Unit;
use crate::utils::test_helper_func::{always_expr, always_negated_expr, always_interval_expr, binary_expr, custom_number_expr, custom_unit_expr, eventually_expr, eventually_negated_expr, function_expr, interval_expr, number_expr, unary_expr, unit_expr};

#[test]
fn count(){
    let mut expr = function_expr(FunctionType::Count, number_expr(), None);
    assert!(expr.equiv_convert().is_ok());
    assert_eq!(expr, function_expr(FunctionType::Sum, custom_number_expr(1000), None));
}

#[test]
fn counttime(){
    let bound = Some(custom_number_expr(10_000));
    let mut expr = function_expr(FunctionType::Counttime, number_expr(), bound.clone());
    assert!(expr.equiv_convert().is_ok());
    assert_eq!(expr, function_expr(FunctionType::Sumtime, custom_number_expr(1000), bound));
}

#[test]
fn always(){
    let mut expr = always_expr(binary_expr(number_expr(), number_expr(), BinaryOperators::Plus));
    assert!(expr.equiv_convert().is_ok());
    assert_eq!(expr, always_expr(custom_number_expr(10_000)));
}

#[test]
fn always_interval(){
    let mut expr = always_interval_expr(interval_expr(unit_expr(Unit::Seconds), unit_expr(Unit::Seconds)), binary_expr(number_expr(), number_expr(), BinaryOperators::Plus));
    assert!(expr.equiv_convert().is_ok());
    assert_eq!(expr, always_interval_expr(interval_expr(number_expr(), number_expr()), custom_number_expr(10_000)));
}

#[test]
fn always_negated(){
    let mut expr = always_negated_expr(unary_expr(number_expr(), UnaryOperators::Not));
    assert!(expr.equiv_convert().is_ok());
    assert_eq!(expr, eventually_expr(number_expr()));
}

#[test]
fn eventually(){
    let mut expr = eventually_expr(binary_expr(number_expr(), number_expr(), BinaryOperators::Plus));
    assert!(expr.equiv_convert().is_ok());
    assert_eq!(expr, eventually_expr(custom_number_expr(10_000)));
}

#[test]
fn eventually_negated(){
    let mut expr = eventually_negated_expr(unary_expr(number_expr(), UnaryOperators::Not));
    assert!(expr.equiv_convert().is_ok());
    assert_eq!(expr, always_expr(custom_number_expr(1000)));
}

#[test]
fn interval(){
    let mut expr = interval_expr(custom_unit_expr(5_000, Unit::Seconds), custom_unit_expr(10_000, Unit::Seconds));
    assert!(expr.equiv_convert().is_ok());
    assert_eq!(expr, interval_expr(number_expr(), custom_number_expr(10_000)));
}

#[test]
fn interval_below_zero(){
    let mut expr = interval_expr(binary_expr(custom_number_expr(5_000), custom_number_expr(10_000), BinaryOperators::Minus), number_expr());
    assert!(expr.equiv_convert().is_err());
    
}

#[test]
fn interval_start_greater_than_end(){
    let mut expr = interval_expr(custom_number_expr(10_000), number_expr());
    assert!(expr.equiv_convert().is_err());
    
}

#[test]
fn unit(){
    let mut expr = unit_expr(Unit::Seconds);
    assert!(expr.equiv_convert().is_ok());
    assert_eq!(expr, number_expr());
}

#[test]
fn unary() {
    let mut expr = unary_expr(number_expr(), UnaryOperators::Negative );
    assert!(expr.equiv_convert().is_ok());
    assert_eq!(expr, custom_number_expr(-5000));
}
