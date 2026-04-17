use crate::{monitor::{operation_eval, streams::{IoTDevice, IoTStream, OutputStream}, types::{DerivedOutput, Device, StackValue, Verdict}}, monitor_setup::operation_types::{AggregateType, Operation}, program::{member_types::MemberType, operations::BinaryOperators}};


// pub struct OutputStream {
//     ltl: LTL,
//     bound: Option<(i128, i128)>,
//     time_verdicts: Vec<(i128, Verdict)>,
//     operations: Vec<Operation>,
// }

impl OutputStream {
    // Calculate the verdict for the output stream.
    pub fn update(&mut self, t_current: i128, devices: &IoTStream) {
        for (t_spawn, ver) in self.time_verdicts.iter_mut() {
            // *ver = rec_calc_for_t(&mut self.operations, 0, devices, &*t_spawn, None, &t_current)
            //     .get_value()
            //     .get_verdict()
            //     .unwrap();

            let res = calc_for_t_iter(&mut self.operations, devices, &*t_spawn, &t_current);
            *ver = res.get_value().get_verdict().unwrap();
        }
    }
}

// To optimase make use of a worklist or selv coded recursive function: https://gemini.google.com/share/12920c0f930c
fn rec_calc_for_t<'a>(
    operations: &mut [Operation],
    cur_idx: usize, 
    devices: &'a IoTStream,
    t_spawn: &i128,
    device: Option<&Device>,
    t_current: &i128,
) -> StackValue<'a> {
    let unsafe_operations_pointer = operations as *mut [Operation];
    let unsafe_mut_pointer = &mut operations[cur_idx] as *mut Operation;
    match unsafe { &mut*unsafe_mut_pointer } {
        Operation::LTLAlwaysUnbounded { idx } => todo!(),
        Operation::LTLBounded { bound, idx, not, ltl_type } => todo!(),
        Operation::Binary { bin_op, idx_lhs, idx_rhs } => {
            let val1 = rec_calc_for_t(unsafe { &mut* unsafe_operations_pointer }, *idx_lhs, devices, t_spawn, device, t_current);
            let val2 = rec_calc_for_t(unsafe { &mut* unsafe_operations_pointer }, *idx_rhs, devices, t_spawn, device, t_current);
            
            match bin_op { //todo: Christian to-do
                BinaryOperators::Plus => val1 + val2,
                BinaryOperators::Minus => val1 - val2,
                BinaryOperators::Times => val1 * val2,
                BinaryOperators::Divide => val1 / val2,
                BinaryOperators::Mod => val1.modulo(val2),
                BinaryOperators::Equal => val1.equals(val2),
                BinaryOperators::Less => todo!(),
                BinaryOperators::Greater => todo!(),
                BinaryOperators::LessEqual => todo!(),
                BinaryOperators::GreaterEqual => todo!(),
                BinaryOperators::NotEqual => val1.not_equals(val2),
                BinaryOperators::Or => todo!(),
                _ => unreachable!()
            }
        },
        Operation::Unary { un_op, idx } => todo!(),
        Operation::Number(val) => (*val).into(),
        Operation::String(val) => (&*val).into(),
        Operation::Member(member_type) => match member_type {
            MemberType::Active => Verdict::from(devices.get_devices()[0].active).into(),
            MemberType::Power => devices.get_devices()[0].power.into(),
            MemberType::Name => (&devices.get_devices()[0].name).into(),
        },
        Operation::CurrentTime => (*t_spawn).into(),
        Operation::TimeFunction { idx, function_type, history, max_bound } => {
            todo!()
        },
        Operation::AggregateFunction { idx, function_type } => {
            todo!()
        },
        Operation::Foreach { idx } => todo!(),
    }
}

