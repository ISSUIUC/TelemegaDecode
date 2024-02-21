#pragma once

#include "IQSource.h"
#include "hackrf//libhackrf/src/hackrf.h"
#include <cstdio>
#include <list>
#include <mutex>
#include <vector>
#include <optional>
#include <atomic>
class HackRfIQSource : public IQSource {
public:
    HackRfIQSource(uint64_t center);
    HackRfIQSource(HackRfIQSource&) = delete;
    HackRfIQSource& operator==(HackRfIQSource&) = delete;
    HackRfIQSource(HackRfIQSource&&) = delete;
    HackRfIQSource& operator==(HackRfIQSource&&) = delete;

    size_t read(IQ* buff, size_t len) override;
    int next_transfer(hackrf_transfer* transfer);
    ~HackRfIQSource();
private:
    int current_gain_setting;
    int new_gain_setting;
    int64_t amp_adjust_time;
    int8_t max_iq_reading;
    hackrf_device* device;
    std::list<std::vector<IQ>> backlog;
    std::mutex lock;
};

