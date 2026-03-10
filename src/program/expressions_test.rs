use crate::program::{
    Program,
    expressions::{ExprKind, SpannedExpr}, operations::BinaryOperators,
};

#[test]
fn test() {
    let program = Program::new("always 1 > 5;").unwrap();
    assert_eq!(
        program.expressions.first(),
        Some(&SpannedExpr {
            expr: ExprKind::Always {
                interval: None,
                not: false,
                expr: Box::new(SpannedExpr {
                    expr: ExprKind::BinaryOperations {
                        lhs: Box::new(SpannedExpr {
                            expr: ExprKind::Number(1000),
                            line: 1,
                            column: 8
                        }),
                        rhs: Box::new(SpannedExpr {
                            expr: ExprKind::Number(5000),
                            line: 1,
                            column: 12
                        }),
                        operator: BinaryOperators::Greater
                    },
                    line: 1,
                    column: 10
                })
            },
            line: 1,
            column: 1
        })
    );
}
