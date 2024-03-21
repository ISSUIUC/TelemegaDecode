//
// Created by 16182 on 2/21/2024.
//

#include "StreamSplitter.h"
#include<algorithm>
#include <iostream>
#include <mutex>

bool StreamSplitter::push(std::vector<std::complex<float>> samps){
    std::unique_lock l(lock);
    if(stop_idx != (uint64_t)-1) return false;
    blocks.push_back(std::move(samps));
    l.unlock();
    new_block.notify_all();
    return true;;
}

std::vector<std::complex<float>> const& StreamSplitter::get_block(size_t consumer_idx){
    std::unique_lock l(lock);

    if(consumer_positions[consumer_idx] >= stop_idx) return terminator_block;
    size_t off = consumer_positions[consumer_idx] - base;

    while(off >= blocks.size()){
        new_block.wait(l);
        off = consumer_positions[consumer_idx] - base;
        if(consumer_positions[consumer_idx] >= stop_idx) return terminator_block;
    }
    return blocks[off];
}

void StreamSplitter::next_block(size_t consumer_idx){
    std::unique_lock l(lock);
    consumer_positions[consumer_idx]++;
    size_t min = *std::min_element(consumer_positions.begin(), consumer_positions.end());
    if(min != base){
        base++;
        blocks.pop_front();
    }
}

void StreamSplitter::close() {
    std::unique_lock l(lock);
    if(stop_idx != (uint64_t)-1) return;
    stop_idx = base + blocks.size();
    l.unlock();
    new_block.notify_all();
}
