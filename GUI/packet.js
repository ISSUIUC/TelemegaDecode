"use strict";
exports.__esModule = true;
exports.parse_packet = void 0;
var decoder = new TextDecoder();
function pyro_voltage(x) {
    return (3.3 * x / 4095 * (100 + 27) / 27);
}
function parse_packet(packet) {
    var u8 = new Uint8Array(packet.data).slice(0, 32);
    var u16 = new Uint16Array(u8.buffer);
    var i16 = new Int16Array(u8.buffer);
    var i32 = new Int32Array(u8.buffer);
    var ptype = u8[4];
    if (ptype == 1) { //TeleMetrum v1.x Sensor Data
        return {
            "serial": u16[0],
            "tick": u16[1] / 100,
            "ptype": 1,
            "state": u8[5],
            "accel": i16[3],
            "pres": i16[4],
            "temp": i16[5] / 100,
            "v_batt": i16[6].toString(),
            "sense_d": i16[7],
            "sense_m": i16[8],
            "acceleration": i16[9] / 16,
            "speed": i16[10] / 16,
            "height": i16[11],
            "ground_press": i16[12],
            "ground_accel": i16[13],
            "accel_plus_g": i16[14],
            "accel_minus_g": i16[15],
            "crc": packet.crc
        };
    }
    else if (ptype == 4) {
        return {
            "serial": u16[0],
            "tick": u16[1] / 100,
            "ptype": 4,
            "flight": u16[3],
            "config_major": u8[8],
            "config_minor": u8[9],
            "apogee_delay": u16[5],
            "main_deploy": u16[6],
            "flight_log_max": u16[7],
            "callsign": decoder.decode(u8.subarray(16, 24)),
            "version": decoder.decode(u8.subarray(24, 32)),
            "crc": packet.crc
        };
    }
    else if (ptype == 5) {
        return {
            "serial": u16[0],
            "tick": u16[1] / 100,
            "ptype": 5,
            "nsats": u8[5] & 0x7,
            "valid": (u8[5] & 0x8) != 0,
            "running": (u8[5] & 0x10) != 0,
            "date_valid": (u8[5] & 0x20) != 0,
            "course_valid": (u8[5] & 0x40) != 0,
            "altitude": i16[3],
            "latitude": i32[2] / Math.pow(10, 7),
            "longitude": i32[3] / Math.pow(10, 7),
            "year": u8[16] + 2000,
            "month": u8[17],
            "day": u8[18],
            "hour": u8[19],
            "minute": u8[20],
            "second": u8[21],
            "pdop": u8[22] / 5,
            "hdop": u8[23] / 5,
            "vdop": u8[24] / 5,
            "mode": u8[25],
            "ground_speed": u16[13] / 100,
            "climb_rate": i16[14] / 100,
            "course": u8[30] * 2,
            "crc": packet.crc
        };
    }
    else if (ptype == 6) {
        return {
            "serial": u16[0],
            "tick": u16[1] / 100,
            "ptype": 6,
            "channels": u8[5],
            "sats": Array.from(u8.subarray(6, 30)),
            "crc": packet.crc
        };
    }
    else if (ptype == 9) {
        return {
            "serial": u16[0],
            "tick": u16[1] / 100,
            "ptype": 9,
            "state": u8[5],
            "v_batt": (3.3 * i16[3] / 4095 * (5.6 + 10.0) / 10.0).toFixed(2),
            "v_pyro": pyro_voltage(i16[4]).toFixed(2),
            "sense": Array.from(u8.slice(10, 14)).map(function (x) { return (x / 68 * 4.14).toFixed(2); }).join(' '),
            "v_apogee": (u8[14] / 68 * 4.14).toFixed(2),
            "v_main": (u8[15] / 68 * 4.14).toFixed(2),
            "ground_pres": i32[4],
            "ground_accel": i16[10],
            "accel_plus_g": i16[11],
            "accel_minus_g": i16[12],
            "acceleration": i16[13] / 16,
            "speed": i16[14] / 16,
            "height": i16[15],
            "crc": packet.crc
        };
    }
    else {
        return {
            "serial": u16[0],
            "tick": u16[1] / 100,
            "ptype": ptype,
            "crc": packet.crc
        };
    }
}
exports.parse_packet = parse_packet;
