
use crate::{errors, monitor::{streams::{IoTDevice, IoTStream, OutputStream}, types::{StackContent, StackValue, Verdict}}, monitor_setup::operation_types::{AggregateType, HistoryValue, LTL, Operation}, program::{member_types::MemberType, operations::BinaryOperators}, utils::vec_helper_funcs::ExtVec};
use std::{error::Error};

impl OutputStream {
    // Calculate the verdict for the output stream.
    pub fn update(&mut self, t_current: i128, devices: &IoTStream) -> Result<(), Box<dyn Error>> {
        if self.ltl == LTL::Eventually(true) {
            return Ok(());
        }
        for (t_spawn, ver) in self.time_verdicts.iter_mut() {
            let res = eval_operations(&mut self.operations, devices, &*t_spawn, &t_current);
            
            match &mut self.ltl {
                LTL::Always => {
                    let res = res?;
                    let res_val = res.get_value().get_verdict().unwrap();
                    //Set verdict
                    if !res_val {
                        *ver = Verdict::False;
                    } else if res.is_decided() {
                        *ver = Verdict::True;
                    }
                },
                LTL::Eventually(_) => {                  
                    let res = res?.get_value().get_verdict().unwrap();
                    let within_bounds = self.bound.is_some_and(|(_, b)| b <= t_current*1_000);
                    if res {
                     //   #[cfg(debug_assertions)] 
                       // println!("{}", "\t--- Removed a property ---".yellow().bold().italic().underline());
                        self.ltl = LTL::Eventually(true);
                        *ver = Verdict::True;
                    } else if within_bounds {
                      //  #[cfg(debug_assertions)] 
                    //    println!("{}", "\t--- Removed a property ---".yellow().bold().italic().underline());
                        self.ltl = LTL::Eventually(true);
                        *ver = Verdict::False;
                    } else if !res {
                        *ver = Verdict::False; 
                    }
                },
            }
        }
        Ok(())
    }
}

#[derive(PartialEq, Debug)]
enum StepType { Deepen, Reduce, ReducePartial }
//enum { bottom, look, }

