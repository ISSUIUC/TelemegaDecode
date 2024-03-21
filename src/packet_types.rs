use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};
use crate::Packet;
use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "packet_type")]
pub enum DecodedPacket {
    SensorPacket(SensorPacket),
    ConfigPacket(ConfigPacket),
    GPSPacket(GPSPacket),
    SatellitePacket(SatellitePacket),
    KalmanVoltagePacket(KalmanVoltagePacket),
    UnknownPacket(UnknownPacket)
}

#[derive(Serialize)]
pub struct UnknownPacket {
    serial: u16,
    tick: f64,
    ptype: u8,
    crc: bool,
}

#[derive(Serialize)]
pub struct SensorPacket {
    serial: u16,
    tick: f64,
    ptype: u8,
    state: u8,
    accel: i16,
    pres: i16,
    temp: f64,
    v_batt: i16,
    sense_d: i16,
    sense_m: i16,
    acceleration: f64,
    speed: f64,
    height: i16,
    ground_press: i16,
    ground_accel: i16,
    accel_plus_g: i16,
    accel_minus_g: i16,
    crc: bool,
}

#[derive(Serialize)]
pub struct ConfigPacket {
    serial: u16,
    tick: f64,
    ptype: u8,
    device_type: u8,
    flight: u16,
    config_major: u8,
    config_minor: u8,
    apogee_delay: u16,
    main_deploy: u16,
    flight_log_max: u16,
    callsign: [u8;8],
    version: [u8;8],
    crc: bool,
}

#[derive(Serialize)]
pub struct GPSPacket {
    serial: u16,
    tick: f64,
    ptype: u8,
    nsats: u8,
    valid: bool,
    running: bool,
    date_valid: bool,
    course_valid: bool,
    altitude: i16,
    latitude: f64,
    longitude: f64,
    year: usize,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    pdop: f64,
    hdop: f64,
    vdop: f64,
    mode: u8,
    ground_speed: f64,
    climb_rate: f64,
    course: f64,
    crc: bool,
}

#[derive(Serialize)]
pub struct SatellitePacket {
    serial: u16,
    tick: f64,
    ptype: u8,
    channels: u8,
    sats: [u8;24],
    crc: bool,
}

#[derive(Serialize)]
pub struct KalmanVoltagePacket {
    serial: u16,
    tick: f64,
    ptype: u8,
    state: u8,
    v_batt: i16,
    v_pyro: i16,
    sense: [u8;6],
    ground_pres: i32,
    ground_accel: i16,
    accel_plus_g: i16,
    accel_minus_g: i16,
    acceleration: f64,
    speed: f64,
    height: i16,
    crc: bool,
}

pub fn decode(packet: &Packet) -> Result<DecodedPacket, std::io::Error> {
    let ptype = packet.data[4];
    let mut d = std::io::Cursor::new(packet.data);
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
            let mut callsign = [0u8;8];
            callsign.copy_from_slice(&packet.data[16..24]);
            let mut version = [0u8;8];
            version.copy_from_slice(&packet.data[24..32]);

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