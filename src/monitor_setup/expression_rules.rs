use std::{collections::HashMap, rc::Rc};

use crate::{monitor_setup::{streams::DerivedStream, types::{DerivedOutput, Device}}, program::{expressions::Expr,function_types::FunctionType}};


impl Expr {
    pub fn eval_expression(
        &self,
        mut streams: Vec<DerivedStream>, 
        key: usize,
        devices: &Rc<HashMap<i128, Vec<Device>>>, 
        time_stream: &Rc<i128>
    ) -> (Vec<DerivedStream>, usize) {   
        match self {
            Expr::Number(c) => {
                let c = *c;
                streams.push(
                    DerivedStream::from_fn(
                        Box::new(move |_, _, _| DerivedOutput::Number(c))
                    )
                );
                (streams, key + 1)
            },
            Expr::String(str) => {
                let value: Rc<str> = str.clone().into();
                streams.push(
                    DerivedStream::from_fn(
                        Box::new(
                            move |_, _, _| DerivedOutput::String(Rc::clone(&value))
                        )
                    )
                );
                (streams, key + 1)
            },
            Expr::CurrentTime => {
                streams.push(
                    DerivedStream::from_fn(
                        Box::new(move |_, _, current_time| DerivedOutput::Number(current_time))
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

                    let f1  = streams[key_new - 1].clone_rc();
                    let devices = Rc::clone(devices);

                    streams.push(
                        DerivedStream::from_fn(
                            Box::new(move |t_prime, _, t| 
                                DerivedOutput::Number(
                                    devices.get(&t).map(|devices| 
                                        devices
                                            .iter()
                                            .fold(0i128, |acc, device|
                                                if let DerivedOutput::Number(n) = f1(t_prime, Some(device), t) { n + acc } 
                                                else { panic!() }
                                            )
                                    ).unwrap_or_default()
                                )
                            )
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

