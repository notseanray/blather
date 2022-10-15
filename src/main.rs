mod firebase;
mod git;
mod storage;

use std::{process, time::Duration};
use crate::firebase::Firebase;

use actix::{Actor, StreamHandler, spawn};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, http::StatusCode};
use actix_web_actors::ws;
use actix_cors::Cors;
use std::sync::{Mutex, Arc};
use serde::Deserialize;

macro_rules! validate_pass {
    ($req:expr) => {
        // this would be much cleaner if we had if let chaining, soon tm
        if let Some(v) = $req.headers().get("key") {
            if CONFIG.password.len() == v.len() {
                let mut result = 0;
                for (x, y) in CONFIG.password.chars().zip(v.to_str().unwrap_or_default().chars()) {
                    result |= x as u32 ^ y as u32;
                }
                result == 0
            } else {
                false
            }
        } else {
            false
        }
    };
}
// use chrono::Weekday;

// ping/keepalive
// jsonrecord
// binrecord

#[derive(Deserialize, Debug)]
pub struct Config {
    pub period_cycle: u32,
    pub max_folder_gb: f32,
    pub password: String,
}

lazy_static::lazy_static! {
    pub(crate) static ref CONFIG: Config = envy::from_env::<Config>().unwrap();
    pub(crate) static ref FIREBASE: Arc<Mutex<Firebase>> = Arc::new(Mutex::new(Firebase::new("./cred.json").expect("No credentials found at that path")));
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

// there are def better ways to do this but i am lazy
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
            W::Sun => C::Sun,
        }
    }
}

struct WsMsg;

impl Actor for WsMsg {
    type Context = ws::WebsocketContext<Self>;
}

fn msg_dispatch(value: String) -> String {
    if value.is_empty() || value.len() < CONFIG.password.len() {
        return String::from("INVALID Empty message not allowed");
    }
    let args: Vec<&str> = value.split_whitespace().collect();
    if args.len() < 2 {
        return String::from("Expected at least two arguments");
    }
    let authed = if CONFIG.password.len() == args[0].len() {
        let mut result = 0;
        for (x, y) in CONFIG.password.chars().zip(args[0].chars()) {
            result |= x as u32 ^ y as u32;
        }
        result == 0
    } else {
        false
    };
    if !authed {
        return String::from("Invalid Password");
    }
    match args[1] {
        "URL" => {
            // return latest week
            if args.len() == 1 {
                unimplemented!();
            }
            let week: u8 = match args[1].parse() {
                Ok(v) => v,
                _ => return String::from("INVALID week given"),
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
    String::from("INVALID request")
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsMsg {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(msg_dispatch(text.to_string())),
            _ => {},
        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(WsMsg {}, &req, stream)
}

async fn shutdown(req: HttpRequest) -> Result<HttpResponse, Error> {
    if validate_pass!(req) {
        process::exit(0);
    }
    Ok(HttpResponse::new(StatusCode::UNAUTHORIZED))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    spawn(async move {
        loop {
            std::thread::sleep(Duration::from_secs(CONFIG.period_cycle.into()));
        }
    });
    HttpServer::new(|| {
            let cors = Cors::permissive();
            App::new()
                .wrap(cors)
                .route("/ws", web::get().to(index))
                .route("/shutdown", web::get().to(shutdown))
        }).bind(("127.0.0.1", 7930))?
        .run()
        .await
}
