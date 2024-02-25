import { LitElement, css, html } from "lit";
import { customElement, property } from "lit/decorators.js"
import { DecodedPacket, SensorPacket, ConfigPacket, GPSPacket, SatellitePacket } from "../packet";


@customElement('sensor-packet')
export class SensorPacketView extends LitElement {
    @property()
    data: SensorPacket;

    render() {
        return html`
            Sensor Packet ${this.data}
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
    public sensor: SensorPacketView = new SensorPacketView();

    @property()
    public config: ConfigPacketView = new ConfigPacketView();

    @property()
    public gps: GpsPacketView = new GpsPacketView();

    @property()
    public sat: SatellitePacketView = new SatellitePacketView();

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