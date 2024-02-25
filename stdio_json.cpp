//
// Created by 16182 on 2/23/2024.
//

#include<nlohmann/json.hpp>
#include"stdio_json.h"

void output_packet(Packet const& p, size_t id){
    nlohmann::json json{
        {"type","packet"},
        {"data",p.data},
        {"crc",p.crc_match},
        {"id",id},
    };
    std::cout << json << std::endl;
}

void output_gain_setting(int lna, int vga){
    nlohmann::json json{
        {"type","gain"},
        {"lna", lna},
        {"vga",vga},
    };
    std::cout << json << std::endl;
}

void output_error(std::string error, const char * file, int line){
    nlohmann::json json{
        {"type","error"},
        {"error", error},
        {"file", file},
        {"line", line},
    };
    std::cout << json << std::endl;
}

void output_center_freq(double center){
    nlohmann::json json{
        {"type","center"},
        {"center", center},
    };
    std::cout << json << std::endl;
}

void output_closed(){
    nlohmann::json json{
        {"type","closed"},
        };
    std::cout << json << std::endl;
}