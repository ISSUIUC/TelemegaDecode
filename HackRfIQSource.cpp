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

#define CHECK(x) do { if((x) != HACKRF_SUCCESS) { std::cerr << "ERROR " << __LINE__ << '\n'; exit(1); } } while(0)

int rx_callback(hackrf_transfer* transfer) {
    HackRfIQSource* sink = (HackRfIQSource*)transfer->rx_ctx;
    return sink->next_transfer(transfer);
}

HackRfIQSource::HackRfIQSource() {
    CHECK(hackrf_init());
    device = nullptr;
    CHECK(hackrf_open_by_serial(nullptr, &device));
    CHECK(hackrf_set_sample_rate(device, 20000000));
    CHECK(hackrf_set_hw_sync_mode(device, 0));
    CHECK(hackrf_set_freq(device, 434850000));
    CHECK(hackrf_set_amp_enable(device, 1));
    CHECK(hackrf_set_vga_gain(device, 14));
    CHECK(hackrf_set_lna_gain(device, 24));
    CHECK(hackrf_start_rx(device, rx_callback, this));
    std::cerr << "RX start\n";
}

#include<thread>
size_t HackRfIQSource::read(IQ *buff, size_t len) {
    for(int tries = 0; tries < 100; tries++){
        {
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
}

int HackRfIQSource::next_transfer(hackrf_transfer *transfer) {
    std::vector<IQ> buff(transfer->valid_length / sizeof(IQ));
    std::memcpy(buff.data(), transfer->buffer, buff.size() * sizeof(IQ));
    std::lock_guard l(lock);
    backlog.push_back(std::move(buff));
    if(backlog.size() > 100){
        std::cerr << "LARGE BACKLOG\n";
    }
    return 0;
}
