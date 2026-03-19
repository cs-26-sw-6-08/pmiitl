use crate::program::{expressions::Expr, units::Unit};

impl Expr {
    pub fn unit_convert(&self) -> Expr {
        match self {
            Expr::Interval { start, end } => Expr::Interval { start: start.unit_convert().into(), end: end.unit_convert().into() },
            Expr::Always { interval, not, expr } => Expr::Always { interval: interval.clone().map(|interval| interval.unit_convert().into()), not: *not, expr: expr.unit_convert().into() },
            Expr::Eventually { interval, not, expr } => Expr::Eventually { interval: interval.clone().map(|interval| interval.unit_convert().into()), not: *not, expr: expr.unit_convert().into() },
            Expr::BinaryOperations { lhs, rhs, operator } => Expr::BinaryOperations { lhs: lhs.unit_convert().into(), rhs: rhs.unit_convert().into(), operator: operator.clone() },
            Expr::UnaryOperations { operand, operator } => Expr::UnaryOperations { operand: operand.unit_convert().into(), operator: operator.clone() },
            Expr::Function { aggregate_type, expr } => Expr::Function { aggregate_type: aggregate_type.clone(), expr: expr.unit_convert().into() },
            Expr::Unit { number, unit } => {
                let Expr::Number(n) = *number.as_ref() else { unreachable!() };
                match unit {
                    Unit::Minutes => Expr::Unit { number: Expr::Number(n * 60).into(), unit: Unit::Seconds },
                    Unit::Hours => Expr::Unit { number: Expr::Number(n * 60 * 60).into(), unit: Unit::Seconds },
                    Unit::KiloWatts => Expr::Unit { number: Expr::Number(n * 1000).into(), unit: Unit::Watt },
                    Unit::KiloWattHours => Expr::Unit { number: Expr::Number(n * 1000 * 60 * 60).into(), unit: Unit::WattSeconds },
                    Unit::WattHours => Expr::Unit { number: Expr::Number(n * 60 * 60).into(), unit: Unit::WattSeconds },
                    Unit::WattMinutes => Expr::Unit { number: Expr::Number(n * 60).into(), unit: Unit::WattSeconds },
                    _=> self.clone()
                }
            },
            _ => self.clone()
        }
    }
}