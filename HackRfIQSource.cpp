//
// Created by 16182 on 2/19/2024.
//

#include "HackRfIQSource.h"
#include<iostream>
#include<fstream>
#include<cstring>

/*
 * sample rate = 20000000
 * sync_mode = 0
 * auto freq = 434850000
 * amp = 1
 * vga, lna = 10 16
 */

constexpr size_t SAMPLE_RATE = 20000000;
constexpr int MAX_GAIN_SETTING = 36;
constexpr int MIN_GAIN_SETTING = 0;

#define CHECK(x) do { int res = (x); if(res != HACKRF_SUCCESS) {  \
std::cerr << "ERROR (" << hackrf_error_name((hackrf_error)res) << ") " << __FILE__ << ':' << __LINE__ << '\n';   \
exit(1);                                                        \
} } while(0)

int rx_callback(hackrf_transfer* transfer) {
    HackRfIQSource* sink = (HackRfIQSource*)transfer->rx_ctx;
    return sink->next_transfer(transfer);
}

std::pair<int,int> gain_index(int idx){
    if(idx < 0) return {0,0};
    if(idx <= 5) return {idx*8,0};
    if(idx <= 5 + 31) return {40, (idx - 5) * 2};
    return {40, 62};
}

HackRfIQSource::HackRfIQSource(uint64_t center) {
    CHECK(hackrf_init());
    device = nullptr;
    CHECK(hackrf_open_by_serial(nullptr, &device));
    CHECK(hackrf_set_sample_rate(device, SAMPLE_RATE));
    CHECK(hackrf_set_hw_sync_mode(device, 0));
    CHECK(hackrf_set_freq(device, center));
    CHECK(hackrf_set_amp_enable(device, 1));
    CHECK(hackrf_set_vga_gain(device, 0));
    CHECK(hackrf_set_lna_gain(device, 0));
    CHECK(hackrf_start_rx(device, rx_callback, this));
    amp_adjust_time = SAMPLE_RATE;
    max_iq_reading = 0;
    new_gain_setting = current_gain_setting = 0;
    std::cerr << "RX start\n";
}

#include<thread>
size_t HackRfIQSource::read(IQ *buff, size_t len) {
    for(int tries = 0; tries < 30; tries++){
        {
            if(new_gain_setting != current_gain_setting){
                auto [lna,vga] = gain_index(new_gain_setting);
                std::cerr << "A";
                CHECK(hackrf_stop_rx(device));
                std::cerr << "B";
                CHECK(hackrf_set_lna_gain(device, lna));
                std::cerr << "C";
                CHECK(hackrf_set_vga_gain(device, vga));
                std::cerr << "D";
                CHECK(hackrf_start_rx(device, rx_callback, this));
                current_gain_setting = new_gain_setting;
                std::cerr << "Gain set: [" << lna << ',' << vga << "]\n";
            }
            std::lock_guard l(lock);
            if(!backlog.empty()){
                auto& block = backlog.front();
                if(block.size() > len){
                    std::copy(block.begin(), block.begin() + len, buff);
                    block.erase(block.begin(), block.begin() + len);
                    return len;
                } else {
                    std::copy(block.begin(), block.end(), buff);
                    size_t size = block.size();
                    backlog.pop_front();
                    return size;
                }
            }
        }
        std::this_thread::sleep_for(std::chrono::milliseconds(10));
    }
    return 0;
}

HackRfIQSource::~HackRfIQSource() {
    CHECK(hackrf_close(device));
    CHECK(hackrf_exit());
}

int HackRfIQSource::next_transfer(hackrf_transfer *transfer) {
    std::vector<IQ> buff(transfer->valid_length / sizeof(IQ));
    std::memcpy(buff.data(), transfer->buffer, buff.size() * sizeof(IQ));

    for(int i = 0; i < transfer->valid_length; i++){
        max_iq_reading = std::max(max_iq_reading, (int8_t)transfer->buffer[i]);
    }

    std::lock_guard l(lock);
    backlog.push_back(std::move(buff));

    if(backlog.size() > 100){
        std::cerr << "LARGE BACKLOG\n";
    }

    amp_adjust_time -= transfer->valid_length / sizeof(IQ);
    if(amp_adjust_time <= 0){
        if(max_iq_reading < 40){
            new_gain_setting = std::min(current_gain_setting + 1, MAX_GAIN_SETTING);
        }
        if(max_iq_reading > 90){
            new_gain_setting = std::max(current_gain_setting - 1, MIN_GAIN_SETTING);
        }
        std::cerr << (int)max_iq_reading << '\n';
        max_iq_reading = 0;
        amp_adjust_time = SAMPLE_RATE;
        if(new_gain_setting != current_gain_setting) {
            return 1;
        }
    }
    return 0;
}
