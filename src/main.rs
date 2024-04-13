// #[macro_use] extern crate rocket;
//
// use rocket::fs::{FileServer, relative};
// use rocket::get;
// use std::sync::Mutex;
// use std::thread;
// use telemega::DecodedPacket;
// use lora::hi;
//
// static mut QUEUE: Mutex<Vec<DecodedPacket>> = Mutex::new(Vec::new());
//
// #[get("/getdata")]
// fn data() -> String {
//     let queued = unsafe { QUEUE.get_mut() };
//     if let Ok(queued) = queued {
//         let json = serde_json::to_string(&queued).unwrap();
//         queued.clear();
//         println!("{json}");
//         json
//     } else {
//         eprintln!("Failed to dequeue packet");
//         "[]".into()
//     }
// }
//
// #[launch]
// fn rocket() -> _ {
//     let decoder_thread = thread::spawn(||{
//         telemega::start_decoders(|_freq, packet| {
//             // println!("{freq},{packet:?}");
//             println!("{}", packet.crc_match());
//             if let Ok(queued) = unsafe { QUEUE.get_mut() } {
//                 queued.push(packet);
//             } else {
//                 eprintln!("Failed to queue packet");
//             }
//         });
//     });
//     let server = rocket::build()
//         .mount("/", routes![data])
//         .mount("/", FileServer::from(relative!("GUI/public")));
//
//     // if webbrowser::open("http://127.0.0.1:8000").is_err() {
//     //     println!("Failed to open webpage");
//     // }
//     server
// }

use lora::{decode, demod};
fn main() {
    decode();
}