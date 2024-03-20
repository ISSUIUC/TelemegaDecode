import { LitElement, css, html } from "lit";
import { customElement, property } from "lit/decorators.js"
import { DecodedPacket, SensorPacket, ConfigPacket, GPSPacket, SatellitePacket } from "../packet";


@customElement('sensor-packet')
export class SensorPacketView extends LitElement {
    @property()
    data: SensorPacket;
    static css = css`
    .tab {
        display: inline-block;
        margin-left: 40px;
    }
    `
    render() {
        return html`
            Sensor Packet:

            <table>
                <tr>
                    <th> Serial: </th>
                    <td> ${this.data.serial}
                </tr>
                <tr>
                    <th> Tick: </th>
                    <td> ${this.data.tick}
                </tr>
                <tr>
                    <th> Type: </th>
                    <td> ${this.data.type}
                </tr>
                <tr>
                    <th> State: </th>
                    <td> ${this.data.state}
                </tr>
                <tr>
                    <th> Accel: </th>
                    <td> ${this.data.accel}
                </tr>
                <tr>
                    <th> Pres: </th>
                    <td> ${this.data.pres}
                </tr>
                <tr>
                    <th> Temp: </th>
                    <td> ${this.data.temp}
                </tr>
                <tr>
                    <th> V_Batt: </th>
                    <td> ${this.data.v_batt}
                </tr>
                <tr>
                    <th> Sense_d: </th>
                    <td> ${this.data.sense_d}
                </tr>
                <tr>
                    <th> Sense_m: </th>
                    <td> ${this.data.sense_m}
                </tr>
                <tr>
                    <th> Acceleration: </th>
                    <td> ${this.data.acceleration}
                </tr>
                <tr>
                    <th> Speed: </th>
                    <td> ${this.data.speed}
                </tr>
                <tr>
                    <th> Height: </th>
                    <td> ${this.data.height}
                </tr>
                <tr>
                    <th> Ground_Press: </th>
                    <td> ${this.data.ground_press}
                </tr>
                <tr>
                    <th> Ground_Accel: </th>
                    <td> ${this.data.ground_accel}
                </tr>
                <tr>
                    <th> Accel_Plus_g: </th>
                    <td> ${this.data.accel_plus_g}
                </tr>
                <tr>
                    <th> Accel_Minus_g: </th>
                    <td> ${this.data.accel_minus_g}
                </tr>
            </table>
        `
    }
}


@customElement('config-packet')
export class ConfigPacketView extends LitElement {
    @property()
    data: ConfigPacket;

    render() {
        return html`
            Sensor Packet:

            <table>
                <tr>
                    <th> Serial: </th>
                    <td> ${this.data.serial}
                </tr>
                <tr>
                    <th> Tick: </th>
                    <td> ${this.data.tick}
                </tr>
                <tr>
                    <th> Type: </th>
                    <td> ${this.data.type}
                </tr>
                <tr>
                    <th> Flight: </th>
                    <td> ${this.data.flight}
                </tr>
                <tr>
                    <th> Config_Major: </th>
                    <td> ${this.data.config_major}
                </tr>
                <tr>
                    <th> Config_Minor: </th>
                    <td> ${this.data.config_minor}
                </tr>
                <tr>
                    <th> Apogee_Delay: </th>
                    <td> ${this.data.apogee_delay}
                </tr>
                <tr>
                    <th> Main_Deploy: </th>
                    <td> ${this.data.main_deploy}
                </tr>
                <tr>
                    <th> Log_Max: </th>
                    <td> ${this.data.flight_log_max}
                </tr>
                <tr>
                    <th> Callsign: </th>
                    <td> ${this.data.callsign}
                </tr>
                <tr>
                    <th> Version: </th>
                    <td> ${this.data.version}
                </tr>
            </table>
        `
    }
}


@customElement('gps-packet')
export class GpsPacketView extends LitElement {
    @property()
    data: GPSPacket;

