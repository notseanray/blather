mod firebase;
mod git;
mod storage;

use crate::firebase::Firebase;
use actix::{spawn, Actor, Context, Handler, Message, MessageResult, Recipient, StreamHandler};
use actix_cors::Cors;
use actix_web::{http::StatusCode, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use firebase::RegistrationData;
use git::CommitPoint;
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use std::{
    collections::HashMap,
    process,
    sync::atomic::AtomicU64,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use storage::{BinSet, Storage};

macro_rules! validate_pass {
    ($req:expr) => {
        // this would be much cleaner if we had if let chaining, soon tm
        if let Some(v) = $req.headers().get("key") {
            if CONFIG.password.len() == v.len() {
                let mut result = 0;
                for (x, y) in CONFIG
                    .password
                    .chars()
                    .zip(v.to_str().unwrap_or_default().chars())
                {
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
    pub(crate) static ref STORAGE: Storage = Storage::new(CONFIG.max_folder_gb).expect("failed to init bin storage");
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

struct UrlMsg {
    week: Option<u32>,
}

impl Message for UrlMsg {
    type Result = String;
}

struct JsonLatest;

impl Message for JsonLatest {
    type Result = String;
}

struct JsonRecord;

impl Message for JsonRecord {
    type Result = Vec<CommitPoint>;
}

struct JsonData;

impl Message for JsonData {
    type Result = Vec<RegistrationData>;
}

impl Handler<JsonData> for ConnectedClients {
    type Result = MessageResult<JsonData>;
    fn handle(&mut self, msg: JsonData, ctx: &mut Self::Context) -> Self::Result {
        // MessageResponse()
        unimplemented!();
    }
}

struct BinRecord;

impl Message for BinRecord {
    type Result = Vec<BinSet>;
}

impl Handler<BinRecord> for ConnectedClients {
    type Result = MessageResult<BinRecord>;
    fn handle(&mut self, _: BinRecord, _: &mut Context<Self>) -> Self::Result {
        MessageResult(STORAGE.dump().unwrap_or_default())
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct MessageResponse(pub String);

#[derive(Default)]
struct ConnectedClients {
    sessions: HashMap<usize, Recipient<MessageResponse>>,
}

impl Actor for ConnectedClients {
    type Context = Context<Self>;
}

struct WsMsg {
    pub hb: Instant,
    pub req: AtomicU64,
    pub name: String,
}

impl Default for WsMsg {
    fn default() -> Self {
        Self {
            hb: Instant::now(),
            req: AtomicU64::from(0),
            name: String::default(),
        }
    }
}

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
            if args.len() == 2 {
                return String::from("test");
            }
            let week: u8 = match args[2].parse() {
                Ok(v) => v,
                _ => return String::from("INVALID week given"),
            };
        }
        "JSONLATEST" => {}
        "JSONDATA" => {
            // return requested json data
        }
        "JSONRECORD" => {
            // return a short record of json backups
        }
        "BINRECORD" => {
            // return a short record of bin dumps
        }
        _ => {}
    }
    String::from("INVALID request")
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsMsg {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Ok(v) => v,
            _ => return,
        };
        match msg {
            ws::Message::Ping(_) => ctx.pong(
                &SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("clock went backward")
                    .as_millis()
                    .to_le_bytes(),
            ),
            ws::Message::Text(text) => ctx.text(msg_dispatch(text.to_string())),
            _ => {}
        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(WsMsg::default(), &req, stream)
}

async fn shutdown(req: HttpRequest) -> Result<HttpResponse, Error> {
    if validate_pass!(req) {
        process::exit(0);
    }
    Ok(HttpResponse::new(StatusCode::UNAUTHORIZED))
}

async fn new_json_backup(req: HttpRequest) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::new(StatusCode::UNAUTHORIZED))
}

async fn new_bin_backup(req: HttpRequest) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::new(StatusCode::UNAUTHORIZED))
}

async fn refresh_session(req: HttpRequest) -> Result<HttpResponse, Error> {
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
            .route("/refresh/session", web::get().to(refresh_session))
            .route("/new/jsonbackup", web::get().to(new_json_backup))
            .route("/new/binbackup", web::get().to(new_bin_backup))
    })
    .bind(("127.0.0.1", 7930))?
    .run()
    .await
}
