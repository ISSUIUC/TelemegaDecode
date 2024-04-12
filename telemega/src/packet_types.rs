use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};
use serde::Serialize;
use crate::Packet;

#[derive(Serialize, Clone, Debug)]
#[serde(tag = "packet_type")]
pub enum DecodedPacket {
    SensorPacket(SensorPacket),
    ConfigPacket(ConfigPacket),
    GPSPacket(GPSPacket),
    SatellitePacket(SatellitePacket),
    KalmanVoltagePacket(KalmanVoltagePacket),
    UnknownPacket(UnknownPacket)
}

impl DecodedPacket {
    pub fn crc_match(&self) -> bool {
        match self {
            DecodedPacket::SensorPacket(packet) => packet.crc,
            DecodedPacket::ConfigPacket(packet) => packet.crc,
            DecodedPacket::GPSPacket(packet) => packet.crc,
            DecodedPacket::SatellitePacket(packet) => packet.crc,
            DecodedPacket::KalmanVoltagePacket(packet) => packet.crc,
            DecodedPacket::UnknownPacket(packet) => packet.crc,
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct UnknownPacket {
    pub serial: u16,
    pub tick: f64,
    pub ptype: u8,
    pub crc: bool,
}

#[derive(Serialize, Clone, Debug)]
pub struct SensorPacket {
    pub serial: u16,
    pub tick: f64,
    pub ptype: u8,
    pub state: u8,
    pub accel: i16,
    pub pres: i16,
    pub temp: f64,
    pub v_batt: i16,
    pub sense_d: i16,
    pub sense_m: i16,
    pub acceleration: f64,
    pub speed: f64,
    pub height: i16,
    pub ground_press: i16,
    pub ground_accel: i16,
    pub accel_plus_g: i16,
    pub accel_minus_g: i16,
    pub crc: bool,
}

#[derive(Serialize, Clone, Debug)]
pub struct ConfigPacket {
    pub serial: u16,
    pub tick: f64,
    pub ptype: u8,
    pub device_type: u8,
    pub flight: u16,
    pub config_major: u8,
    pub config_minor: u8,
    pub apogee_delay: u16,
    pub main_deploy: u16,
    pub flight_log_max: u16,
    pub callsign: String,
    pub version: String,
    pub crc: bool,
}

#[derive(Serialize, Clone, Debug)]
pub struct GPSPacket {
    pub serial: u16,
    pub tick: f64,
    pub ptype: u8,
    pub nsats: u8,
    pub valid: bool,
    pub running: bool,
    pub date_valid: bool,
    pub course_valid: bool,
    pub altitude: i16,
    pub latitude: f64,
    pub longitude: f64,
    pub year: usize,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub pdop: f64,
    pub hdop: f64,
    pub vdop: f64,
    pub mode: u8,
    pub ground_speed: f64,
    pub climb_rate: f64,
    pub course: f64,
    pub crc: bool,
}

#[derive(Serialize, Clone, Debug)]
pub struct SatellitePacket {
    pub serial: u16,
    pub tick: f64,
    pub ptype: u8,
    pub channels: u8,
    pub sats: [u8; 24],
    pub crc: bool,
}

#[derive(Serialize, Clone, Debug)]
pub struct KalmanVoltagePacket {
    pub serial: u16,
    pub tick: f64,
    pub ptype: u8,
    pub state: u8,
    pub v_batt: i16,
    pub v_pyro: i16,
    pub sense: [u8; 6],
    pub ground_pres: i32,
    pub ground_accel: i16,
    pub accel_plus_g: i16,
    pub accel_minus_g: i16,
    pub acceleration: f64,
    pub speed: f64,
    pub height: i16,
    pub crc: bool,
}

pub(crate) fn decode(packet: &Packet) -> Result<DecodedPacket, std::io::Error> {
    let ptype = packet.data[4];
    let mut d = Cursor::new(packet.data);
    let p = match ptype {
        1 => {
            DecodedPacket::SensorPacket(SensorPacket{
                serial: d.read_u16::<LittleEndian>()?,
                tick: d.read_u16::<LittleEndian>()? as f64 / 100.0,
                ptype: d.read_u8()?,
                state: d.read_u8()?,
                accel: d.read_i16::<LittleEndian>()?,
                pres: d.read_i16::<LittleEndian>()?,
                temp: d.read_i16::<LittleEndian>()? as f64 / 100.0,
                v_batt: d.read_i16::<LittleEndian>()?,
                sense_d: d.read_i16::<LittleEndian>()?,
                sense_m: d.read_i16::<LittleEndian>()?,
                acceleration: d.read_i16::<LittleEndian>()? as f64 / 16.0,
                speed: d.read_i16::<LittleEndian>()? as f64 / 16.0,
                height: d.read_i16::<LittleEndian>()?,
                ground_press: d.read_i16::<LittleEndian>()?,
                ground_accel: d.read_i16::<LittleEndian>()?,
                accel_plus_g: d.read_i16::<LittleEndian>()?,
                accel_minus_g: d.read_i16::<LittleEndian>()?,
                crc: packet.crc_match,
            })
        },
        4 => {
            let callsign = std::str::from_utf8(&packet.data[16..24]).unwrap_or("ERR").to_string();
            let version = std::str::from_utf8(&packet.data[24..32]).unwrap_or("ERR").to_string();

            DecodedPacket::ConfigPacket(ConfigPacket{
                serial: d.read_u16::<LittleEndian>()?,
                tick: d.read_u16::<LittleEndian>()? as f64 / 100.0,
                ptype: d.read_u8()?,
                device_type: d.read_u8()?,
                flight: d.read_u16::<LittleEndian>()?,
                config_major: d.read_u8()?,
                config_minor: d.read_u8()?,
                apogee_delay: d.read_u16::<LittleEndian>()?,
                main_deploy: d.read_u16::<LittleEndian>()?,
                flight_log_max: d.read_u16::<LittleEndian>()?,
                callsign,
                version,
                crc: packet.crc_match,
            })
        },
        5 => {
            let packed = packet.data[5];
            DecodedPacket::GPSPacket(GPSPacket{
                serial: d.read_u16::<LittleEndian>()?,
                tick: d.read_u16::<LittleEndian>()? as f64 / 100.0,
                ptype: d.read_u8()?,
                nsats: d.read_u8()? & 0x7,
                valid: packed & 0x8 != 0,
                running: packed & 0x10 != 0,
                date_valid: packed & 0x20 != 0,
                course_valid: packed & 0x40 != 0,
                altitude: d.read_i16::<LittleEndian>()?,
                latitude: d.read_i32::<LittleEndian>()? as f64 / 1E7,
                longitude: d.read_i32::<LittleEndian>()? as f64 / 1E7,
                year: d.read_u8()? as usize + 2000,
                month: d.read_u8()?,
                day: d.read_u8()?,
                hour: d.read_u8()?,
                minute: d.read_u8()?,
                second: d.read_u8()?,
                pdop: d.read_u8()? as f64 / 5.0,
                hdop: d.read_u8()? as f64 / 5.0,
                vdop: d.read_u8()? as f64 / 5.0,
                mode: d.read_u8()?,
                ground_speed: d.read_u16::<LittleEndian>()? as f64 / 100.0,
                climb_rate: d.read_i16::<LittleEndian>()? as f64 / 100.0,
                course: d.read_u8()? as f64 * 2.0,
                crc: packet.crc_match,
            })
        },
        6 => {
            let mut sats = [0u8;24];
            sats.copy_from_slice(&packet.data[6..30]);
            DecodedPacket::SatellitePacket(SatellitePacket{
                serial: d.read_u16::<LittleEndian>()?,
                tick: d.read_u16::<LittleEndian>()? as f64 / 100.0,
                ptype: d.read_u8()?,
                channels: d.read_u8()?,
                sats,
                crc: packet.crc_match,
            })
        },
        9 => {
            let mut sense = [0u8;6];
            sense.copy_from_slice(&packet.data[10..16]);
            let mut d2 = Cursor::new(&packet.data[16..]);
            DecodedPacket::KalmanVoltagePacket(KalmanVoltagePacket{
                serial: d.read_u16::<LittleEndian>()?,
                tick: d.read_u16::<LittleEndian>()? as f64 / 100.0,
                ptype: d.read_u8()?,
                state: d.read_u8()?,
                v_batt: d.read_i16::<LittleEndian>()?,
                v_pyro: d.read_i16::<LittleEndian>()?,
                sense,
                ground_pres: d2.read_i32::<LittleEndian>()?,
                ground_accel: d2.read_i16::<LittleEndian>()?,
                accel_plus_g: d2.read_i16::<LittleEndian>()?,
                accel_minus_g: d2.read_i16::<LittleEndian>()?,
                acceleration: d2.read_i16::<LittleEndian>()? as f64 / 16.0,
                speed: d2.read_i16::<LittleEndian>()? as f64 / 16.0,
                height: d2.read_i16::<LittleEndian>()?,
                crc: packet.crc_match,
            })
        }
        _ => {
            DecodedPacket::UnknownPacket(UnknownPacket{
                serial: d.read_u16::<LittleEndian>()?,
                tick: d.read_u16::<LittleEndian>()? as f64 / 100.0,
                ptype: d.read_u8()?,
                crc: packet.crc_match,
            })
        }
    };

    Ok(p)
}