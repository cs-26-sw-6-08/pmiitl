use std::{collections::HashMap, sync::{Arc, Mutex}};

use reqwest::Client;
use serde::Serialize;
use tokio::time::{Duration, interval};

pub struct DiscordLogger {
    webhook_url: String,
    client: Client,
    time_between: Duration,
    queue: Mutex<HashMap<usize, Vec<i128>>>,
}

#[derive(Serialize)]
struct DiscordPostPayload {
    embeds : Vec<DiscordEmbed>
}

#[derive(Serialize)]
struct DiscordEmbed {
    title: String,
    color: u128,
    fields: Vec<DiscordField>
}

#[derive(Serialize)]
struct DiscordField {
    name: String,
    value: String,
    inline: bool
}

impl DiscordLogger {
    pub fn new(webhook_url: String, time_between: Duration) -> Self {
        DiscordLogger {
            webhook_url,
            client: Client::new(),
            time_between,
            queue: Mutex::new(HashMap::new()),
        }
    }

    pub fn add_violation(&self, property: usize, time: i128) {
        let mut locked_map = self.queue.lock().unwrap();
        let entry = locked_map.entry(property).or_default();
        entry.push(time);
    }

    pub fn start_sending(self: Arc<Self>) {
        let logger = Arc::clone(&self);
        let mut interval = interval(logger.time_between);
        tokio::spawn(async move {
            loop {
                interval.tick().await;
                logger.send().await;
            }
        });
    }

    async fn send(&self) {
        let items = {
            let mut queue = self.queue.lock().unwrap();
            if queue.is_empty() {
                return;
            };
            let items = queue.clone();
            queue.clear();
            items
        };
        
        let fields = items.iter().map(|(property, time_steps)| {
            DiscordField{ name: format!("Property {}", property), value: time_steps.iter().map(i128::to_string).collect::<Vec<String>>().join(", "), inline: true }
        }).collect();

        let payload = DiscordPostPayload{ embeds: vec![DiscordEmbed{ title: "Property violations:".into(), color: 16215405, fields }] };

        let _ = self
            .client
            .post(&self.webhook_url)
            .json(&payload)
            .send()
            .await;
    }
}
