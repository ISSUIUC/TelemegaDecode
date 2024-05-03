import { LitElement, css, html } from "lit";
import { customElement, property } from "lit/decorators.js"
import { DecodedPacket, SensorPacket, ConfigPacket, GPSPacket, SatellitePacket, parse_packet, KalmanVoltagePacket } from "../packet";


@customElement('sensor-packet')
export class KalmanVoltagePacketView extends LitElement {
    @property()
    data: KalmanVoltagePacket;
    static css = css`
    .tab {
        display: inline-block;
        margin-left: 40px;
    }
    `
    render() {
        if(this.data == null) return html`NODATA`;
        return html`
            CRC: ${this.data.crc}

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
                    <th> V_Batt: </th>
                    <td> ${this.data.v_batt}
                </tr>
                <tr>
                    <th> V_Pyro: </th>
                    <td> ${this.data.v_pyro}
                </tr>
                <tr>
                    <th> Sense: </th>
                    <td> ${this.data.sense}
                </tr>
                <tr>
                    <th> v_apogee: </th>
                    <td> ${this.data.v_apogee}
                </tr>
                <tr>
                    <th> v_main: </th>
                    <td> ${this.data.v_main}
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
                    <td> ${this.data.ground_pres}
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
            CRC: ${this.data.crc}

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
        if(this.data == null) return html`NODATA`;
        return html`
            CRC: ${this.data.crc}

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
            CRC: ${this.data.crc}
            <table>
                <tr>
                    <th>Serial:</th>
                    <td>${this.data.serial}</td>
                </tr>
                <tr>
                    <th>Tick:</th>
                    <td>${this.data.tick}</td>
                </tr>
                <tr>
                    <th>Channels:</th>
                    <td>${this.data.channels}</td>
                </tr>
            </table>
        `
    }
}

@customElement('telemetrum-dataview')
export class TeleMegaDataView extends LitElement {
    @property()
    public sensor: KalmanVoltagePacketView = new KalmanVoltagePacketView();

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
        <div class="grid-item"><b>Sustainer AL0</b></div>
        <div class="grid-item"><b>Sustainer AL1</b></div>
        <div class="grid-item"><b>Booster AL0</b></div>
        <div class="grid-item"><b>Other</b></div>
        <div class="grid-item">${view2}</div>
        <div class="grid-item">${view}</div>
        <div class="grid-item">${view3}</div>
        <div class="grid-item">${view4}</div>
      </div>
            `
    }
}


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
            let v = view4;
            if(elem.serial == 11216){
                v = view;
            }
            if(elem.serial == 11124){
                v = view2;
            }
            
            if(elem.serial == 11069){
                v = view3
            }
            console.log(elem)
            switch(elem.ptype){
                case 1:
                case 9:
                    v.sensor.data = elem as KalmanVoltagePacket;
                    v.sensor.requestUpdate();
                    break;
                case 4:
                    v.config.data = elem as ConfigPacket;
                    v.config.requestUpdate();
                    break;
                case 5:
                    v.gps.data = elem as GPSPacket;
                    v.gps.requestUpdate();
                    break;
                case 6:
                    v.sat.data = elem as SatellitePacket;
                    v.sat.requestUpdate();
                    break;
            }
            console.log(elem);
        }
        view.requestUpdate();
    }
}, 300)