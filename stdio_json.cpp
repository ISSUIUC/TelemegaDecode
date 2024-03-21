//
// Created by 16182 on 2/23/2024.
//

#include<nlohmann/json.hpp>
#include"stdio_json.h"
#include<mutex>

std::mutex print_guard{};

void output_packet(Packet const& p, size_t id){
    nlohmann::json json{
        {"type","packet"},
        {"data",p.data},
        {"crc",p.crc_match},
        {"id",id},
    };
    std::lock_guard<std::mutex> l(print_guard);
    std::cout << json << std::endl;
}

void output_gain_setting(int lna, int vga){
    nlohmann::json json{
        {"type","gain"},
        {"lna", lna},
        {"vga",vga},
    };
    std::lock_guard<std::mutex> l(print_guard);
    std::cout << json << std::endl;
}

void output_error(std::string error, const char * file, int line){
    nlohmann::json json{
        {"type","error"},
        {"error", error},
        {"file", file},
        {"line", line},
    };
    std::lock_guard<std::mutex> l(print_guard);
    std::cout << json << std::endl;
}

void output_center_freq(double center){
    nlohmann::json json{
        {"type","center"},
        {"center", center},
    };
    std::lock_guard<std::mutex> l(print_guard);
    std::cout << json << std::endl;
}

void output_closed(){
    nlohmann::json json{
        {"type","closed"},
        };
    std::lock_guard<std::mutex> l(print_guard);
    std::cout << json << std::endl;
}