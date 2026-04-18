

use crate::{errors, monitor::{streams::{IoTDevice, IoTStream, OutputStream}, types::{DerivedOutput, StackValue, Verdict}}, monitor_setup::operation_types::{AggregateType, HistoryValue, LTL, Operation}, program::member_types::MemberType, utils::trait_helper_funcs::OptionExt};

use std::error::Error;

impl OutputStream {
    // Calculate the verdict for the output stream.
    pub fn update(&mut self, t_current: i128, devices: &IoTStream) -> Result<(), Box<dyn Error>> {

        for (t_spawn, ver) in self.time_verdicts.iter_mut() {
            let res = eval_operations(&mut self.operations, devices, &*t_spawn, &t_current);
            
            match self.ltl {
                LTL::Always => {
                    let res = res?;
                    let res_val = res.get_value().get_verdict().unwrap();

                    //Set verdict
                    if res_val == Verdict::False {
                        *ver = Verdict::False;
                    } else if res.is_decided() {
                        *ver = res_val;
                    }
                },
                LTL::Eventually(_) => todo!(),
            }
        }
        Ok(())
    }
}

#[derive(PartialEq, Debug)]
enum StepType { Deepen, Reduce }

pub(crate) fn eval_operations<'a>(
    operations: &mut [Operation], 
    devices: &'a IoTStream,
    t_spawn: &i128,
    t_current: &i128,
) -> Result<StackValue<'a>, Box<dyn Error>> {
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
            (Operation::CurrentTime, _) => value_stack.push((*t_spawn * 1_000).into()),
            (Operation::Member(mem_type), _) => {
                value_stack.push(match mem_type {
                    MemberType::Active => device_pointer.ok_or(errors::Error::DevicePointerError)?.active.into(),
                    MemberType::Power =>  device_pointer.ok_or(errors::Error::DevicePointerError)?.power.into(),
                    MemberType::Name =>  StackValue::from(device_pointer.map(|d| &d.name).ok_or(errors::Error::DevicePointerError)?),
                });
            },

            //todo: If we want optimize add case where or-case for fst value is true
            // BinOp / UnOp
            (Operation::Binary { idx_lhs, idx_rhs,.. }, Deepen) => {
                idx_stack.extend([
                    (cur_idx, Reduce),
                    (*idx_rhs, Deepen),
                    (*idx_lhs, Deepen)
                ]);
            },
            (Operation::Binary { bin_op, .. }, Reduce) => {
                let v1 = value_stack.pop().or_pop_err()?;
                let v2 = value_stack.pop().or_pop_err()?;

                value_stack.push( v2.bin_op(v1, bin_op) );
            },
            (Operation::Unary { idx , ..}, Deepen) => { 
                idx_stack.extend([(cur_idx, Reduce),(*idx, Deepen)]); 
            },
            (Operation::Unary { un_op, .. }, Reduce) => {
                let res = value_stack.pop().or_pop_err()?.un_op(un_op);
                value_stack.push(res);
            },

            // Aggregate Functions
            (Operation::AggregateFunction { .. }, Deepen) => {
                idx_stack.push((cur_idx, Reduce));
                device_stack.extend(devices.get_devices());
                value_stack.extend( [0.into(),0.into()] );
            }
            (Operation::AggregateFunction { function_type, idx }, Reduce) => {
                let val = value_stack.pop().or_pop_err()?;
                if !device_stack.is_empty() {
                    let acc = value_stack.pop().or_pop_err()?;
                    value_stack.push( acc + val );
                    device_pointer = device_stack.pop();
                    idx_stack.extend([
                        (cur_idx, Reduce),
                        (*idx, Deepen),
                    ]);
                } else {
                    let acc = value_stack.pop().or_pop_err()?;
                    //let res = acc.zip(res).map(|(a, b)| a + b).expect("Error in Aggregate function");
                    let res = acc + val;
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
                    idx_stack.extend([(cur_idx, Reduce), (*idx, Deepen)]);

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
                let val = value_stack.pop().or_pop_err()?.get_value().get_num()?;
                
                /*//If bound has already been exceeded we aren't interested in calculating further
                if let Some(bound) = max_bound && (*t_current - *t_spawn) == (*bound) as i128 {
                    let his_val = history[(t_spawn % (*bound as i128)) as usize].value;
                    value_stack.push(
                       compute_function_type(function_type, his_val, *t_spawn, *t_current).into()
                    );
                    continue;
                }*/

                let val = time_function_reduce_step(val, *t_spawn, *max_bound, history);
                let val: StackValue = compute_function_type(function_type, val, *t_spawn, *t_current).into();
                value_stack.push(val.to_undecided());
            },
            
            // LTL 
            (Operation::LTLAlwaysUnbounded { idx }, Deepen) => {
                idx_stack.extend([(cur_idx, Reduce), (*idx, Deepen)]);
            },
            (Operation::LTLAlwaysUnbounded { .. }, Reduce) => {
                let val = value_stack.pop().or_pop_err()?;
                value_stack.push(
                    val.and(Verdict::Undecided.into())
                );
            },
            //todo: Write fucking test-cases. this shit is not helping anyone
            (Operation::LTLBounded { idx, bound, ltl_type, .. }, Deepen) => {
                let (a,b) = bound;
                //If over bound, should add verdict to stack and move back up
                if *t_spawn + *b < *t_current { 
                    value_stack.push(
                        match ltl_type {
                            LTL::Always | LTL::Eventually(true) => Verdict::True.into(),
                            LTL::Eventually(false) => Verdict::False.into(),
                        }
                    ); 
                }
                else if *a+*t_spawn <= *t_current {
                    idx_stack.extend([(cur_idx, Reduce), (*idx, Deepen)]);
                }
            },
            (Operation::LTLBounded { bound, not, ltl_type, .. }, Reduce) => {
                let (_, b) = bound;
                let val = value_stack.pop().or_pop_err()?;
                //Check whether it is decideable or not
                let val = match (ltl_type, *t_current < *t_spawn + *b) {
                    (LTL::Always, true) => val.and(Verdict::Undecided.into()),
                    (LTL::Eventually(_), true) => val.or(Verdict::Undecided.into()),
                    _ => val
                };
                //Not the value if necessary
                let val = if *not { !val } else { val };
                value_stack.push(val);
            },
        }
    }
    value_stack.pop().or_pop_err()
}

 
fn compute_function_type(
    function_type: &AggregateType, 
    cur_val: i128, 
    t_spawn: i128, 
    t_current: i128
) -> i128 {
    match function_type {
        AggregateType::Sum => cur_val,
        AggregateType::Avg => cur_val/ t_current - t_spawn,
    }
}

///Warning: This function has side effects
#[inline]
fn time_function_reduce_step(
    newest_val: i128,
    t_spawn: i128,
    bound: Option<usize>,
    history_vec: &mut Vec<HistoryValue>
) -> i128 {

    //Which idx should be overwritten
    let arr_idx =  if let Some(bound) = bound {
            (t_spawn % (bound as i128)) as usize } 
        else { t_spawn as usize };

    //Sum up the value according to the history and update history accordingly
    match history_vec.get_mut(arr_idx) {
        Some(HistoryValue { value, spawn_point  }) => {
            if *spawn_point == t_spawn {
                *value += newest_val; 
            } else {
                *value = newest_val;
                *spawn_point = t_spawn;
            }
            *value
        },
        None => {
            history_vec.resize(arr_idx+1, (0_i128,-1_i128).into());
            history_vec[arr_idx] = (newest_val, t_spawn).into();
            newest_val
        },
    }
}