    render() {
        return html`
            Sensor Packet

            <table>
                <tr>
                    <th> Serial: </th>
                    <td> ${this.data.serial}
                </tr>
                <tr>
                    <th> Tick: </th>
                    <td> ${this.data.tick}
                </tr>
                <tr>
                    <th> Type: </th>
                    <td> ${this.data.type}
                </tr>
                <tr>
                    <th> nsats: </th>
                    <td> ${this.data.nsats}
                </tr>
                <tr>
                    <th> Valid: </th>
                    <td> ${this.data.valid}
                </tr>
                <tr>
                    <th> Running: </th>
                    <td> ${this.data.running}
                </tr>
                <tr>
                    <th> Date_Valid: </th>
                    <td> ${this.data.date_valid}
                </tr>
                <tr>
                    <th> Course_Valid: </th>
                    <td> ${this.data.course_valid}
                </tr>
                <tr>
                    <th> Altitude: </th>
                    <td> ${this.data.altitude}
                </tr>
                <tr>
                    <th> Latitude: </th>
                    <td> ${this.data.latitude}
                </tr>
                <tr>
                    <th> Longitude: </th>
                    <td> ${this.data.longitude}
                </tr>
                <tr>
                    <th> Year: </th>
                    <td> ${this.data.year}
                </tr>
                <tr>
                    <th> Month: </th>
                    <td> ${this.data.month}
                </tr>
                <tr>
                    <th> Day: </th>
                    <td> ${this.data.day}
                </tr>
                <tr>
                    <th> Hour: </th>
                    <td> ${this.data.hour}
                </tr>
                <tr>
                    <th> Minute: </th>
                    <td> ${this.data.minute}
                </tr>
                <tr>
                    <th> Second: </th>
                    <td> ${this.data.second}
                </tr>
            </table>

        `
    }
}


@customElement('sat-packet')
export class SatellitePacketView extends LitElement {
    @property()
    data: SatellitePacket;

    render() {
        return html`
            Sensor Packet
        `
    }
}

@customElement('telemetrum-dataview')
export class TeleMegaDataView extends LitElement {
    @property()
    public serial: number;

    @property()
    public sensor: SensorPacketView = new SensorPacketView();

    @property()
    public config: ConfigPacketView = new ConfigPacketView();

    @property()
    public gps: GpsPacketView = new GpsPacketView();

    @property()
    public sat: SatellitePacketView = new SatellitePacketView();

    static styles = css`
    .grid-container {
        display: grid;
        grid-template-columns: 50% 50%;
        border: solid 1px;
    }
    .grid-item {
        border: solid 1px;
        padding: 3px;
    }
    `
    render() {
        return html`
    <div>
        Serial: ${this.serial}
        <br>
      <div class="grid-container">
        <div class="grid-item">Sensor: ${this.sensor}</div>
        <div class="grid-item">Config: ${this.config}</div>
        <div class="grid-item">GPS: ${this.gps}</div>
        <div class="grid-item">Sat: ${this.sat}</div>
      </div>
    </div>
        `
    }
}


const view = new TeleMegaDataView();
const view2 = new TeleMegaDataView();
const view3 = new TeleMegaDataView();
const view4 = new TeleMegaDataView();

view.serial = 1018;

// document.body.appendChild(view);
// document.body.appendChild(view2);
// document.body.appendChild(view3);
// document.body.appendChild(view4);




let number = 7;
let string = "str";
let boolean = true;

view.sensor.data = {serial : number,
    tick : number,
    type : 1,
    state : number,
    accel : number,
    pres : number,
    temp : number,
    v_batt : number,
    sense_d : number,
    sense_m : number,
    acceleration : number,
    speed : number,
    height : number,
    ground_press : number,
    ground_accel : number,
    accel_plus_g : number,
    accel_minus_g : number,}

view.config.data = {
    serial : number,
    tick : number,
    type : 4,
    flight : number,
    config_major : number,
    config_minor : number,
    apogee_delay : number,
    main_deploy : number,
    flight_log_max : number,
    callsign : string,
    version : string,
}

view.gps.data = {
    serial : number,
    tick : number,
    type : 5,
    nsats : number,
    valid : boolean,
    running: boolean,
    date_valid: boolean,
    course_valid: boolean,
    altitude : number,
    latitude : number,
    longitude : number,
    year : number,
    month : number,
    day : number,
    hour : number,
    minute : number,
    second : number,
    pdop : number,
    hdop : number,
    vdop : number,
    mode : number,
    ground_speed : number,
    climb_rate : number,
    course : number,
}

view2.sensor.data = {serial : number,
    tick : number,
    type : 1,
    state : number,
    accel : number,
    pres : number,
    temp : number,
    v_batt : number,
    sense_d : number,
    sense_m : number,
    acceleration : number,
    speed : number,
    height : number,
    ground_press : number,
    ground_accel : number,
    accel_plus_g : number,
    accel_minus_g : number,}

view2.config.data = {
    serial : number,
    tick : number,
    type : 4,
    flight : number,
    config_major : number,
    config_minor : number,
    apogee_delay : number,
    main_deploy : number,
    flight_log_max : number,
    callsign : string,
    version : string,
}
view2.gps.data = {
    serial : number,
    tick : number,
    type : 5,
    nsats : number,
    valid : boolean,
    running: boolean,
    date_valid: boolean,
    course_valid: boolean,
    altitude : number,
    latitude : number,
    longitude : number,
    year : number,
    month : number,
    day : number,
    hour : number,
    minute : number,
    second : number,
    pdop : number,
    hdop : number,
    vdop : number,
    mode : number,
    ground_speed : number,
    climb_rate : number,
    course : number,
}

