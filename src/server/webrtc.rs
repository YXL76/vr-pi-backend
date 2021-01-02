use actix::prelude::*;
use std::collections::HashMap;

use super::message::{ClientMessage, Connect, Disconnect, Message};

/// 管理所有 session
pub struct WebrtcServer {
    sessions: HashMap<usize, Recipient<Message>>,
}

impl Default for WebrtcServer {
    fn default() -> WebrtcServer {
        WebrtcServer {
            sessions: HashMap::new(),
        }
    }
}

impl WebrtcServer {
    /// 向所有客户端发送消息
    fn send_message(&self, message: &str, skip_id: usize) {
        for (id, addr) in &self.sessions {
            if *id != skip_id {
                let _ = addr.do_send(Message(message.to_owned()));
            }
        }
    }
}

impl Actor for WebrtcServer {
    /// 默认上下文
    type Context = Context<Self>;
}

/// 处理新连接
impl Handler<Connect> for WebrtcServer {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) {
        self.sessions.insert(msg.id, msg.addr);
    }
}

/// 处理连接断开
impl Handler<Disconnect> for WebrtcServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        self.sessions.remove(&msg.id);
    }
}

/// 处理客户端消息
impl Handler<ClientMessage> for WebrtcServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        self.send_message(msg.msg.as_str(), msg.id);
    }
}
