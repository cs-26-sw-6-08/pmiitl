
use crate::{program::operations::BinaryOperators, utils::test_helper_func::{always_expr, always_interval_expr, always_negated_expr, binary_expr, custom_number_expr, custom_unit_expr, eventually_expr, eventually_interval_expr, eventually_negated_expr, function_expr, interval_expr, number_expr, unary_expr, unit_expr}};


#[test]
fn should_pass(){
    // Always = success
    assert!(always_expr(number_expr()).monitorability_check().is_ok());
    // Always Interval = success
    assert!(always_interval_expr(interval_expr(number_expr(), number_expr()), number_expr()).monitorability_check().is_ok());
    // Eventually interval = success
    assert!(eventually_interval_expr(interval_expr(number_expr(), number_expr()),number_expr()).monitorability_check().is_ok());
    // Always eventually interval = success
    assert!(always_expr(eventually_interval_expr(interval_expr(number_expr(), number_expr()), number_expr())).monitorability_check().is_ok());
    // Always interval eventually = success
    assert!(always_interval_expr(interval_expr(number_expr(), number_expr()), eventually_expr(number_expr())).monitorability_check().is_ok());
    // Eventually interval always = success
    assert!(eventually_interval_expr(interval_expr(number_expr(), number_expr()), always_expr(number_expr())).monitorability_check().is_ok());
    // Eventually interval eventually = success
    assert!(eventually_interval_expr(interval_expr(number_expr(), number_expr()), eventually_expr(number_expr())).monitorability_check().is_ok());
    // Always bool and always bool = success
    assert!(always_expr(binary_expr(number_expr(), always_expr(number_expr()), BinaryOperators::And)).monitorability_check().is_ok());
    // Always bool or always bool = success
    assert!(always_expr(binary_expr(number_expr(), always_expr(number_expr()), BinaryOperators::Or)).monitorability_check().is_ok());
}

#[test]
fn should_fail() {
    // Eventually = fail
    assert!(eventually_expr(number_expr()).monitorability_check().is_err());
    // Always eventually = fail
    assert!(always_expr(eventually_expr(number_expr())).monitorability_check().is_err());
    // Eventually always = fail
    assert!(eventually_expr(always_expr(number_expr())).monitorability_check().is_err());
    // Eventually eventually = fail
    assert!(eventually_expr(eventually_expr(number_expr())).monitorability_check().is_err());
    // Eventually always interval = fail
    assert!(eventually_expr(always_interval_expr(interval_expr(number_expr(), number_expr()),number_expr())).monitorability_check().is_err());
    // Eventually eventually interval = fail
    assert!(eventually_expr(eventually_interval_expr(interval_expr(number_expr(), number_expr()),number_expr())).monitorability_check().is_err());
    // Always bool and eventually bool = fail
    assert!(always_expr(binary_expr(number_expr(), eventually_expr(number_expr()), BinaryOperators::And)).monitorability_check().is_err());
    // Always bool or eventually bool = fail
    assert!(always_expr(binary_expr(number_expr(), eventually_expr(number_expr()), BinaryOperators::Or)).monitorability_check().is_err());
}


