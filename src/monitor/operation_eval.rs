use crate::{monitor::{streams::{IoTDevice, IoTStream, OutputStream}, types::{DerivedOutput, StackValue, Verdict}}, monitor_setup::operation_types::{AggregateType, HistoryValue, Operation}, program::member_types::MemberType};


impl OutputStream {
    // Calculate the verdict for the output stream.
    pub fn update(&mut self, t_current: i128, devices: &IoTStream) {
        for (t_spawn, ver) in self.time_verdicts.iter_mut() {
            let res = eval_operations(&mut self.operations, devices, &*t_spawn, &t_current);
            *ver = res.get_value().get_verdict().unwrap();
        }
    }
}

#[derive(PartialEq, Debug)]
enum StepType { Deepen, Reduce }

fn eval_operations<'a>(
    operations: &mut [Operation], 
    devices: &'a IoTStream,
    t_spawn: &i128,
    t_current: &i128,
) -> StackValue<'a> {
    use StepType::*;

    let mut idx_stack: Vec<(usize, StepType)> = Vec::with_capacity(50);
    let mut value_stack: Vec<StackValue> = Vec::with_capacity(50);
    let mut device_stack: Vec<&IoTDevice> = Vec::with_capacity(50);
    let mut device_pointer: Option<&IoTDevice> = None;

    idx_stack.push((0usize, StepType::Deepen));
   
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
            (Operation::Unary { idx , ..}, Deepen) => { idx_stack.extend([(cur_idx, Reduce),(*idx, Deepen)]); },
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

                //Violation Occurred with one of the devices
                if  prev_res.is_some_and(|v| *v.get_value() == DerivedOutput::Verdict(Verdict::False)) {
                    value_stack.push(Verdict::False.into());
                
                //Not all devices have been looked at yet
                } else if !device_stack.is_empty() {
                    device_pointer = device_stack.pop();
                    idx_stack.extend([
                        (cur_idx, Reduce),
                        (*idx, Deepen),
                    ]);

                //No devices violated the expression
                } else {
                    value_stack.push(Verdict::True.into());
                }
            },

            // Time functions
            (Operation::TimeFunction { idx, .. }, Deepen) => {
                idx_stack.extend([(cur_idx, Reduce), (*idx, Deepen)]);
            },
            (Operation::TimeFunction { function_type, history, max_bound, .. }, Reduce) => {
                let res = value_stack.pop().and_then( |v| v.get_value().get_num() ).expect("Time func not working");
                
                let arr_idx =  if let Some(bound) = max_bound {
                    (t_spawn % (*bound as i128)) as usize } 
                else { 0 };

                let his_val = match history.get_mut(arr_idx) {
                    Some(HistoryValue { value, spawn_point  }) => {
                        if *spawn_point == *t_spawn {
                            *value += res;
                        } else {
                            *value = res;
                            *spawn_point = *t_spawn;
                        }
                        *value
                    },
                    None => {
                        history.resize(arr_idx+1, (0_i128,-1_i128).into());
                        history[arr_idx] = (res, *t_spawn).into();
                        res
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
                    crate::monitor_setup::operation_types::LTL::Eventually(_) => {
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
