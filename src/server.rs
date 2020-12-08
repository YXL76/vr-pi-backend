use actix::prelude::*;
use rand::{self, rngs::ThreadRng, Rng};
use serde::{Deserialize, Serialize};
use serde_json::{json, Result};
use std::collections::HashMap;
use std::f64::consts::PI;

/// Chat server sends this messages to session
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

/// Message for chat server communications

/// New chat session is created
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>,
}

/// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

/// Send message to specific room
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    /// Id of the client session
    pub id: usize,
    /// Peer message
    pub msg: String,
}

/// `ChatServer` manages chat rooms and responsible for coordinating chat
/// session. implementation is super primitive
pub struct ChatServer {
    sessions: HashMap<usize, Recipient<Message>>,
    rng: ThreadRng,
}

impl Default for ChatServer {
    fn default() -> ChatServer {
        ChatServer {
            sessions: HashMap::new(),
            rng: rand::thread_rng(),
        }
    }
}

impl ChatServer {
    /// Send message to all users in the room
    fn send_message(&self, message: &str, skip_id: usize) {
        for (id, addr) in &self.sessions {
            if *id != skip_id {
                let _ = addr.do_send(Message(message.to_owned()));
            }
        }
    }
}

/// Make actor from `ChatServer`
impl Actor for ChatServer {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}

/// Handler for Connect message.
///
/// Register new session and assign unique id to this session
impl Handler<Connect> for ChatServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        // register session with random id
        let id = self.rng.gen::<usize>();
        self.sessions.insert(id, msg.addr);

        // send id back
        id
    }
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        // remove address
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

/// Handler for Message message.
impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        self.send_message(msg.msg.as_str(), msg.id);
        /* if let Ok(data) = data_processing(msg.msg.as_str()) {
            self.send_message(data.as_str(), msg.id)
        } */
    }
}
