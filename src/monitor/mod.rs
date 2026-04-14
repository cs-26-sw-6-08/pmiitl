pub mod streams;
pub mod types;

#[cfg(test)]
mod streams_test;



use std::error::Error;

use crate::program::Program;

use tokio::time::{Duration, interval};

impl Program {

    pub async fn monitor(&mut self, time_interval: i128) -> Result<(), Box<dyn Error>> {
        println!("Monitor has started...");
        
        let Some(streams) = &mut self.environment else { todo!() };
        let mut interval = interval(Duration::from_secs(time_interval as u64));
        let mut t = 0;
        
        loop {
            interval.tick().await;

            async {
                //todo: await devices

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

                    (prop_num, is_violated)
                }).filter(|(_, v)| *v);

                for (prop_num, _) in violated_verdicts {
                    println!("Prop {} was violated at time: {t}", prop_num+1);
                }
                
                t += time_interval;
            }.await
        }
    }

}