//
// Created by 16182 on 2/23/2024.
//

#ifndef GFSK_STDIO_JSON_H
#define GFSK_STDIO_JSON_H

#include<iostream>
#include"StreamingFec.h"

void output_packet(Packet const& p, size_t id);
void output_gain_setting(int lna, int vga);
void output_error(std::string error, const char * file, int line);
void output_center_freq(double center);
void output_closed();

#endif //GFSK_STDIO_JSON_H
