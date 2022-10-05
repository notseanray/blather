mod firebase;
mod git;
mod storage;

use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, http::StatusCode};
use actix_web_actors::ws;
use serde::Deserialize;
use envy::from_env;
// use chrono::Weekday;

// ping/keepalive
// jsonrecord
// binrecord

#[derive(Deserialize, Debug)]
struct Config {
    period: usize,
    max_folder_gb: f32,
    password: String,
}

lazy_static::lazy_static! {
    pub static ref CONFIG: Config = envy::from_env().unwrap();
}

// this is declared in chrono already, but we need to be able to deserialize it
#[derive(Deserialize, Debug, PartialEq)]
enum Weekday {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}

impl From<Weekday> for chrono::Weekday {
    fn from(s: Weekday) -> Self {
        type C = chrono::Weekday;
        type W = Weekday;
        match s {
            W::Mon => C::Mon,
            W::Tue => C::Tue,
            W::Wed => C::Wed,
            W::Thu => C::Thu,
            W::Fri => C::Fri,
            W::Sat => C::Sat,
            _ => unimplemented!(),
        }
    }
}



struct WsMsg;

impl Actor for WsMsg {
    type Context = ws::WebsocketContext<Self>;
}

fn msg_dispatch(value: String) -> String {
    if value.is_empty() {
        return String::from("Empty message not allowed");
    }
    let args: Vec<&str> = value.split_whitespace().collect();
    match args[0] {
        "URL" => {
            // return latest week
            if args.len() == 1 {
                unimplemented!();
            }
            let week: u8 = match args[1].parse() {
                Ok(v) => v,
                _ => return String::from("invalid week given"),
            };
        },
        "JSONDATA" => {
            // return requested json data
        },
        "JSONRECORD" => {
            // return a short record of json backups
        },
        "BINRECORD" => {
            // return a short record of bin dumps
        },
        _ => {},
    }
    unimplemented!();
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsMsg {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            _ => {},
        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(WsMsg {}, &req, stream);
    println!("{:?}", resp);
    resp
}

async fn shutdown(req: HttpRequest) -> Result<HttpResponse, Error> {
    if let Some(v) = req.headers().get("key") {
        if CONFIG.password.len() == msg.len() {
            let mut result = 0;
            for (x, y) in CONFIG.password.chars().zip(v.chars()) {
                result |= x as u32 ^ y as u32;
            }
            result == 0
        } else {
            false
        }
    }
    Ok(HttpResponse::new(StatusCode::OK))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    HttpServer::new(|| {
            App::new()
                .route("/ws/", web::get().to(index))
                .route("/shutdown/", web::get().to(shutdown))
        }).bind(("127.0.0.1", 7930))?
        .run()
        .await
}
