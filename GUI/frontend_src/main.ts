import { LitElement, css, html } from "lit";
import { customElement, property } from "lit/decorators.js"
import { DecodedPacket, SensorPacket, ConfigPacket, GPSPacket, SatellitePacket } from "../packet";


@customElement('sensor-packet')
export class SensorPacketView extends LitElement {
    @property()
    data: SensorPacket;

    render() {
        return html`
            Sensor Packet
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
    public sensor: SensorPacket;

    @property()
    public config: ConfigPacket;

    @property()
    public gps: GPSPacket;

    @property()
    public sat: SatellitePacket;

    render() {
        return html`
    <div>
        Serial: ${this.serial}
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