#include"StreamingBitSync.h"
#include"StreamingGFSK.h"
#include"StreamingFec.h"
#include"FileIQSource.h"
#include"HackRfIQSource.h"

#include<iostream>
#include<string>
#include<chrono>
#include<csignal>

static volatile bool stop = false;

void sigint_handler(int signal){
    (void)signal;
    std::cerr << "INT\n";
    stop = true;
}

void print_packet(const Packet& packet){
    std::cout << '\n';
    if(packet.crc_match){
        std::cout << "CRC OK!\n";
    } else {
        std::cout << "CRC ERROR\n";
    }

    auto data = packet.data;
    std::cout << std::dec;
    std::cout << "Serial Number: " << *((uint16_t*)(data.data())) << '\n';
    std::cout << "Time Stamp: " << *((uint16_t*)(data.data()+2)) << '\n';
    std::cout << "Type: " << (int)data[4] << '\n';

    if(data[4] == 1){ //sensor packet
        int v_batt = *((uint16_t*)(data.data()+12));
        int thermo = *((uint16_t*)(data.data()+10));
        int count = *((uint16_t*)(data.data()+8));
        float volts = v_batt / 32767.0 * 5.0;
        float temp = (thermo - 19791.268) / 32728.0 * 1.25 / 0.00247;
        float pressure = ((count / 16.0) / 2047.0 + 0.095) / 0.009 * 1000.0;
        std::cout << "State: " << (int)data[5] << '\n';
        std::cout << "Accel: " << *((uint16_t*)(data.data()+6)) << '\n';
        std::cout << "Pressure: " << pressure << '\n';
        std::cout << "Temp: " << temp << '\n';
        std::cout << "Volts: " << volts << '\n';
    }

    if(data[4] == 4){ //config packet
        std::cout << "Dev type: " << (int)data[5] << '\n';
        std::cout << "Flight Number: " << (int)data[6] << '\n';
        std::cout << "Config Major: " << (int)data[7] << '\n';
        std::cout << "Config Minor: " << (int)data[8] << '\n';
        std::cout << "Apogee delay: " << *((uint16_t*)(data.data()+10)) << '\n';
        std::cout << "Main delay: " << *((uint16_t*)(data.data()+12)) << '\n';
        std::cout << "Flight log max: " << *((uint16_t*)(data.data()+14)) << '\n';
        std::cout << "Callsign: " << std::string(data.data() + 16, data.data() + 24) << '\n';
        std::cout << "Version: " << std::string(data.data() + 24, data.data() + 32) << '\n';
    }

    if(data[4] == 5) { //gps packet
        std::cout << "Flags: " << (int)data[5] << '\n';
        std::cout << "Altitude: " << *((uint16_t*)(data.data()+6)) << '\n';
        std::cout << "Latitude: " << *((uint32_t*)(data.data()+8)) << '\n';
        std::cout << "Longitude: " << *((uint32_t*)(data.data()+12)) << '\n';
        std::cout << "Year: " << (int)data[16] << '\n';
        std::cout << "Month: " << (int)data[17] << '\n';
        std::cout << "Day: " << (int)data[18] << '\n';
        std::cout << "Hour: " << (int)data[19] << '\n';
        std::cout << "Minute: " << (int)data[20] << '\n';
        std::cout << "Second: " << (int)data[21] << '\n';
        std::cout << "pdop: " << (int)data[22] << '\n';
        std::cout << "hdop: " << (int)data[23] << '\n';
        std::cout << "vdop: " << (int)data[24] << '\n';
        std::cout << "Mode: " << (int)data[25] << '\n';
        std::cout << "Ground Speed: " << *((uint32_t*)(data.data()+26)) << '\n';
    }
}

int main(){
    double hz = 20000000.1;
    double center = -200000;
    double baud = 38400;
    int packet_ct = 0;
    StreamingFecDecoder fec([&](Packet p){
        std::cout << ++packet_ct << '\n';
        (void)p;
        print_packet(p);
    });

    StreamingBitSync syncer = {hz/baud, [&](bool b){
        fec.next(b);
    }};

    StreamingGFSKDecoder decoder = {hz, center, [&](Transition t){
        syncer.next(t);
    }};
    //    FileIQSource src("../tele200.dat");
    HackRfIQSource src;
    std::vector<IQ> buffer(1024*128);
    signal(SIGINT, sigint_handler);


    while(size_t ct = src.read(buffer.data(), buffer.size())){
        if(stop) break;
        for(size_t i = 0; i < ct; i++){
            decoder.next(std::complex<float_type>(buffer[i].i, buffer[i].q));
        }
    }
    std::cerr << "Finish\n";
}