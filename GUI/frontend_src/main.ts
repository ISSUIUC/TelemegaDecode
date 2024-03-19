import { LitElement, css, html } from "lit";
import { customElement, property } from "lit/decorators.js"
import { DecodedPacket, SensorPacket, ConfigPacket, GPSPacket, SatellitePacket, parse_packet } from "../packet";


@customElement('sensor-packet')
export class SensorPacketView extends LitElement {
    @property()
    data: SensorPacket;

    render() {
        if(this.data == null) return html`NODATA`;
        return html`
            <table>
                <tr>
                Tick: ${this.data.tick} State: ${this.data.state}
    </tr><tr>
                Accel: ${this.data.accel} Pres: ${this.data.pres} Temp: ${this.data.temp} c
    </tr><tr>
                Battery Voltage ${this.data.v_batt}
                </tr><tr>
                Drogue Continuity ${this.data.sense_d} Main Continuity ${this.data.sense_m}
                </tr><tr>
                Acceleration: ${this.data.acceleration}m/s Speed ${this.data.speed}m/s Height ${this.data.height}m
                </tr><tr>
                Ground Pressure ${this.data.ground_press} Ground Accel ${this.data.ground_accel}
                </tr><tr>
                Accel Plus G ${this.data.accel_plus_g} Accel Minus G ${this.data.accel_minus_g}
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
            Sensor Packet
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
    public sensor: string = "";

    @property()
    public config: string = "";

    @property()
    public gps: string = "";

    @property()
    public sat: string = "";

    render() {
        return html`
    <div>
        Serial: ${this.serial}
        <br>
        Sensor: ${this.sensor}
        <br>
        Config: ${this.config}
        <br>
        Gps: ${this.gps}
        <br>
        Sat: ${this.sat}
    </div>
        `
    }
}


const view = new TeleMegaDataView();

view.serial = 1018;

document.body.appendChild(view);

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
                case 9:
                    view.sensor = JSON.stringify(elem);
                    view.sensor.requestUpdate();
                    break;
                case 4:
                    view.config = JSON.stringify(elem);
                    view.config.requestUpdate();
                    break;
                case 5:
                    view.gps = JSON.stringify(elem);
                    view.gps.requestUpdate();
                    break;
                case 6:
                    view.sat = JSON.stringify(elem);
                    view.sat.requestUpdate();
                    break;
            }
            console.log(elem);
        }
        view.requestUpdate();
    }
}, 300)