#pragma once

#include<functional>
#include"Transition.h"

class StreamingBitSync {
public:
    StreamingBitSync(double baud_width, std::function<void(bool)> out): state(false), baud_width(baud_width), out(std::move(out)) {}

    void next(Transition sample){
        bool old_state = state;
        double idx = (double)sample.idx;
        state = sample.new_state;

        if(alignment == 0){
            alignment = idx;
            return;
        }

        if(alignment > idx){
            return;
        }

        while(idx > alignment) {
            alignment += baud_width;
            out(old_state);

            if(alignment - baud_width / 2 < idx && idx < alignment + baud_width / 2){
                alignment = alignment * 0.8 + idx * 0.2;
                break;
            }
        }
    }

private:
    bool state;
    double alignment{};
    double baud_width{};
    std::function<void(bool)> out;
};