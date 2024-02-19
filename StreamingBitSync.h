#pragma once

#include<functional>
#include"Transition.h"

class StreamingBitSync {
public:
    StreamingBitSync(double baud_width, std::function<void(bool)> out): state(false), baud_width(baud_width), out(std::move(out)) {}

    void next(Transition sample){
        bool old_state = state;
        state = sample.new_state;

        if(alignment == 0){
            alignment = sample.idx;
            return;
        }

        if(alignment > sample.idx){
            return;
        }

        while(sample.idx > alignment) {
            alignment += baud_width;
            out(old_state);

            if(alignment - baud_width / 2 < sample.idx && sample.idx < alignment + baud_width / 2){
                alignment = alignment * 0.8 + sample.idx * 0.2;
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