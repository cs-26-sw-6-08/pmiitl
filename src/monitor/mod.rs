pub mod streams;
pub mod types;

#[cfg(test)]
mod streams_test;

use std::error::Error;
use crate::{monitor::streams::{IoTDevice, IoTStream}, program::Program};
use tokio::time::{Duration, interval};
use std::time::Instant;

use std::{thread, time};

use colored::*;

impl Program {

    pub async fn monitor(&mut self, time_interval: i128) -> Result<(), Box<dyn Error>> {
        println!("Monitor has started...");
        
        let Some(streams) = &mut self.environment else { todo!() }; //Overvej custom error
        let mut interval = interval(Duration::from_secs(time_interval as u64));
        let mut t = 0;
        
        let temp_IoT_stream = IoTStream::from(vec![IoTDevice::from((String::from("Roomba"), 5, true))]);

        loop {
            interval.tick().await;
            let start = Instant::now();
            
            //todo: await devices

            async {

                let violated_verdicts = streams
                .iter_mut()
                .enumerate()
                .map(|(prop_num, stream)| {
                    // SDI update
                    stream.insert(t); 

                    // Calculate the new state of the streams
                    stream.update(); 

                    // Give verdicts
                    let is_violated = stream.get_violated_verdict_single(); 
                    
                    stream.clean_up();

                    println!("{:#?}",stream);

                    (prop_num, is_violated)
                }).filter(|(_, v)| *v);

                for (prop_num, _) in violated_verdicts {
                    println!("Prop {} was violated at time: {t}", prop_num+1);
                }

                
                t += time_interval;
            }.await;

            let elapsed = start.elapsed();
            let colored_time = match elapsed.as_secs() > time_interval as u64 {
                true => format!("{:?}",elapsed).red(),
                false => format!("{:?}",elapsed).green(),
            };
            let colored_time = (elapsed.as_secs() > time_interval as u64 ).then_some(format!("{:?}",elapsed).red()).or_else(format!("{:?}",elapsed).green());
            println!("Yarjis translate to engliesh det tog så lang tid at regne for {} her er tiden {}", t, colored_time);
        }
    }
}