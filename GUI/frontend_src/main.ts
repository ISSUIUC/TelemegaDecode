import { LitElement, css, html } from "lit";
import { customElement, property } from "lit/decorators.js"
import { DecodedPacket, SensorPacket, ConfigPacket, GPSPacket, SatellitePacket, parse_packet } from "../packet";


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
        if(this.data == null) return html`NODATA`;
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
                    <td> ${this.data.ptype}
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
        if(this.data == null) return html`NODATA`;
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
                    <td> ${this.data.ptype}
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
                    <th> Flight_Log_Max: </th>
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
        if(this.data == null) return html`NODATA`;
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
                    <td> ${this.data.ptype}
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
        if(this.data == null) return html`NODATA`;
        return html`
            Sensor Packet
        `
    }
}

@customElement('telemetrum-dataview')
export class TeleMegaDataView extends LitElement {
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
        grid-template-columns: auto auto;
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
            let v = null;
            if(elem.serial == 10978){
                v = view;
            }
            if(elem.serial == 11047){
                v = view2;
            }
            if(elem.serial == 1018){
                v = view4;
            }
            if(elem.serial == 11069){
                v = view3
            }
            console.log(elem)
            switch(elem.ptype){
                case 1:
                case 9:
                    v.sensor.data = elem;
                    v.sensor.requestUpdate();
                    break;
                case 4:
                    v.config.data = elem;
                    v.config.requestUpdate();
                    break;
                case 5:
                    v.gps.data = elem;
                    v.gps.requestUpdate();
                    break;
                case 6:
                    v.sat.data = elem;
                    v.sat.requestUpdate();
                    break;
            }
            console.log(elem);
        }
        view.requestUpdate();
    }
}, 300)