#include"StreamingGFSK.h"
#include<iostream>
#include<cmath>

using cf = std::complex<float_type>;

template<int N>
static void sosfilt(const cf sos[N][6], cf* x, size_t len, cf zi[N][2]) {
    // iterate over every i sample section
    for (size_t i = 0; i < len; ++i)
    {
        cf x_c = x[i];
        // iterate over every j section sample
        for (size_t j = 0; j < N; ++j)
        {
            const cf* section = sos[j];
            cf* zi_n = zi[j];
            cf x_n = section[0] * x_c + zi_n[0];
            zi_n[0] = section[1] * x_c - section[4] * x_n + zi_n[1];
            zi_n[1] = section[2] * x_c - section[5] * x_n;
            x_c = x_n;
        }
        x[i] = x_c;
    }
}

constexpr float PI = 3.141592653589793238462643383279502884;

static void shift(cf* x, size_t len, size_t start, double freq, double hz){
    double signed_sin_len = std::round(hz/freq);
    uint64_t sin_len = (uint64_t)std::abs(signed_sin_len);
    if(hz/signed_sin_len != freq){
        std::cerr << "Warning, shift freq changed (" << freq << ") -> (" << hz/signed_sin_len << ")\n";
    }
    std::vector<cf> sin_buff(sin_len);
    for(uint64_t i = 0; i < sin_len; i++){
        sin_buff[i] = cf(
            std::sin(2*PI*(1.f/signed_sin_len)*(i+(start % sin_len))), 
            std::cos(2*PI*(1.f/signed_sin_len)*(i+(start % sin_len)))
        );
    }
    for(size_t i = 0; i < len; i += sin_len){
        for(size_t j = 0; j < sin_len && i + j < len; j++){
            x[i+j] *= sin_buff[j];
        }
    }
}

static void polar_discriminate(cf* x, size_t len, const cf* prev, size_t off) {
    for(size_t i = len - 1; i >= off; i--){
        x[i] = x[i-off] * std::conj(x[i]);
    }
    for(size_t i = 0; i < off; i++){
        x[i] = prev[i] * std::conj(x[i]);
    }
}

static void bool_map(const cf* x, size_t len, bool* out, bool (*func)(cf)) {
    for(size_t i = 0; i < len; i++){
        out[i] = func(x[i]);
    }
}

static void transitions(const bool* x, size_t len, size_t start, bool prev, std::function<void(Transition)>& out) {
    if(len == 0) return;
    std::vector<Transition> transitions;
    if(prev != x[0]) {
        out({start, x[0]});
    }
    for(size_t i = 0; i + 1 < len; i++){
        if(x[i] != x[i+1]){
            out({i+1+start, x[i+1]});
        }
    }
}

static constexpr cf LOW_PASS_SOS[3][6] = {
    {2.91377410754218e-11,5.82754821508436e-11,2.91377410754218e-11,1.0,-0.9844141274160969,0.0,},
    {1.0,2.0,1.0,1.0,-1.9746602956354231,0.9749039346327976,},
    {1.0,1.0,0.0,1.0,-1.990093692505087,0.9903392357172509,},
};

StreamingGFSKDecoder::StreamingGFSKDecoder(
        double sample_rate,
        double center,
        std::function<void(Transition)> out
    ): sample_rate(sample_rate), center(center), out(std::move(out)) {
        off = (size_t)std::abs(sample_rate / center / 4);
        buffer_size = 1024*32;
        prev_samps.resize(off);
        staging.resize(off);
        buffer.resize(buffer_size);
        bits = std::make_unique<bool[]>(buffer_size);
        prev_bit = false;
        buffer_idx = 0;
        total_idx = 0;
    }

void StreamingGFSKDecoder::process_buffer() {
    if(buffer_idx < off){
        std::cerr << "Error, buffer too small";
        return;
    }
    shift(buffer.data(), buffer_idx, total_idx, center, sample_rate);
    sosfilt<3>(LOW_PASS_SOS, buffer.data(), buffer_idx, zi);
    std::copy(buffer.begin() + buffer_idx - off, buffer.begin() + buffer_idx, staging.begin());
    polar_discriminate(buffer.data(), buffer_idx, prev_samps.data(), off);
    std::swap(prev_samps, staging);
    bool_map(buffer.data(), buffer_idx, bits.get(), [](cf x){return x.imag() < 0.0;});
    transitions(bits.get(), buffer_idx, total_idx, prev_bit, out);
    prev_bit = bits[buffer_idx-1];
    total_idx += buffer_idx;
    buffer_idx = 0;
}