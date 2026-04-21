use crate::program::member_types::MemberType;
use crate::program::operations::UnaryOperators;
use crate::program::{
    expressions::Expr,
    operations::BinaryOperators,
};
use crate::utils::test_helper_func::{binary_expr, member_expr, number_expr, custom_number_expr, unary_expr};

#[test]
fn times() {
    let mut expr = binary_expr(number_expr(), number_expr(), BinaryOperators::Times);
    assert!(expr.equiv_convert().is_ok());
    assert_eq!(expr, Expr::Number(25_000));
}

#[test]
fn divide() {
    for (mut expr, expected) in [
        (binary_expr(custom_number_expr(25_000), custom_number_expr(5_000), BinaryOperators::Divide), Expr::Number(5_000)),
        (binary_expr(custom_number_expr(123), custom_number_expr(5), BinaryOperators::Divide), Expr::Number(24)),
        (binary_expr(custom_number_expr(5), custom_number_expr(-0), BinaryOperators::Divide), Expr::Number(0)),
        (binary_expr(number_expr(), custom_number_expr(0), BinaryOperators::Divide), Expr::Number(0)),
        (binary_expr(custom_number_expr(5_000), custom_number_expr(2_000), BinaryOperators::Divide), Expr::Number(2_500))
    ] {
        assert!(expr.equiv_convert().is_ok());
        println!("{}", expr);
        assert!(expr.eq(&expected))
    }

    
}

#[test]
fn plus() {
    let mut expr = binary_expr(number_expr(), number_expr(), BinaryOperators::Plus);
    assert!(expr.equiv_convert().is_ok());
    assert_eq!(expr, Expr::Number(5_000+5_000));
}

#[test]
fn minus() {
    let mut expr = binary_expr(number_expr(), number_expr(), BinaryOperators::Minus);
    assert!(expr.equiv_convert().is_ok());
    assert_eq!(expr, Expr::Number(5_000-5_000));
}

#[test]
fn modulo() {
    for (mut expr, expected) in [
        (binary_expr(custom_number_expr(2), custom_number_expr(4), BinaryOperators::Mod), Expr::Number(2)),
        (binary_expr(custom_number_expr(123), custom_number_expr(1_000), BinaryOperators::Mod), Expr::Number(123)),
        (binary_expr(custom_number_expr(5_000), custom_number_expr(1_000), BinaryOperators::Mod), Expr::Number(0)),
        (binary_expr(custom_number_expr(123_456_789), custom_number_expr(1_234), BinaryOperators::Mod), Expr::Number(25)),
        (binary_expr(number_expr(), custom_number_expr(0), BinaryOperators::Mod), Expr::Number(0)),
        (binary_expr(custom_number_expr(-50), custom_number_expr(12), BinaryOperators::Mod), Expr::Number(10)),
        (binary_expr(custom_number_expr(50), custom_number_expr(-12), BinaryOperators::Mod), Expr::Number(-10)),
    ] {
        assert!(expr.equiv_convert().is_ok());
        assert!(expr.eq(&expected))
    }
}

#[test]
fn and_or_implies_eqal_greaterequal_lessequal() {
    for operator in [BinaryOperators::And, BinaryOperators::Or, BinaryOperators::Implies, BinaryOperators::Equal, BinaryOperators::GreaterEqual, BinaryOperators::LessEqual] {
        let mut expr = binary_expr(number_expr(), number_expr(), operator.clone());
        assert!(expr.equiv_convert().is_ok());
        assert_eq!(expr, Expr::Number(1_000));
    }
   
}

#[test]
fn and_equivalence_conversion() {
    let mut expr = binary_expr(number_expr(), member_expr(MemberType::Power), BinaryOperators::And);
    assert!(expr.equiv_convert().is_ok());
    assert_eq!(expr, unary_expr(binary_expr(unary_expr(number_expr(), UnaryOperators::Not), unary_expr(member_expr(MemberType::Power), UnaryOperators::Not), BinaryOperators::Or), UnaryOperators::Not));
}



#[test]
fn notequal_greater_less() {
    for operator in [BinaryOperators::NotEqual, BinaryOperators::Greater, BinaryOperators::Less] {
        let mut expr = binary_expr(number_expr(), number_expr(), operator.clone());
        assert!(expr.equiv_convert().is_ok());
        assert_eq!(expr, Expr::Number(0));
    }
        
}

#[test]
fn implies_equivalence_conversion() {
    let mut expr = binary_expr(number_expr(), member_expr(MemberType::Power), BinaryOperators::Implies);
    assert!(expr.equiv_convert().is_ok());
    assert_eq!(expr, binary_expr(unary_expr(number_expr(), UnaryOperators::Not), member_expr(MemberType::Power), BinaryOperators::Or));
 
}

