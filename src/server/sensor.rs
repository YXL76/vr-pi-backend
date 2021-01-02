use std::collections::HashMap;

use actix::prelude::*;

use super::message::{ClientMessage, Connect, Disconnect, Message};

/// 管理所有 session
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
    /// 向所有客户端发送消息
    fn send_message(&self, message: &str, skip_id: usize) {
        for (id, addr) in &self.sessions {
            if *id != skip_id {
                let _ = addr.do_send(Message(message.to_owned()));
            }
        }
    }
}

impl Actor for SensorServer {
    /// 默认上下文
    type Context = Context<Self>;
}

/// 处理新连接
impl Handler<Connect> for SensorServer {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) {
        self.sessions.insert(msg.id, msg.addr);
    }
}

/// 处理连接断开
impl Handler<Disconnect> for SensorServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        self.sessions.remove(&msg.id);
    }
}

/// 处理客户端消息
impl Handler<ClientMessage> for SensorServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        self.send_message(msg.msg.as_str(), msg.id);
    }
}
