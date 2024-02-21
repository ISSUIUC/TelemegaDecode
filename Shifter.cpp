//
// Created by 16182 on 2/20/2024.
//

#include "Shifter.h"
#include<vector>
#include<iostream>
#include<limits>

constexpr float PI = 3.141592653589793238462643383279502884;
void Shifter::shift(std::complex<float> *x, size_t len, uint64_t start) {
    size_t off = start%shift_buff.size();

    size_t i = 0;
    for(size_t j = off; j < shift_buff.size(); j++){
        x[i++] *= shift_buff.at(j);
        if(i == len) return;
    }

    size_t fulls = (len - (shift_buff.size() - off)) / shift_buff.size();
    for(size_t full = 0; full < fulls; full++){
        for(auto & j : shift_buff){
            x[i++] *= j;
        }
    }

    size_t rest = len - ((shift_buff.size() - off) + fulls * shift_buff.size());
    for(size_t j = 0; j < rest; j++) {
        x[i++] *= shift_buff.at(j);
    }
}

Shifter::Shifter(double freq, double hz): freq(freq), hz(hz) {
    int best_repeat_ct = 1;
    double best_freq_diff = std::numeric_limits<double>::infinity();

    for(int sin_repeat_ct = 1; sin_repeat_ct < 10; sin_repeat_ct++){
        double signed_sin_len = std::round(hz/freq*sin_repeat_ct) / sin_repeat_ct;
        if(signed_sin_len == 0) break;
        double diff = (hz / signed_sin_len) - freq;
        if(std::abs(diff) < std::abs(best_freq_diff)){
            best_freq_diff = diff;
            best_repeat_ct = sin_repeat_ct;
        }
    }

    if(best_freq_diff != 0.0){
        std::cerr << "Shift frequency change (" << freq << " -> " << freq + best_freq_diff << ")\n";
    }

    double signed_sin_len = std::round(hz/freq*best_repeat_ct) / best_repeat_ct;
    if(signed_sin_len == 0){
        signed_sin_len = 1;
    };
    uint64_t sin_len = (uint64_t)std::abs(signed_sin_len);
    shift_buff.resize(sin_len);
    for(uint64_t i = 0; i < sin_len; i++){
        shift_buff[i] = std::complex<float>(
            std::cos(2*PI*(1.f/(float)(signed_sin_len*best_repeat_ct))*i),
            std::sin(2*PI*(1.f/(float)(signed_sin_len*best_repeat_ct))*i)
        );
    }
}

Shifter::Shifter() {
    shift_buff.resize(1);
    freq = 0;
    hz = 0;
};
