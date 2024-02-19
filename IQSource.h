#pragma once

#include<cstdint>
#include<fstream>

struct IQ {
    int8_t i;
    int8_t q;
};

class IQSource {
public:
    virtual size_t read(IQ* buff, size_t len) = 0;
};