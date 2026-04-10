use std::collections::HashMap;

use crate::{monitor_setup::{streams::DerivedStream, types::{DerivedOutput, Device}}, program::{expressions::Expr,function_types::FunctionType}};


impl Expr {
    pub fn eval_expression(
    &self,
    mut streams: Vec<DerivedStream>, 
    key: usize,
    devices: &HashMap<i128, Vec<Device>>, 
    time_stream: &i128
) -> (Vec<DerivedStream>, usize) {   
        match self {
            Expr::Number(c) => {
                let value = *c;
                streams.push(
                    DerivedStream::from_fn(
                        Box::new(move |_, ds, _, _| DerivedOutput::Number(value))
                    )
                );
                (streams, key + 1)
            },
            Expr::String(str) => {
                todo!();
                // let value = str.clone();
                // streams.push(
                //     DerivedStream::from_fn(
                //         Box::new(move |_, _, _| DerivedOutput::String(value))
                //     )
                // );
                // (streams, key + 1)
            },
            Expr::CurrentTime => {
                streams.push(
                    DerivedStream::from_fn(
                        Box::new(move |_, ds, _, current_time| DerivedOutput::Number(current_time))
                    )
                );
                (streams, key + 1)
            },
            Expr::Unit { number, unit } => unreachable!(),
            Expr::Interval { start, end } => todo!(),
            Expr::Always { interval, not, expr } => todo!(),
            Expr::Eventually { interval, not, expr } => todo!(),
            Expr::BinaryOperations { lhs, rhs, operator } => todo!(),
            Expr::UnaryOperations { operand, operator } => todo!(),
            Expr::Member { access_type } => todo!(),
            Expr::Function { aggregate_type, expr } => match aggregate_type {
                FunctionType::Sum => {
                    let (mut streams, key_new) = expr.eval_expression(streams, key, devices, time_stream);
                    streams.reserve(1);

                    let f_ptr  = &streams[key_new - 1] as *const DerivedStream;
                    let device_ptr = devices as *const HashMap<i128, Vec<Device>>;

                    streams.push(
                        DerivedStream::from_fn(
                            Box::new(move |t_prime, ds, _, t| {
                                let devices_map = unsafe { &*device_ptr }; //Unsafe ok in this case :)
                                let unsafe_f = unsafe { &*f_ptr }; //Unsafe ok in this case :)

                                DerivedOutput::Number(devices_map.get(&t).unwrap().iter().fold(0, |acc, device| {
                                    match (unsafe_f.0)(t_prime, ds, Some(device), t) {
                                        DerivedOutput::Number(n) => n + acc,
                                        _ => unreachable!()
                                    }
                                }))
                            })
                        )
                    );
                    (streams, key_new + 1)

                },
                FunctionType::Avg => todo!(),
                FunctionType::Sumtime => todo!(),
                FunctionType::Avgtime => todo!(),
                FunctionType::Foreach => todo!(),
                FunctionType::Count|FunctionType::Counttime => unreachable!(),
            },
        }
    }
}

