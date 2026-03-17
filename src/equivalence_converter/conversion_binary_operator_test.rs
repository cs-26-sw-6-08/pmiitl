use crate::program::member_types::MemberType;
use crate::program::operations::UnaryOperators;
use crate::program::{
    expressions::Expr,
    operations::BinaryOperators,
};
use crate::utils::test_helper_func::{always_expr, binary_expr, bool_expr, current_time, eventually_expr, function_expr, interval_expr, member_expr, number_expr, custom_number_expr, string_expr, unary_expr, unit_expr, custom_unit_expr, until_expr};

#[test]
fn times() {
    assert_eq!(binary_expr(number_expr(), number_expr(), BinaryOperators::Times).convert().unwrap(), Expr::Number(5000*5000));
    assert_eq!(binary_expr(number_expr(), bool_expr(), BinaryOperators::Times).convert().unwrap(), Expr::Number(5000));
    assert_eq!(binary_expr(bool_expr(), number_expr(), BinaryOperators::Times).convert().unwrap(), Expr::Number(5000));
    assert_eq!(binary_expr(bool_expr(), bool_expr(), BinaryOperators::Times).convert().unwrap(), Expr::Number(1));
}

#[test]
fn divide() {
    assert_eq!(binary_expr(number_expr(), number_expr(), BinaryOperators::Divide).convert().unwrap(), Expr::Number(5000/5000));
    assert_eq!(binary_expr(number_expr(), bool_expr(), BinaryOperators::Divide).convert().unwrap(), Expr::Number(5000));
    assert_eq!(binary_expr(bool_expr(), number_expr(), BinaryOperators::Divide).convert().unwrap(), Expr::Number(1/5000));
    assert_eq!(binary_expr(bool_expr(), bool_expr(), BinaryOperators::Divide).convert().unwrap(), Expr::Number(1));
}

#[test]
fn plus() {
    assert_eq!(binary_expr(number_expr(), number_expr(), BinaryOperators::Plus).convert().unwrap(), Expr::Number(5000+5000));
    assert_eq!(binary_expr(number_expr(), bool_expr(), BinaryOperators::Plus).convert().unwrap(), Expr::Number(5000+1));
    assert_eq!(binary_expr(bool_expr(), number_expr(), BinaryOperators::Plus).convert().unwrap(), Expr::Number(1+5000));
    assert_eq!(binary_expr(bool_expr(), bool_expr(), BinaryOperators::Plus).convert().unwrap(), Expr::Number(1+1));
}

#[test]
fn minus() {
    assert_eq!(binary_expr(number_expr(), number_expr(), BinaryOperators::Minus).convert().unwrap(), Expr::Number(5000-5000));
    assert_eq!(binary_expr(number_expr(), bool_expr(), BinaryOperators::Minus).convert().unwrap(), Expr::Number(5000-1));
    assert_eq!(binary_expr(bool_expr(), number_expr(), BinaryOperators::Minus).convert().unwrap(), Expr::Number(1-5000));
    assert_eq!(binary_expr(bool_expr(), bool_expr(), BinaryOperators::Minus).convert().unwrap(), Expr::Number(1-1));
}

#[test]
fn modulo() {
    assert_eq!(binary_expr(number_expr(), number_expr(), BinaryOperators::Mod).convert().unwrap(), Expr::Number(0));
    assert_eq!(binary_expr(number_expr(), bool_expr(), BinaryOperators::Mod).convert().unwrap(), Expr::Number(0));
    assert_eq!(binary_expr(bool_expr(), number_expr(), BinaryOperators::Mod).convert().unwrap(), Expr::Number(1));
    assert_eq!(binary_expr(bool_expr(), bool_expr(), BinaryOperators::Mod).convert().unwrap(), Expr::Number(0));
}

#[test]
fn and_or_implies_eqal_greaterequal_lessequal() {
    for operator in [BinaryOperators::And, BinaryOperators::Or, BinaryOperators::Implies, BinaryOperators::Equal, BinaryOperators::GreaterEqual, BinaryOperators::LessEqual] {
        assert_eq!(binary_expr(number_expr(), number_expr(), operator.clone()).convert().unwrap(), Expr::Boolean(true));
        assert_eq!(binary_expr(number_expr(), bool_expr(), operator.clone()).convert().unwrap(), Expr::Boolean(true));
        assert_eq!(binary_expr(bool_expr(), number_expr(), operator.clone()).convert().unwrap(), Expr::Boolean(true));
        assert_eq!(binary_expr(bool_expr(), bool_expr(), operator.clone()).convert().unwrap(), Expr::Boolean(true));
    }
   
}

#[test]
fn and_equivalence_conversion() {
    assert_eq!(binary_expr(number_expr(), member_expr(MemberType::Active), BinaryOperators::And).convert().unwrap(), 
                unary_expr(binary_expr(unary_expr(number_expr(), UnaryOperators::Not), unary_expr(member_expr(MemberType::Active), UnaryOperators::Not), BinaryOperators::Or), UnaryOperators::Not));
}



#[test]
fn notequal_greater_less() {
    for operator in [BinaryOperators::NotEqual, BinaryOperators::Greater, BinaryOperators::Less] {
        assert_eq!(binary_expr(number_expr(), number_expr(), BinaryOperators::NotEqual).convert().unwrap(), Expr::Boolean(false));
        assert_eq!(binary_expr(number_expr(), bool_expr(), BinaryOperators::NotEqual).convert().unwrap(), Expr::Boolean(false));
        assert_eq!(binary_expr(bool_expr(), number_expr(), BinaryOperators::NotEqual).convert().unwrap(), Expr::Boolean(false));
        assert_eq!(binary_expr(bool_expr(), bool_expr(), BinaryOperators::NotEqual).convert().unwrap(), Expr::Boolean(false));
    }
}

#[test]
fn implies_equivalence_conversion() {
    assert_eq!(binary_expr(number_expr(), member_expr(MemberType::Active), BinaryOperators::Implies).convert().unwrap(), 
                binary_expr(unary_expr(number_expr(), UnaryOperators::Not), member_expr(MemberType::Active), BinaryOperators::Or));
}

