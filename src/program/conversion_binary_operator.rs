
use crate::program::{
    expressions::ExprKind, operations::BinaryOperators, units::Unit,
};

pub fn conversion_binary_operations(lhs: ExprKind, rhs: ExprKind, operator: &BinaryOperators) -> Option<ExprKind> {
    match operator {
        BinaryOperators::Times => match (lhs, rhs) {
            /* Ws * n */
            (ExprKind::Unit { number, unit: Unit::WattSeconds }, ExprKind::Number(m)) |  (ExprKind::Number(m), ExprKind::Unit { number, unit: Unit::WattSeconds }) => match number.as_ref() {
                ExprKind::Number(n) => Some(ExprKind::Unit { number: ExprKind::Number(*n * m).into(), unit: Unit::WattSeconds }),
                _ => None
            },
            /* s * n */
            (ExprKind::Unit { number, unit: Unit::Seconds }, ExprKind::Number(m)) | (ExprKind::Number(m), ExprKind::Unit { number, unit: Unit::Seconds })=> match number.as_ref() {
                ExprKind::Number(n) => Some(ExprKind::Unit { number: ExprKind::Number(n * m).into(), unit: Unit::Seconds }),
                _ => None
            },
            /* s * W */
            (ExprKind::Unit { number: number1, unit: Unit::Seconds }, ExprKind::Unit { number: number2, unit: Unit::Watt }) | (ExprKind::Unit { number: number1, unit: Unit::Watt }, ExprKind::Unit { number: number2, unit: Unit::Seconds })=> match (number1.as_ref(), number2.as_ref()) {
                (ExprKind::Number(n1), ExprKind::Number(n2)) => Some(ExprKind::Unit { number: ExprKind::Number(n1 * n2).into(), unit: Unit::WattSeconds}),
                _ => None
            },
            /* W * n */
            (ExprKind::Unit { number, unit: Unit::Watt }, ExprKind::Number(m)) | (ExprKind::Number(m), ExprKind::Unit { number, unit: Unit::Watt }) => match number.as_ref() {
                ExprKind::Number(n) => Some(ExprKind::Unit { number: ExprKind::Number(n * m).into(), unit: Unit::Watt }),
                _ => None
            },
            /* n * m */
            (ExprKind::Number(n), ExprKind::Number(m)) => Some(ExprKind::Number(n*m)),
            /* n * b | b * n */
            (ExprKind::Number(n), ExprKind::Boolean(b)) | (ExprKind::Boolean(b), ExprKind::Number(n)) => Some(ExprKind::Number(n * if b { 1 } else { 0 })),
            _ => None,
        }
        BinaryOperators::Divide => match (lhs, rhs) {
            /* Ws / n */
            (ExprKind::Unit { number, unit: Unit::WattSeconds }, ExprKind::Number(m)) => match number.as_ref() {
                ExprKind::Number(n) => Some(ExprKind::Unit { number: ExprKind::Number(*n / m).into(), unit: Unit::WattSeconds }),
                _ => None
            },
            /* Ws / s */
            (ExprKind::Unit { number: number1, unit: Unit::WattSeconds }, ExprKind::Unit { number: number2, unit: Unit::Seconds }) => match (number1.as_ref(), number2.as_ref()) {
                (ExprKind::Number(n1), ExprKind::Number(n2)) => Some(ExprKind::Unit { number: ExprKind::Number(*n1 / *n2).into(), unit: Unit::Watt}),
                _ => None
            },
            /* Ws / W */
            (ExprKind::Unit { number: number1, unit: Unit::WattSeconds }, ExprKind::Unit { number: number2, unit: Unit::Watt }) => match (number1.as_ref(), number2.as_ref()) {
                (ExprKind::Number(n1), ExprKind::Number(n2)) => Some(ExprKind::Unit { number: ExprKind::Number(n1 / *n2).into(), unit: Unit::Seconds }),
                _ => None
            },
            /* s / n */
            (ExprKind::Unit { number, unit: Unit::Seconds }, ExprKind::Number(m)) => match number.as_ref() {
                ExprKind::Number(n) => Some(ExprKind::Unit { number: ExprKind::Number(n / m).into(), unit: Unit::Seconds }),
                _ => None
            },
            /* W / n */
            (ExprKind::Unit { number, unit: Unit::Watt }, ExprKind::Number(m)) => match number.as_ref() {
                ExprKind::Number(n) => Some(ExprKind::Unit { number: ExprKind::Number(n / m).into(), unit: Unit::Watt }),
                _ => None
            },
            /* n / m */
            (ExprKind::Number(n), ExprKind::Number(m)) => Some(ExprKind::Number(n/m)),
            /* n / b */
            (ExprKind::Number(n), ExprKind::Boolean(b)) => Some(ExprKind::Number(n / if b { 1 } else { 0 })),
            /* b / n */
            (ExprKind::Boolean(b), ExprKind::Number(n)) => Some(ExprKind::Number(if b { 1 } else { 0 } / n)),
            _ => None,
        }
        BinaryOperators::Plus => match (lhs, rhs) {
            /* Ws + Ws */
            (ExprKind::Unit { number: number1, unit: Unit::WattSeconds }, ExprKind::Unit { number: number2, unit: Unit::WattSeconds }) => match (number1.as_ref(), number2.as_ref()) {
                (ExprKind::Number(n1), ExprKind::Number(n2)) => Some(ExprKind::Unit { number: ExprKind::Number(*n1 + *n2).into(), unit: Unit::WattSeconds }),
                _ => None
            },
            /* s + s */
            (ExprKind::Unit { number: number1, unit: Unit::Seconds }, ExprKind::Unit { number: number2, unit: Unit::Seconds }) => match (number1.as_ref(), number2.as_ref()) {
                (ExprKind::Number(n1), ExprKind::Number(n2)) => Some(ExprKind::Unit { number: ExprKind::Number(n1 + n2).into(), unit: Unit::Seconds}),
                _ => None
            },
             /* W + W */
            (ExprKind::Unit { number: number1, unit: Unit::Watt }, ExprKind::Unit { number: number2, unit: Unit::Watt }) => match (number1.as_ref(), number2.as_ref()) {
                (ExprKind::Number(n1), ExprKind::Number(n2)) => Some(ExprKind::Unit { number: ExprKind::Number(n1 + n2).into(), unit: Unit::Watt}),
                _ => None
            },
            /* n + m */
            (ExprKind::Number(n), ExprKind::Number(m)) => Some(ExprKind::Number(n+m)),
            /* n + b | b + n*/
            (ExprKind::Number(n), ExprKind::Boolean(b)) | (ExprKind::Boolean(b), ExprKind::Number(n)) => Some(ExprKind::Number(n + if b { 1 } else { 0 })),
            _ => None
        }
        BinaryOperators::Minus => match (lhs, rhs) {
            /* Ws - Ws */
            (ExprKind::Unit { number: number1, unit: Unit::WattSeconds }, ExprKind::Unit { number: number2, unit: Unit::WattSeconds }) => match (number1.as_ref(), number2.as_ref()) {
                (ExprKind::Number(n1), ExprKind::Number(n2)) => Some(ExprKind::Unit { number: ExprKind::Number(*n1 - *n2).into(), unit: Unit::WattSeconds }),
                _ => None
            },
            /* s - s */
            (ExprKind::Unit { number: number1, unit: Unit::Seconds }, ExprKind::Unit { number: number2, unit: Unit::Seconds }) => match (number1.as_ref(), number2.as_ref()) {
                (ExprKind::Number(n1), ExprKind::Number(n2)) => Some(ExprKind::Unit { number: ExprKind::Number(n1 - n2).into(), unit: Unit::Seconds}),
                _ => None
            },
            /* W - W */
            (ExprKind::Unit { number: number1, unit: Unit::Watt }, ExprKind::Unit { number: number2, unit: Unit::Watt }) => match (number1.as_ref(), number2.as_ref()) {
                (ExprKind::Number(n1), ExprKind::Number(n2)) => Some(ExprKind::Unit { number: ExprKind::Number(n1 - n2).into(), unit: Unit::Watt }),
                _ => None,
            },
            /* n - m */
            (ExprKind::Number(n), ExprKind::Number(m)) => Some(ExprKind::Number(n-m)),
            /* n - b */
            (ExprKind::Number(n), ExprKind::Boolean(b)) => Some(ExprKind::Number(n - if b { 1 } else { 0 })),
            /* b - n */
            (ExprKind::Boolean(b), ExprKind::Number(n)) => Some(ExprKind::Number(if b { 1 } else { 0 } - n)),
            _ => None,
        }
        BinaryOperators::Mod => match (lhs, rhs) {
            /* W % W */
            (ExprKind::Unit { number: number1, unit: Unit::Watt }, ExprKind::Unit { number: number2, unit: Unit::Watt }) => match (number1.as_ref(), number2.as_ref()) {
                (ExprKind::Number(n1), ExprKind::Number(n2)) => Some(ExprKind::Unit { number: ExprKind::Number(n1 % n2).into(), unit: Unit::Watt}),
                _ => None
            },
            /* s % s */
            (ExprKind::Unit { number: number1, unit: Unit::Seconds }, ExprKind::Unit { number: number2, unit: Unit::Seconds }) => match (number1.as_ref(), number2.as_ref()) {
                (ExprKind::Number(n1), ExprKind::Number(n2)) => Some(ExprKind::Unit { number: ExprKind::Number(n1 % n2).into(), unit: Unit::Seconds}),
                _ => None
            },
            /* Ws % Ws */
            (ExprKind::Unit { number: number1, unit: Unit::WattSeconds }, ExprKind::Unit { number: number2, unit: Unit::WattSeconds }) => match (number1.as_ref(), number2.as_ref()) {
                (ExprKind::Number(n1), ExprKind::Number(n2)) => Some(ExprKind::Unit { number: ExprKind::Number(n1 % n2).into(), unit: Unit::WattSeconds}),
                _ => None
            },
            /* n % m */
            (ExprKind::Number(n), ExprKind::Number(m)) => Some(ExprKind::Number(n%m)),
             /* n % b */
            (ExprKind::Number(n), ExprKind::Boolean(b)) => Some(ExprKind::Number(n % if b { 1 } else { 0 })),
            /* b % n */
            (ExprKind::Boolean(b), ExprKind::Number(n)) => Some(ExprKind::Number(if b { 1 } else { 0 } % n)),
            _ => None,
            
        },
        BinaryOperators::And => match (lhs, rhs) {
             /* b & n */
            (ExprKind::Boolean(b), ExprKind::Number(n)) => Some(ExprKind::Boolean(b && n != 0)),
            /* n & b */
            (ExprKind::Number(n), ExprKind::Boolean(b)) => Some(ExprKind::Boolean(n != 0 && b)),
            /* p & q */
            (ExprKind::Boolean(p), ExprKind::Boolean(q)) => Some(ExprKind::Boolean(p&&q)),
            _ => None,
        },
        BinaryOperators::Or => match (lhs, rhs) {
            /* p | q */
            (ExprKind::Boolean(p), ExprKind::Boolean(q)) => Some(ExprKind::Boolean(p||q)),
            /* b | n */
            (ExprKind::Boolean(b), ExprKind::Number(n)) => Some(ExprKind::Boolean(b || n != 0)),
            /* n | b */
            (ExprKind::Number(n), ExprKind::Boolean(b)) => Some(ExprKind::Boolean(n != 0 || b)),
            _ => None,
        },
        BinaryOperators::Equal => match (lhs, rhs) {
            /* p = q */
            (ExprKind::Boolean(p), ExprKind::Boolean(q)) => Some(ExprKind::Boolean(p==q)),
            /* n = n */
            (ExprKind::Number(n), ExprKind::Number(m)) => Some(ExprKind::Boolean(n==m)),
            /* b = n */
            (ExprKind::Boolean(b), ExprKind::Number(n)) => Some(ExprKind::Boolean(b == (n != 0))),
            /* n = b */
            (ExprKind::Number(n), ExprKind::Boolean(b)) => Some(ExprKind::Boolean((n != 0) == b)),
            _ => None,
        },
        BinaryOperators::NotEqual => match (lhs, rhs) {
            /* p != q */
            (ExprKind::Boolean(p), ExprKind::Boolean(q)) => Some(ExprKind::Boolean(p!=q)),
            /* n != n */
            (ExprKind::Number(n), ExprKind::Number(m)) => Some(ExprKind::Boolean(n!=m)),
            /* b != n */
            (ExprKind::Boolean(b), ExprKind::Number(n)) => Some(ExprKind::Boolean(b != (n != 0))),
            /* n != b*/
            (ExprKind::Number(n), ExprKind::Boolean(b)) => Some(ExprKind::Boolean((n != 0) != b)),
            _ => None,
        },
        BinaryOperators::Greater => match (lhs, rhs) {
            /* n > n */
            (ExprKind::Number(n), ExprKind::Number(m)) => Some(ExprKind::Boolean(n>m)),
            /* n > b */
            (ExprKind::Number(n), ExprKind::Boolean(b)) => Some(ExprKind::Boolean((n != 0) > b)),
            /* b > n */
            (ExprKind::Boolean(b), ExprKind::Number(n)) => Some(ExprKind::Boolean(b > (n != 0))),
            _ => None,
        },
        BinaryOperators::GreaterEqual => match (lhs, rhs) {
            /* n >= n */
            (ExprKind::Number(n), ExprKind::Number(m)) => Some(ExprKind::Boolean(n>=m)),
            /* n >= b */
            (ExprKind::Number(n), ExprKind::Boolean(b)) => Some(ExprKind::Boolean((n != 0) >= b)),
            /* b >= n */  
            (ExprKind::Boolean(b), ExprKind::Number(n)) => Some(ExprKind::Boolean(b >= (n != 0))),
            _ => None,      
        },
        BinaryOperators::Less => match (lhs, rhs) {
            /* n < n */
            (ExprKind::Number(n), ExprKind::Number(m)) => Some(ExprKind::Boolean(n<m)),
            /* n < b */
            (ExprKind::Number(n), ExprKind::Boolean(b)) => Some(ExprKind::Boolean((n != 0) < b)),
            /* b < n */
            (ExprKind::Boolean(b), ExprKind::Number(n)) => Some(ExprKind::Boolean(b < (n != 0))),
            _ => None,
        },
        BinaryOperators::LessEqual => match (lhs, rhs) {
            /* n <= n */
            (ExprKind::Number(n), ExprKind::Number(m)) => Some(ExprKind::Boolean(n<=m)),
             /* b <= n */
            (ExprKind::Boolean(b), ExprKind::Number(n)) => Some(ExprKind::Boolean(b <= (n != 0))),
            /* n <= b */
            (ExprKind::Number(n), ExprKind::Boolean(b)) => Some(ExprKind::Boolean((n != 0) <= b)),
            _ => None
        },
        BinaryOperators::Implies => todo!(),
    }
}

/*
IMPLIES!!! b -> b, b->n

n+n,n-n,n*n,n/n,n%n   => n
b&b, b|b, b->b, b!=b, b=b   => b

n<n, n>=n, n<=n, n!=n, n=n, n>n  =>b

n+b, n-b, n*b, n/b, n%b => n
b&n, b|n, b->n, b=n, b!=n, n<b, n>=b, n<=b, n>b  => b

!b !n => b
-b, -n => n
*/

/*
kWh => Ws
Wh => Ws
Wm => Ws
Ws => Ws*n, Ws/n, Ws+Ws, Ws-Ws, Ws/s, Ws/W, Ws/Ws, Ws / W

h 
m
s => s*n, s/n, s+s, s-s, s*W, s/s, 

kW
W => W*n, W/n, W+W, W-W, W/W, s*W, 

*/