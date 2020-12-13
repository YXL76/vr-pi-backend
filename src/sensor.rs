use std::time::{Duration, Instant};

use actix::*;
use actix_web_actors::ws;

use super::server::{ClientMessage, Connect, Disconnect, Message, SensorServer};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(30);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(40);

pub struct SensorSession {
    pub id: usize,
    pub hb: Instant,
    pub addr: Addr<SensorServer>,
}

impl Actor for SensorSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();
        self.addr.do_send(Connect {
            id: self.id,
            addr: addr.recipient(),
        })
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.addr.do_send(Disconnect { id: self.id });
        Running::Stop
    }
}

impl Handler<Message> for SensorSession {
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for SensorSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                let msg = text.to_owned();
                self.addr.do_send(ClientMessage { id: self.id, msg })
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Continuation(_)) => {
                ctx.stop();
            }
            _ => (),
        }
    }
}

impl SensorSession {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                act.addr.do_send(Disconnect { id: act.id });
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
}
