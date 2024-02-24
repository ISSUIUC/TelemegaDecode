export type GFSKPacket = {
    type: "packet",
    crc: boolean,
    data: number[],
    id: number,
}


export type GFSKMessage = GFSKPacket 
| {
    type: "error",
    error: string,
    file: string,
    line: number,
} | {
    type: "gain",
    lna: number,
    vga: number,
} | {
    type: "center",
    center: number,
} | {
    type: "closed"
}

const decoder = new TextDecoder();

export function parse_packet(packet: GFSKPacket){
    const u8 = new Uint8Array(packet.data).slice(0,32);
    const u16 = new Uint16Array(u8.buffer);
    const i16 = new Int16Array(u8.buffer);
    const i32 = new Int32Array(u8.buffer);
    const type = u8[4];
    if(type == 1){ //TeleMetrum v1.x Sensor Data
        return {
            "serial" : u16[0],
            "tick" : u16[1] / 100,
            "type" : u8[4],
            "state" : u8[5],
            "accel" : i16[3],
            "pres" : i16[4],
            "temp" : i16[5] / 100,
            "v_batt" : i16[6],
            "sense_d" : i16[7],
            "sense_m" : i16[8],
            "acceleration" : i16[9] / 16,
            "speed" : i16[10] / 16,
            "height" : i16[11],
            "ground_press" : i16[12],
            "ground_accel" : i16[13],
            "accel_plus_g" : i16[14],
            "accel_minus_g" : i16[15],
        }
    } else if(type == 4) {
        return {
            "serial" : u16[0],
            "tick" : u16[1] / 100,
            "type" : u8[4],
            "flight" : u16[3],
            "config_major" : u8[8],
            "config_minor" : u8[9],
            "apogee_delay" : u16[5],
            "main_deploy" : u16[6],
            "flight_log_max" : u16[7],
            "callsign" : decoder.decode(u8.subarray(16,24)),
            "version" : decoder.decode(u8.subarray(24,32)),
        }
    } else if(type == 5){
        return {
            "serial" : u16[0],
            "tick" : u16[1] / 100,
            "type" : u8[4],
            "nsats" : u8[5] & 0x7,
            "valid" : (u8[5] & 0x8) != 0,
            "running": (u8[5] & 0x10) != 0,
            "date_valid": (u8[5] & 0x20) != 0,
            "course_valid": (u8[5] & 0x40) != 0,
            "altitude" : i16[3],
            "latitude" : i32[2] / 10**7,
            "longitude" : i32[3] / 10**7,
            "year" : u8[16] + 2000,
            "month" : u8[17],
            "day" : u8[18],
            "hour" : u8[19],
            "minute" : u8[20],
            "second" : u8[21],
            "pdop" : u8[22] / 5,
            "hdop" : u8[23] / 5,
            "vdop" : u8[24] / 5,
            "mode" : u8[25],
            "ground_speed" : u16[13] / 100,
            "climb_rate" : i16[14] / 100,
            "course" : u8[30] * 2,
        }
    } else if(type == 6){
        return {
            "serial" : u16[0],
            "tick" : u16[1] / 100,
            "type" : u8[4],
            "channels" : u8[5],
            "sats" : u16.subarray(3,15)
        }
    }
}