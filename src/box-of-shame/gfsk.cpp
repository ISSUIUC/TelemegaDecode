#include <iostream>
#include <string>
#include <chrono>
#include <thread>
#include <csignal>
#include <algorithm>

#include "StreamingBitSync.h"
#include "StreamingGFSK.h"
#include "StreamingFec.h"

struct IQ {
    int8_t i;
    int8_t q;

    std::complex<float> cf() const {
        return std::complex<float>(i,q);
    }
};

using cf = std::complex<float>;

class FullDecoder {
public:
    FullDecoder(double center, double hz, double baud, size_t id, std::function<void(Packet, size_t)> o) : out(std::move(o)) {
        fec = std::make_shared<StreamingFecDecoder>([=](Packet p){
            out(p, id);
        });
        bit = std::make_shared<StreamingBitSync>(hz/baud, [=](bool b){
            fec->next(b);
        });
        gfsk = std::make_unique<StreamingGFSKDecoder>(hz, center, [=](Transition t){
            bit->next(t);
        });
    }

    void next(const cf* x, size_t len){
        gfsk->next(x, len);
    }

private:
    std::shared_ptr<StreamingFecDecoder> fec;
    std::shared_ptr<StreamingBitSync> bit;
    std::unique_ptr<StreamingGFSKDecoder> gfsk;

    std::function<void(Packet, size_t)> out;
};

double find_center_frequency(std::vector<double> const& freqs){
    if(freqs.empty()) return 0;
    double max = *std::max_element(freqs.begin(),  freqs.end());
    return max + 100000.0;
}

typedef void (*raw_out_func)(void*, Packet, size_t);

extern "C" {
    FullDecoder* create_decoder(double center, double hz, double baud, size_t id, raw_out_func out_func, void* arg) {
        std::function<void(Packet, size_t)> out = [=](Packet p, size_t i) { (out_func)(arg, p, i); };
        return new FullDecoder(center, hz, baud, id, out);
    }

    void feed_into_decoder(FullDecoder* decoder, const cf* buf, size_t len) {
        decoder->next(buf, len);
    }

    void destroy_decoder(FullDecoder* decoder) {
        free(decoder);
    }
}
