#pragma once

#include<complex>
#include<cstdint>
#include<vector>

class Shifter {
public:
    Shifter(double freq, double hz);
    Shifter();
    void shift(std::complex<float>* x, size_t len, size_t start);

private:
    std::vector<std::complex<float>> shift_buff;
    double freq;
    double hz;

};


