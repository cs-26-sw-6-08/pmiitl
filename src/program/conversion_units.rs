use crate::program::{expressions::ExprKind, units::Unit};

pub fn conversion_unit(expr: &ExprKind) -> ExprKind {
    match expr {
        ExprKind::Unit { number, unit: Unit::KiloWattHours } => match number.as_ref() {
            ExprKind::Number(n) => ExprKind::Unit { number: ExprKind::Number(*n * 1000 * 60 * 60).into(), unit: Unit::WattSeconds },
            _ => unreachable!()
        },
        ExprKind::Unit { number, unit: Unit::WattHours } => match number.as_ref() {
            ExprKind::Number(n) => ExprKind::Unit { number: ExprKind::Number(*n * 60 * 60).into(), unit: Unit::WattSeconds },
            _ => unreachable!()
        },
        ExprKind::Unit { number, unit: Unit::WattMinutes } => match number.as_ref() {
            ExprKind::Number(n) => ExprKind::Unit { number: ExprKind::Number(*n * 60).into(), unit: Unit::WattSeconds },
            _ => unreachable!()
        },
        ExprKind::Unit { number, unit: Unit::Hours } => match number.as_ref() {
            ExprKind::Number(n) => ExprKind::Unit { number: ExprKind::Number(*n * 60 * 60).into(), unit: Unit::Seconds },
            _ => unreachable!()
        },
        ExprKind::Unit { number, unit: Unit::Minutes } => match number.as_ref() {
            ExprKind::Number(n) => ExprKind::Unit { number: ExprKind::Number(*n * 60).into(), unit: Unit::Seconds },
            _ => unreachable!()
        },
        ExprKind::Unit { number, unit: Unit::KiloWatts } => match number.as_ref() {
            ExprKind::Number(n) => ExprKind::Unit { number: ExprKind::Number(*n * 1000).into(), unit: Unit::Watt },
            _ => unreachable!()
        },
        _ => expr.clone()
    }
}