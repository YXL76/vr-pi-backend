use actix::prelude::*;

/// server 将此消息发送到 session
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

/// 创建了新的 session
#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub id: usize,
    pub addr: Recipient<Message>,
}

/// session 断开
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

/// 发送到客户端的消息
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub id: usize,
    pub msg: String,
}
