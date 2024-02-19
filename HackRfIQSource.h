#pragma once

#include "IQSource.h"
#include "hackrf_transfer/libhackrf/src/hackrf.h"
#include <cstdio>
#include <list>
#include <mutex>
#include <vector>
class HackRfIQSource : public IQSource {
public:
    HackRfIQSource();
    HackRfIQSource(HackRfIQSource&) = delete;
    HackRfIQSource& operator==(HackRfIQSource&) = delete;
    HackRfIQSource(HackRfIQSource&&) = delete;
    HackRfIQSource& operator==(HackRfIQSource&&) = delete;

    size_t read(IQ* buff, size_t len) override;
    int next_transfer(hackrf_transfer* transfer);
    ~HackRfIQSource();
private:
    hackrf_device* device;
    std::list<std::vector<IQ>> backlog;
    std::mutex lock;
};

