#include"StreamingGFSK.h"
#include<iostream>
#include<cmath>

using cf = std::complex<float_type>;

template<int N>
static void sosfilt(const float sos[N][6], cf* x, size_t len, cf zi[N][2]) {
    // iterate over every i sample section
    for (size_t i = 0; i < len; ++i)
    {
        cf x_c = x[i];
        // iterate over every j section sample
        for (size_t j = 0; j < N; ++j)
        {
            const float* section = sos[j];
            cf* zi_n = zi[j];
            cf x_n = section[0] * x_c + zi_n[0];
            zi_n[0] = section[1] * x_c - section[4] * x_n + zi_n[1];
            zi_n[1] = section[2] * x_c - section[5] * x_n;
            x_c = x_n;
        }
        x[i] = x_c;
    }
}


static void sosfilt_fast(const float sos[3][6], cf* x, size_t len, cf zi[3][2]) {
    // iterate over every i sample section
    for (size_t i = 0; i < len; ++i)
    {
        cf x_c_0 = x[i];
        // iterate over every j section sample
        const float* section1 = sos[0];
        const float* section2 = sos[1];
        const float* section3 = sos[2];

        cf* zi_0 = zi[0];
        cf* zi_1 = zi[1];
        cf* zi_2 = zi[2];
        cf cp_zi_0[2];
        cf cp_zi_1[2];
        cf cp_zi_2[2];

        cf x_0  = section1[0] * x_c_0 + zi_0[0];
        cf x_1  = section2[0] * x_0 + zi_1[0];
        cf x_2  = section3[0] * x_1 + zi_2[0];

        cp_zi_0[0] = section1[1] * x_c_0 - section1[4] * x_0 + zi_0[1];
        cp_zi_1[0] = section2[1] * x_0 - section2[4] * x_1 + zi_1[1];
        cp_zi_2[0] = section3[1] * x_1 - section3[4] * x_2 + zi_2[1];

        cp_zi_0[1] = section1[2] * x_c_0 - section1[5] * x_0;
        cp_zi_1[1] = section2[2] * x_0 - section2[5] * x_1;
        cp_zi_2[1] = section3[2] * x_1 - section3[5] * x_2;

        x[i] = x_2;

        zi[0][0] = cp_zi_0[0];
        zi[1][0] = cp_zi_1[0];
        zi[2][0] = cp_zi_2[0];

        zi[0][1] = cp_zi_0[1];
        zi[1][1] = cp_zi_1[1];
        zi[2][1] = cp_zi_2[1];
    }
}

static cf linear_sample(const cf* x, const cf* prev, size_t prev_len, double u){
    if(u >= 0){
        size_t i_u = (size_t)u;
        float fract = u - i_u;

        cf left = x[i_u];
        cf right = x[i_u+1];
        return left * (1 - fract) + right * fract;
    } else if(u >= -1) {
        float fract = u + 1;

        cf left = prev[prev_len-1];
        cf right = x[0];
        return left * (1 - fract) + right * fract;
    } else {
        size_t i_u = (size_t)(-u);
        float fract = -(i_u + u);
        cf left = prev[prev_len - i_u - 1];
        cf right = prev[prev_len - i_u];

        return left * fract + right * (1-fract);
    }
}

static void polar_discriminate(cf* x, size_t len, const cf* prev, size_t prev_len, double off) {
    for(ssize_t i = len - 1; i >= 0; i--){
        x[i] = linear_sample(x, prev, prev_len, i - off) * std::conj(x[i]);
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

//static constexpr float LOW_PASS_SOS_50K[3][6] = {
//    {2.91377410754218e-11,5.82754821508436e-11,2.91377410754218e-11,1.0,-0.9844141274160969,0.0,},
//    {1.0,2.0,1.0,1.0,-1.9746602956354231,0.9749039346327976,},
//    {1.0,1.0,0.0,1.0,-1.990093692505087,0.9903392357172509,},
//};

static constexpr float LOW_PASS_SOS_33K[3][6] = {
    {3.869494405731452e-12,7.738988811462904e-12,3.869494405731452e-12,1.0,-0.989582475318754,0.0,},
    {1.0,2.0,1.0,1.0,-1.98308989599488,0.9831986360344092,},
    {1.0,1.0,0.0,1.0,-1.9934396492520414,0.9935489568062257,},
};

StreamingGFSKDecoder::StreamingGFSKDecoder(
        double sample_rate,
        double center,
        std::function<void(Transition)> out
    ): shifter(-center, sample_rate), out(std::move(out)) {
        off = 50;
        off_buffer_size = 1 + (size_t)std::ceil(std::abs(off));
        buffer_size = 1024*32;
        prev_samps.resize(off_buffer_size);
        staging.resize(off_buffer_size);
        buffer.resize(buffer_size);
        bits = std::make_unique<bool[]>(buffer_size);
        prev_bit = false;
        buffer_idx = 0;
        total_idx = 0;
    }

void StreamingGFSKDecoder::process_buffer() {
    if(buffer_idx < off_buffer_size){
        std::cerr << "Error, buffer too small";
        return;
    }
    shifter.shift(buffer.data(), buffer_idx, total_idx);
    sosfilt_fast(LOW_PASS_SOS_33K, buffer.data(), buffer_idx, zi);


    std::copy(buffer.begin() + buffer_idx - off_buffer_size, buffer.begin() + buffer_idx, staging.begin());
    polar_discriminate(buffer.data(), buffer_idx, prev_samps.data(), prev_samps.size(), off);
    std::swap(prev_samps, staging);
    bool_map(buffer.data(), buffer_idx, bits.get(), [](cf x){return x.imag() < 0.0;});
    transitions(bits.get(), buffer_idx, total_idx, prev_bit, out);
    prev_bit = bits[buffer_idx-1];
    total_idx += buffer_idx;
    buffer_idx = 0;
}