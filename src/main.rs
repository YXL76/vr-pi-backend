use std::time::Instant;

use actix::*;
use actix_files as fs;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use rand::{thread_rng, Rng};

mod sensor;
mod server;
mod webrtc;

async fn webrtc_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::WebrtcServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        webrtc::WebrtcSession {
            id: thread_rng().gen::<usize>(),
            hb: Instant::now(),
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

async fn sensor_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::SensorServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        sensor::SensorSession {
            id: thread_rng().gen::<usize>(),
            hb: Instant::now(),
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let webrtc_server = server::WebrtcServer::default().start();
    let sensor_server = server::SensorServer::default().start();

    HttpServer::new(move || {
        App::new()
            .data(webrtc_server.clone())
            .data(sensor_server.clone())
            // redirect to index.html
            .service(web::resource("/").route(web::get().to(|| {
                HttpResponse::Found()
                    .header("LOCATION", "/static/index.html")
                    .finish()
            })))
            // websocket
            .service(web::resource("/webrtc/").to(webrtc_route))
            .service(web::resource("/sensor/").to(sensor_route))
            // static resources
            .service(fs::Files::new("/static/", "static/"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
