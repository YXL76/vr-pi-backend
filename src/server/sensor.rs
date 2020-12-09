use std::{collections::HashMap, f64::consts::PI};

use actix::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Result};

use super::message::{ClientMessage, Connect, Disconnect, Message};

pub struct SensorServer {
    sessions: HashMap<usize, Recipient<Message>>,
}

impl Default for SensorServer {
    fn default() -> SensorServer {
        SensorServer {
            sessions: HashMap::new(),
        }
    }
}

impl SensorServer {
    fn send_message(&self, message: &str, skip_id: usize) {
        for (id, addr) in &self.sessions {
            if *id != skip_id {
                let _ = addr.do_send(Message(message.to_owned()));
            }
        }
    }
}

impl Actor for SensorServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for SensorServer {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) {
        self.sessions.insert(msg.id, msg.addr);
    }
}

impl Handler<Disconnect> for SensorServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        self.sessions.remove(&msg.id);
    }
}

#[derive(Serialize, Deserialize)]
struct Rotation {
    alpha: f64,
    beta: f64,
    gamma: f64,
}

fn data_processing(data: &str) -> Result<String> {
    let a = 5.71559214e-05; // 与舵机相匹配
    let b = 3.60082305e-02; // 与舵机相匹配
    let c = 2.50000000;
    // let direction = 90.0;
    let rotation: Rotation = serde_json::from_str(data)?;

    let vertical_direction = rotation.gamma * -180.0 / PI;
    // let vertical_direction = direction - (rotation.gamma + PI / 2.0) / (PI / 2.0) * 90.0;
    let level_direction = rotation.alpha * -180.0 / PI;
    // let level_direction = direction - (rotation.alpha + PI / 2.0) / (PI / 2.0) * 90.0;
    let level_direction = 180.0 - level_direction;

    let vertical_duty = a * vertical_direction * vertical_direction + b * vertical_direction + c;
    let level_duty = a * level_direction * level_direction + b * level_direction + c;

    let output = json!({
        "verticalDuty": vertical_duty,
        "levelDuty": level_duty,
    });
    Ok(output.to_string())
}

impl Handler<ClientMessage> for SensorServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        if let Ok(data) = data_processing(msg.msg.as_str()) {
            self.send_message(data.as_str(), msg.id);
        }
    }
}
