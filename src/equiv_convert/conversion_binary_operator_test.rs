use crate::program::member_types::MemberType;
use crate::program::operations::UnaryOperators;
use crate::program::{
    expressions::Expr,
    operations::BinaryOperators,
};
use crate::utils::test_helper_func::{binary_expr, bool_expr, member_expr, number_expr, unary_expr};

#[test]
fn times() {
    for (mut expr, expected) in [
        (binary_expr(number_expr(), number_expr(), BinaryOperators::Times), Expr::Number(5000*5000)),
        (binary_expr(number_expr(), bool_expr(), BinaryOperators::Times), Expr::Number(5000)),
        (binary_expr(bool_expr(), number_expr(), BinaryOperators::Times), Expr::Number(5000)),
        (binary_expr(bool_expr(), bool_expr(), BinaryOperators::Times), Expr::Number(1))
    ] {
        assert!(expr.equiv_convert().is_ok());
        assert!(expr.eq(&expected))
    }
}

#[test]
fn divide() {
    for (mut expr, expected) in [
        (binary_expr(number_expr(), number_expr(), BinaryOperators::Divide), Expr::Number(5000/5000)),
        (binary_expr(number_expr(), bool_expr(), BinaryOperators::Divide), Expr::Number(5000)),
        (binary_expr(bool_expr(), number_expr(), BinaryOperators::Divide), Expr::Number(1/5000)),
        (binary_expr(bool_expr(), bool_expr(), BinaryOperators::Divide), Expr::Number(1))
    ] {
        assert!(expr.equiv_convert().is_ok());
        assert!(expr.eq(&expected))
    }

    
}

#[test]
fn plus() {
    for (mut expr, expected) in [
        (binary_expr(number_expr(), number_expr(), BinaryOperators::Plus), Expr::Number(5000+5000)),
        (binary_expr(number_expr(), bool_expr(), BinaryOperators::Plus), Expr::Number(5000+1)),
        (binary_expr(bool_expr(), number_expr(), BinaryOperators::Plus), Expr::Number(1+5000)),
        (binary_expr(bool_expr(), bool_expr(), BinaryOperators::Plus), Expr::Number(1+1))
    ] {
        assert!(expr.equiv_convert().is_ok());
        assert!(expr.eq(&expected))
    }
}

#[test]
fn minus() {
    for (mut expr, expected) in [
        (binary_expr(number_expr(), number_expr(), BinaryOperators::Minus), Expr::Number(5000-5000)),
        (binary_expr(number_expr(), bool_expr(), BinaryOperators::Minus), Expr::Number(5000-1)),
        (binary_expr(bool_expr(), number_expr(), BinaryOperators::Minus), Expr::Number(1-5000)),
        (binary_expr(bool_expr(), bool_expr(), BinaryOperators::Minus), Expr::Number(1-1)),
    ] {
        assert!(expr.equiv_convert().is_ok());
        assert!(expr.eq(&expected))
    }
}

#[test]
fn modulo() {
    for (mut expr, expected) in [
        (binary_expr(number_expr(), number_expr(), BinaryOperators::Mod), Expr::Number(0)),
        (binary_expr(number_expr(), bool_expr(), BinaryOperators::Mod), Expr::Number(0)),
        (binary_expr(bool_expr(), number_expr(), BinaryOperators::Mod), Expr::Number(1)),
        (binary_expr(bool_expr(), bool_expr(), BinaryOperators::Mod), Expr::Number(0))
    ] {
        assert!(expr.equiv_convert().is_ok());
        assert!(expr.eq(&expected))
    }
}

#[test]
fn and_or_implies_eqal_greaterequal_lessequal() {
    for operator in [BinaryOperators::And, BinaryOperators::Or, BinaryOperators::Implies, BinaryOperators::Equal, BinaryOperators::GreaterEqual, BinaryOperators::LessEqual] {
        for (mut expr, expected) in [
            (binary_expr(number_expr(), number_expr(), operator.clone()), Expr::Boolean(true)),
            (binary_expr(number_expr(), bool_expr(), operator.clone()), Expr::Boolean(true)),
            (binary_expr(bool_expr(), number_expr(), operator.clone()), Expr::Boolean(true)),
            (binary_expr(bool_expr(), bool_expr(), operator.clone()), Expr::Boolean(true)),
        ] {
            assert!(expr.equiv_convert().is_ok());
            assert!(expr.eq(&expected))
        }
    }
   
}

#[test]
fn and_equivalence_conversion() {
    let mut expr = binary_expr(number_expr(), member_expr(MemberType::Active), BinaryOperators::And);
    assert!(expr.equiv_convert().is_ok());
    assert_eq!(expr, unary_expr(binary_expr(unary_expr(number_expr(), UnaryOperators::Not), unary_expr(member_expr(MemberType::Active), UnaryOperators::Not), BinaryOperators::Or), UnaryOperators::Not));
}



#[test]
fn notequal_greater_less() {
    for operator in [BinaryOperators::NotEqual, BinaryOperators::Greater, BinaryOperators::Less] {
        for (mut expr, expected) in [
            (binary_expr(number_expr(), number_expr(), operator.clone()), Expr::Boolean(false)),
            (binary_expr(number_expr(), bool_expr(), operator.clone()), Expr::Boolean(false)),
            (binary_expr(bool_expr(), number_expr(), operator.clone()), Expr::Boolean(false)),
            (binary_expr(bool_expr(), bool_expr(), operator.clone()), Expr::Boolean(false)),
        ] {
            assert!(expr.equiv_convert().is_ok());
            assert!(expr.eq(&expected))
        }
    }
        
}

#[test]
fn implies_equivalence_conversion() {
    let mut expr = binary_expr(number_expr(), member_expr(MemberType::Active), BinaryOperators::Implies);
    assert!(expr.equiv_convert().is_ok());
    assert_eq!(expr, binary_expr(unary_expr(number_expr(), UnaryOperators::Not), member_expr(MemberType::Active), BinaryOperators::Or));
 
}

