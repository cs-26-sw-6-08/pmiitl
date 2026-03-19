use crate::program::{expressions::Expr, units::Unit};

impl Expr {
    pub fn unit_convert(&mut self) {
        match self {
            Expr::Interval { start, end } => {
                start.unit_convert();
                end.unit_convert();
            }
            Expr::Always {
                interval,
                not: _,
                expr,
            } => {
                if let Some(interval) = interval {
                    interval.unit_convert();
                };
                expr.unit_convert();
            }
            Expr::Eventually {
                interval,
                not: _,
                expr,
            } => {
                if let Some(interval) = interval {
                    interval.unit_convert();
                };
                expr.unit_convert();
            }
            Expr::BinaryOperations { lhs, rhs, operator: _ } => {
                lhs.unit_convert();
                rhs.unit_convert();
            }
            Expr::UnaryOperations { operand, operator: _ } => operand.unit_convert(),
            Expr::Function {
                aggregate_type: _,
                expr,
            } => expr.unit_convert(),
            Expr::Unit { number, unit } => {
                let Expr::Number(n) = number.as_mut() else {
                    unreachable!()
                };
                match unit {
                    Unit::Minutes => {
                        *n *= 60;
                        *unit = Unit::Seconds;
                    },
                    Unit::Hours => {
                        *n *= 60 * 60;
                        *unit = Unit::Seconds;
                    },
                    Unit::KiloWatts => {
                        *n *= 1000;
                        *unit = Unit::Watt;
                    },
                    Unit::KiloWattHours => {
                        *n *= 1000 * 60 * 60;
                        *unit = Unit::WattSeconds;
                    },
                    Unit::WattHours => {
                        *n *= 60 * 60;
                        *unit = Unit::WattSeconds;
                    },
                    Unit::WattMinutes => {
                        *n *= 60;
                        *unit = Unit::WattSeconds;
                    },
                    _ => {},
                };
            }
            _ => {},
        }
    }
}