pub(crate) fn eval_operations<'a>(
    operations: &mut [Operation], 
    devices: &'a IoTStream,
    t_spawn: &i128,
    t_current: &i128,
) -> Result<StackValue<'a>, Box<dyn Error>> {
    use StepType::*;

    let mut worklist_stack: Vec<(usize, StepType)> = Vec::with_capacity(50);
    let mut value_stack: Vec<StackValue> = Vec::with_capacity(50);
    let mut device_stack: Vec<&IoTDevice> = Vec::with_capacity(50);
    let mut device_pointer: Option<&IoTDevice> = None;

    worklist_stack.push((0usize, StepType::Deepen));
    
    while let Some((cur_idx, step_type)) = worklist_stack.pop() {
        // let cur_op = &mut operations[cur_idx] as *mut Operation;
        let cur_op = &mut operations[cur_idx] as *mut Operation;

        match  (unsafe { &mut*cur_op }, step_type)  { 
            //todo: Sørg for at arithmetic operations er korrekte -> E.g. 1 * 1 = 1 and not 1000
            // Base cases
            (Operation::Number(val), _) => value_stack.push((*val).into()),
            (Operation::String(str), _) => value_stack.push((&*str).into()),
            (Operation::CurrentTime, _) => value_stack.push((*t_spawn * 1_000).into()),
            (Operation::Member(mem_type), _) => {
                value_stack.push(match mem_type {
                    //todo: Remove active from membertype
                    MemberType::Active => todo!(),
                    MemberType::Power =>  device_pointer.ok_or(errors::Error::DevicePointer)?.power.into(),
                    MemberType::Name =>  StackValue::from(device_pointer.map(|d| &d.name).ok_or(errors::Error::DevicePointer)?),
                });
            },
            // BinOp / UnOp
            (Operation::Binary { idx_lhs,.. }, Deepen) => {
                worklist_stack.extend([
                    (cur_idx, ReducePartial),
                    (*idx_lhs, Deepen)
                ]);
            },
            (Operation::Binary { bin_op, idx_rhs, .. }, ReducePartial) => {
                //If the binary operation is an 'or' and returned true, then the rest shouldn't be evaluated
                // Read as: 'or' -> last_val.is_false
                if !matches!(bin_op, BinaryOperators::Or) 
                || !value_stack.last().is_some_and(|val| matches!(*val.get_value(), StackContent::Verdict(true))) {
                    worklist_stack.extend([(cur_idx, Reduce), (*idx_rhs, Deepen)]);
                }
            },
            (Operation::Binary { bin_op, .. }, Reduce) => {
                let v_rhs = value_stack.pop_or_err()?;
                let v_lhs = value_stack.pop_or_err()?;
                value_stack.push( v_lhs.bin_op(v_rhs, bin_op) );
            },
            (Operation::Unary { idx , ..}, Deepen) => { 
                worklist_stack.extend([(cur_idx, Reduce),(*idx, Deepen)]); 
            },
            (Operation::Unary { un_op, .. }, Reduce) => {
                let res = value_stack.pop_or_err()?.un_op(un_op);
                value_stack.push(res);
            },

            // Aggregate Functions
            (Operation::AggregateFunction { idx, .. }, Deepen) => {
                worklist_stack.extend([
                    (cur_idx, ReducePartial),
                    (*idx, Deepen),
                ]);
                
                //Put devices on device stack and pop the first
                device_stack.extend(devices.get_devices());
                device_pointer = device_stack.pop();

                //Accumulation starts at zero
                value_stack.push( 0.into() );
            }
            (Operation::AggregateFunction { idx, ..}, ReducePartial) => {
                //Pop the accumulated value and newest value on the stack and add them
                let res  = value_stack.pop_or_err()? + value_stack.pop_or_err()?;
                value_stack.push( res );

                if let Some(device) = device_stack.pop() {
                    device_pointer = Some(device);
                    worklist_stack.extend([
                        (cur_idx, ReducePartial),
                        (*idx, Deepen),
                    ]);
                } else { worklist_stack.push((cur_idx, Reduce)); }
            }
            (Operation::AggregateFunction { function_type, .. }, Reduce) => {
                let res  = value_stack.pop_or_err()?;
                value_stack.push(
                    match function_type {
                        AggregateType::Sum => res,
                        AggregateType::Avg => res / (devices.get_devices().len() as i128).into(),
                    }
                );
            },
            (Operation::Foreach { .. }, Deepen) => {
                worklist_stack.push((cur_idx, Reduce));
                device_stack.extend(devices.get_devices());
                value_stack.push( true.into() )
            },
            (Operation::Foreach { idx }, Reduce) => {
                //Violation didn't occur and not all devices have been looked at
                //todo: Figure out if undecided should be here as well
                if value_stack.last().is_some_and(|v| matches!(*v.get_value(), StackContent::Verdict(true)))
                && !device_stack.is_empty() {
                    let _ = value_stack.pop();
                    device_pointer = device_stack.pop();
                    worklist_stack.extend([(cur_idx, Reduce), (*idx, Deepen)]);
                
                //If here, then a violation occured or not depending on the last value in value_stack
                } else { //todo: Fix --> Ogs fix for nestede aggregate functions 
                    device_stack.clear(); 
                }
            },
            // Time functions
            (Operation::TimeFunction { idx, bound, history, function_type }, Deepen) => {
                //If bound has already been exceeded we aren't interested in calculating further
                //todo: Can be optimized by adding this check when the bound is just about to be overstepped
                match bound { 
                    //The difference between t_c and t_s is the time the bound has been active. 
                    //If it exceeds the end (b) (added 1 because of it the num being inclusive), then it shouldn't evaluate the expression and it is decided (or untainted) 
                    Some((a, b)) if (*t_current - *t_spawn) == *b + 1 => {
                        let prev_val = history[(t_spawn % (*b - *a + 1)) as usize].value;
                        value_stack.push(
                            function_type_computation(function_type, prev_val, *t_spawn, *t_current - 1).into()
                        );
                    },
                    _ => worklist_stack.extend([(cur_idx, Reduce), (*idx, Deepen)])
                }                
            },
            (Operation::TimeFunction { function_type, history, bound, .. }, Reduce) => {
                let val = value_stack.pop_or_err()?.get_value().get_num()?;
                let val = time_function_reduce_step(val, *t_spawn, *bound, history);
                let val: StackValue = function_type_computation(function_type, val, *t_spawn, *t_current).into();
                value_stack.push(val.to_undecided());
            },
            
            // LTL 
            (Operation::LTLAlwaysUnbounded { idx }, Deepen) => {
                worklist_stack.extend([(cur_idx, Reduce), (*idx, Deepen)]);
            },
            (Operation::LTLAlwaysUnbounded { .. }, Reduce) => {
                let val = value_stack.pop_or_err()?;
                value_stack.push(
                    val.to_undecided()
                );
            },
            //todo: Write fucking test-cases. this shit is not helping anyone
            (Operation::LTLBounded { idx, bound, ltl_type, .. }, Deepen) => {
                let (a,b) = bound;
                //If over bound, should add verdict to stack and move back up
                //fst is lowerbound, snd is upperbound
                match (*a+*t_spawn <= *t_current, *t_current <= *t_spawn + *b ) {
                    (true, true) => {
                        worklist_stack.extend([(cur_idx, Reduce), (*idx, Deepen)]);
                    },
                    (true, false) => value_stack.push(
                        match ltl_type {
                            LTL::Always | LTL::Eventually(true) => true.into(),
                            LTL::Eventually(false) => false.into(),
                        }
                    ),
                    _ => ()
                }
            },
            (Operation::LTLBounded { not, .. }, Reduce) => {
                let val = value_stack.pop_or_err()?;
                //Undecideable when here -> As the bound haven't been reached yet
                let val = val.to_undecided();
                //Not the value if necessary
                let val = if *not { !val } else { val };
                value_stack.push(val);
            },
            _ => Err(errors::Error::IllegalOperation)?
        }
    }
    value_stack.pop_or_err()
}

#[inline]
fn function_type_computation(
    function_type: &AggregateType, 
    cur_val: i128, 
    t_spawn: i128, 
    t_current: i128
) -> i128 {
    match function_type {
        AggregateType::Sum => cur_val,
        AggregateType::Avg => cur_val / (t_current - t_spawn),
    }
}

///Warning: This function has side effects
#[inline]
fn time_function_reduce_step(
    newest_val: i128,
    t_spawn: i128,
    bound: Option<(i128, i128)>,
    history_vec: &mut Vec<HistoryValue>
) -> i128 {

    //Which idx should be overwritten
    let arr_idx =  if let Some((a,b)) = bound {
            (t_spawn % (b-a + 1)) as usize } 
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