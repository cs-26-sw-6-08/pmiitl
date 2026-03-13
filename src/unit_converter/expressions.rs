use crate::program::{expressions::ExprKind, units::Unit};

impl ExprKind {
    pub fn unit_converter(&self) -> ExprKind {
        match self {
            ExprKind::Interval { start, end } => ExprKind::Interval { start: start.unit_converter().into(), end: end.unit_converter().into() },
            ExprKind::Always { interval, not, expr } => ExprKind::Always { interval: interval.clone().map(|interval| interval.unit_converter().into()), not: *not, expr: expr.unit_converter().into() },
            ExprKind::Eventually { interval, not, expr } => ExprKind::Eventually { interval: interval.clone().map(|interval| interval.unit_converter().into()), not: *not, expr: expr.unit_converter().into() },
            ExprKind::Until { interval, not, lhs, rhs } => ExprKind::Until { interval: interval.clone().map(|interval| interval.unit_converter().into()), not: *not, lhs: lhs.unit_converter().into(), rhs: rhs.unit_converter().into() },
            ExprKind::BinaryOperations { lhs, rhs, operator } => ExprKind::BinaryOperations { lhs: lhs.unit_converter().into(), rhs: rhs.unit_converter().into(), operator: operator.clone() },
            ExprKind::UnaryOperations { operand, operator } => ExprKind::UnaryOperations { operand: operand.unit_converter().into(), operator: operator.clone() },
            ExprKind::Function { aggregate_type, expr } => ExprKind::Function { aggregate_type: aggregate_type.clone(), expr: expr.unit_converter().into() },
            ExprKind::Unit { number, unit } => {
                let n = match number.as_ref() {
                    ExprKind::Number(n) => *n,
                    _ => unreachable!()
                };
                match unit {
                    Unit::Minutes => ExprKind::Unit { number: ExprKind::Number(n * 60).into(), unit: Unit::Seconds },
                    Unit::Hours => ExprKind::Unit { number: ExprKind::Number(n * 60 * 60).into(), unit: Unit::Seconds },
                    Unit::KiloWatts => ExprKind::Unit { number: ExprKind::Number(n * 1000).into(), unit: Unit::Watt },
                    Unit::KiloWattHours => ExprKind::Unit { number: ExprKind::Number(n * 1000 * 60 * 60).into(), unit: Unit::WattSeconds },
                    Unit::WattHours => ExprKind::Unit { number: ExprKind::Number(n * 60 * 60).into(), unit: Unit::WattSeconds },
                    Unit::WattMinutes => ExprKind::Unit { number: ExprKind::Number(n * 60).into(), unit: Unit::WattSeconds },
                    _=> self.clone()
                }
            },
            _ => self.clone()
        }
    }
}