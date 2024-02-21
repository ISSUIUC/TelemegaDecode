#pragma once

#include<cstdint>
#include<fstream>
#include<complex>

struct IQ {
    int8_t i;
    int8_t q;

    std::complex<float> cf() const {
        return std::complex<float>(i,q);
    }
};

class IQSource {
public:
    virtual size_t read(IQ* buff, size_t len) = 0;
};