view3.sensor.data = {serial : number,
    tick : number,
    type : 1,
    state : number,
    accel : number,
    pres : number,
    temp : number,
    v_batt : number,
    sense_d : number,
    sense_m : number,
    acceleration : number,
    speed : number,
    height : number,
    ground_press : number,
    ground_accel : number,
    accel_plus_g : number,
    accel_minus_g : number,}

view3.config.data = {
    serial : number,
    tick : number,
    type : 4,
    flight : number,
    config_major : number,
    config_minor : number,
    apogee_delay : number,
    main_deploy : number,
    flight_log_max : number,
    callsign : string,
    version : string,
}
view3.gps.data = {
    serial : number,
    tick : number,
    type : 5,
    nsats : number,
    valid : boolean,
    running: boolean,
    date_valid: boolean,
    course_valid: boolean,
    altitude : number,
    latitude : number,
    longitude : number,
    year : number,
    month : number,
    day : number,
    hour : number,
    minute : number,
    second : number,
    pdop : number,
    hdop : number,
    vdop : number,
    mode : number,
    ground_speed : number,
    climb_rate : number,
    course : number,
}

view4.sensor.data = {serial : number,
    tick : number,
    type : 1,
    state : number,
    accel : number,
    pres : number,
    temp : number,
    v_batt : number,
    sense_d : number,
    sense_m : number,
    acceleration : number,
    speed : number,
    height : number,
    ground_press : number,
    ground_accel : number,
    accel_plus_g : number,
    accel_minus_g : number,}

view4.config.data = {
    serial : number,
    tick : number,
    type : 4,
    flight : number,
    config_major : number,
    config_minor : number,
    apogee_delay : number,
    main_deploy : number,
    flight_log_max : number,
    callsign : string,
    version : string,
}
view4.gps.data = {
    serial : number,
    tick : number,
    type : 5,
    nsats : number,
    valid : boolean,
    running: boolean,
    date_valid: boolean,
    course_valid: boolean,
    altitude : number,
    latitude : number,
    longitude : number,
    year : number,
    month : number,
    day : number,
    hour : number,
    minute : number,
    second : number,
    pdop : number,
    hdop : number,
    vdop : number,
    mode : number,
    ground_speed : number,
    climb_rate : number,
    course : number,
}



@customElement('four-data-view')
export class FourDataView extends LitElement {
    // @property()
    // public serial: number;
    
    // @property()
    // public sensor: SensorPacketView = new SensorPacketView();
    
    // @property()
    // public config: ConfigPacketView = new ConfigPacketView();
    
    // @property()
    // public gps: GpsPacketView = new GpsPacketView();
    
    // @property()
    // public sat: SatellitePacketView = new SatellitePacketView();
    static styles = css`
    .grid-container {
        display: grid;
        grid-template-columns: 25% 25% 25% 25%;
    }
    `
    render() {
        return html`
      <div class="grid-container">
        <div class="grid-item"><b>TeleMega 1 Data</b></div>
        <div class="grid-item"><b>TeleMega 2 Data</b></div>
        <div class="grid-item"><b>TeleMega 3 Data</b></div>
        <div class="grid-item"><b>TeleMega 4 Data</b></div>
        <div class="grid-item">${view}</div>
        <div class="grid-item">${view2}</div>
        <div class="grid-item">${view3}</div>
        <div class="grid-item">${view4}</div>
      </div>
            `
    }
}

// <table>
//         <tr>
//             <th>TeleMega 1 Data:</th>
//             <th>TeleMega 2 Data:</th>
//             <th>TeleMega 3 Data:</th>
//             <th>TeleMega 4 Data:</th>
//         </tr>
//         <tr>
//             <td>${view}</td>
//             <td>${view2}</td>
//             <td>${view3}</td>
//             <td>${view4}</td>
//         </tr>
//       </table>


const fullDataView = new FourDataView();
document.body.appendChild(fullDataView);

let in_flight = false;
setInterval(async ()=>{
    if(in_flight) return;
    in_flight = true;
    const json: DecodedPacket[] = await (await fetch("/getdata")).json()
    in_flight = false;
    if(json instanceof Array){
        for(const elem of json){
            switch(elem.type){
                case 1:
                    view.sensor.data = elem;
                    view.sensor.requestUpdate();
                    break;
                case 4:
                    view.config.data = elem;
                    view.config.requestUpdate();
                    break;
                case 5:
                    view.gps.data = elem;
                    view.gps.requestUpdate();
                    break;
                case 6:
                    view.sat.data = elem;
                    view.sat.requestUpdate();
                    break;
            }
            console.log(elem);
        }
        view.requestUpdate();
    }
}, 300)