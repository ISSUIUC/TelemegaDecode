#pragma once

#include<complex>
#include<memory>
#include<functional>
#include<vector>
#include"Transition.h"
#include"Shifter.h"

using float_type = float;

class StreamingGFSKDecoder {
public:
    StreamingGFSKDecoder(
        double sample_rate,
        double center,
        std::function<void(Transition)> out
    );

    void next(std::complex<float>* x, size_t len) {
        for(size_t i = 0; i < len; i++){
            buffer[buffer_idx++] = x[i];
            if(buffer_idx == buffer_size){
                process_buffer();
            }
        }
    }

    void flush_buffer() {
        if(buffer_idx > off){
            process_buffer();
        }
    }

private:
    void process_buffer();

    std::vector<std::complex<float_type>> buffer{};
    std::unique_ptr<bool[]> bits{};
    size_t buffer_idx{};
    uint64_t total_idx{};
    size_t buffer_size{};
    std::vector<std::complex<float_type>> prev_samps{};
    std::vector<std::complex<float_type>> staging{};
    bool prev_bit{};
    std::complex<float_type> zi[3][2] = {};
    Shifter shifter;
    size_t off{};
    std::function<void(Transition)> out;
};