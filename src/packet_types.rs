use byteorder::{LittleEndian, ReadBytesExt};
use rocket::tokio::io::AsyncReadExt;
use crate::Packet;
use serde::Serialize;
use byteorde::LittleEndian;
#[derive(Serialize)]
#[serde(tag = "packet_type")]
enum DecodedPacket {
    SensorPacket(SensorPacket)
}

struct SensorPacket {
    serial: u16,
    tick: f64,
    ptype: u8,
    state: i16,
    accel: i16,
    pres: i16,
    temp: f64,
    v_batt: f64,
    sense_d: i16,
    sense_m: i16,
    acceleration: f64,
    speed: f64,
    height: i16,
    ground_press: i16,
    ground_accel: i16,
    accel_plus_g: i16,
    accel_minus_g: i16,
}

pub fn decode(packet: Packet) -> Result<DecodedPacket, std::io::Error> {
    let ptype = packet.data[4];
    let d = std::io::Cursor::new(packet.data());
    match ptype {
        1 => {
            DecodedPacket::SensorPacket(SensorPacket{
                serial: d.read_u16()?,
                tick: d.read_u16()? as f64 / 100.0,
                ptype: d.read_u8()?,
                state: d.read_u8()?,
                accel: d.read_i16()?,

            })
        }
    }

    todo!()
}