fn calc_for_t_iter<'a>(
    operations: &mut [Operation], 
    devices: &'a IoTStream,
    t_spawn: &i128,
    t_current: &i128,
) -> StackValue<'a> {
    use StepType::*;

    let mut idx_stack: Vec<(usize, StepType)> = Vec::from([(0usize, StepType::Deepen)]);
    let mut value_stack: Vec<StackValue> = Vec::with_capacity(20);
    let mut device_stack: Vec<&IoTDevice> = Vec::with_capacity(20);
    let mut device_pointer: Option<&IoTDevice> = None;

    while let Some((cur_idx, step_type)) = idx_stack.pop() {
        // let cur_op = &mut operations[cur_idx] as *mut Operation;
        let cur_op = &mut operations[cur_idx] as *mut Operation;

        match  (unsafe { &mut*cur_op }, step_type)  {
            // Base cases
            (Operation::Number(val), _) => value_stack.push((*val).into()),
            (Operation::String(val), _) => value_stack.push((&*val).into()),
            (Operation::CurrentTime, _) => value_stack.push((*t_spawn).into()),
            (Operation::Member(mem_type), _) => {
                let msg = "Device pointer was not implemented correctly";
                value_stack.push(match mem_type {
                    MemberType::Active => device_pointer.expect(msg).active.into(),
                    MemberType::Power =>  device_pointer.expect(msg).power.into(),
                    MemberType::Name =>  StackValue::from(device_pointer.map(|d| &d.name).expect(msg)),
                });
            },

            // BinOp / UnOp
            (Operation::Binary { idx_lhs, idx_rhs,.. }, Deepen) => {
                idx_stack.extend([
                    (cur_idx, Reduce),
                    (*idx_rhs, Deepen),
                    (*idx_lhs, Deepen)
                ]);
            },
            (Operation::Binary { bin_op, .. }, Reduce) => {
                let res2 = value_stack.pop();
                let res1 = value_stack.pop();

                value_stack.push(
                    res1
                        .zip(res2)
                        .map(|(v1, v2)| v1.bin_op(v2, bin_op))
                        .expect("Binary operation not implemented correctly")
                );
            },
            (Operation::Unary { idx , ..}, Deepen) => {
                idx_stack.extend([
                    (cur_idx, Reduce),
                    (*idx, Deepen),
                ]);
            },
            (Operation::Unary { un_op, .. }, Reduce) => {
                let res = value_stack.pop()
                        .map(|v1| v1.un_op(un_op))
                        .expect("Binary operation not implemented correctly");

                value_stack.push(res);
            },

            // Aggregate Functions
            (Operation::AggregateFunction { .. }, Deepen) => {
                idx_stack.push((cur_idx, Reduce));
                device_stack.extend(devices.get_devices());
                value_stack.push( 0.into() )
            }
            (Operation::AggregateFunction { function_type, idx }, Reduce) => {
                let res = value_stack.pop();
                if !device_stack.is_empty() {
                    let acc = value_stack.pop();
                    value_stack.push(
                        acc.zip(res).map(|(a, b)| a + b).expect("Error in Aggregate function")
                    );
                    device_pointer = device_stack.pop();
                    idx_stack.extend([
                        (cur_idx, Reduce),
                        (*idx, Deepen),
                        ]);
                    } else {
                        let acc = value_stack.pop();
                        let res = acc.zip(res).map(|(a, b)| a + b).expect("Error in Aggregate function");
                        
                        value_stack.push(
                            match function_type {
                                AggregateType::Sum => res,
                                AggregateType::Avg => res / (devices.get_devices().len() as i128).into(),
                        }
                        );
                    }
                },
            (Operation::Foreach { .. }, Deepen) => {
                idx_stack.push((cur_idx, Reduce));
                device_stack.extend(devices.get_devices());
                value_stack.push( Verdict::True.into() )
            },
            (Operation::Foreach { idx }, Reduce) => {
                let prev_res = value_stack.pop();
                if  prev_res.is_some_and(|v| *v.get_value() == DerivedOutput::Verdict(Verdict::False)) {
                    value_stack.push(Verdict::False.into());
                } else if !device_stack.is_empty() {
                    device_pointer = device_stack.pop();
                    idx_stack.extend([
                        (cur_idx, Reduce),
                        (*idx, Deepen),
                    ]);
                } else {
                    value_stack.push(Verdict::True.into());
                }
            },

            // Time functions
            (Operation::TimeFunction { idx, .. }, Deepen) => {
                idx_stack.extend([(cur_idx, Reduce), (*idx, Deepen)]);
            },
            (Operation::TimeFunction { function_type, history, max_bound, .. }, Reduce) => {
                let val = value_stack.pop().and_then( |v| v.get_value().get_num() ).expect("Time func not working");
                let his_val = match max_bound {
                    Some(bound) => {
                        let arr_idx = (t_spawn % (*bound as i128)) as usize;

                        if history.len() < arr_idx { history.resize(arr_idx+1, 0) }
                        history[arr_idx] += val;
                        history[arr_idx]
                    },
                    None => {
                        history[0] += val;
                        history[0]
                    },
                };

                let val: StackValue = match function_type {
                        AggregateType::Sum => his_val,
                        AggregateType::Avg => his_val/(t_current - t_spawn),
                }.into();
                value_stack.push(val.as_undecided());
            },
            
            // LTL 
            (Operation::LTLAlwaysUnbounded { idx }, Deepen) => {
                idx_stack.extend([(cur_idx, Reduce), (*idx, Deepen)]);
            },
            (Operation::LTLAlwaysUnbounded { .. }, Reduce) => {
                let val = value_stack.pop().expect("Error in LTL");
                value_stack.push(
                    val.and(Verdict::Undecided.into())
                );
            },
            (Operation::LTLBounded { idx, bound, .. }, Deepen) => {
                let (a,_) = bound;
                if *a+*t_spawn <= *t_current {
                    idx_stack.extend([(cur_idx, Reduce), (*idx, Deepen)]);
                }
            },
            (Operation::LTLBounded { bound, not, ltl_type, .. }, Reduce) => {
                let (_, b) = bound;
                let val = value_stack.pop().expect("Error in ltl");
                
                let val = match ltl_type {
                    crate::monitor_setup::operation_types::LTL::Always => {
                        let ver = if *t_current < *t_spawn + *b { Verdict::Undecided } else { Verdict::True };
                        val.and(ver.into())
                    },
                    crate::monitor_setup::operation_types::LTL::Eventually => {
                        let ver = if *t_current < *t_spawn + *b { Verdict::Undecided } else { Verdict::False };
                        val.or(ver.into())
                    }
                };
                value_stack.push(
                    if *not { val.not() } 
                    else { val }
                );
            },
        }
    }
    value_stack.pop().unwrap()
}


#[derive(PartialEq, Debug)]
enum StepType { Deepen, Reduce }