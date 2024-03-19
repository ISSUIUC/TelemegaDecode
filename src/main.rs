#[macro_use] extern crate rocket;

use std::path::Path;
use rocket::fs::{FileServer, relative};
use rocket::get;
use std::sync::{LockResult, Mutex};
use webbrowser;
use std::thread;
mod gfsk;
mod packet_types;

use crate::gfsk::packet::Packet;

static mut QUEUE: Mutex<Vec<Packet>> = Mutex::new(Vec::new());

#[get("/data")]
fn data() -> String {
    let queued = unsafe { QUEUE.get_mut() };
    "hello".into()
}

#[launch]
fn rocket() -> _ {
    let decoder_thread = thread::spawn(||{
        gfsk::start_decoders(|freq, packet|{
            println!("{freq},{packet:?}");
        });
    });
    let server = rocket::build()
        .mount("/", routes![data])
        .mount("/", FileServer::from(relative!("GUI/public")));

    if webbrowser::open("http://127.0.0.1:8000").is_err() {
        println!("Failed to open webpage");
    }
    server
}