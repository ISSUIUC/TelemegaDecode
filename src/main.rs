#[macro_use] extern crate rocket;

use rocket::fs::{FileServer, relative};
use rocket::get;
use std::sync::Mutex;
use webbrowser;
use std::thread;

mod gfsk;
mod packet_types;

use crate::gfsk::packet::Packet;
use crate::packet_types::{decode, DecodedPacket};

static mut QUEUE: Mutex<Vec<DecodedPacket>> = Mutex::new(Vec::new());

#[get("/getdata")]
fn data() -> String {
    let queued = unsafe { QUEUE.get_mut() };
    if let Ok(queued) = queued {
        let json = serde_json::to_string(&queued).unwrap();
        queued.clear();
        println!("{json}");
        json
    } else {
        eprintln!("Failed to dequeue packet");
        "[]".into()
    }
}

#[launch]
fn rocket() -> _ {
    let decoder_thread = thread::spawn(||{
        gfsk::start_decoders(|freq, packet|{
            // println!("{freq},{packet:?}");
            println!("{}", packet.crc_match);
            if let Ok(queued) = unsafe { QUEUE.get_mut() } {
                queued.push(decode(&packet).unwrap());
            } else {
                eprintln!("Failed to queue packet");
            }
        });
    });
    let server = rocket::build()
        .mount("/", routes![data])
        .mount("/", FileServer::from(relative!("GUI/public")));

    // if webbrowser::open("http://127.0.0.1:8000").is_err() {
    //     println!("Failed to open webpage");
    // }
    server
}