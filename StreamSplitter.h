//
// Created by 16182 on 2/21/2024.
//

#ifndef GFSK_STREAMSPLITTER_H
#define GFSK_STREAMSPLITTER_H
#include<vector>
#include<deque>
#include<complex>
#include<mutex>
#include<condition_variable>

class StreamSplitter {
public:
    explicit StreamSplitter(size_t consumers) {
        consumer_positions.resize(consumers);
        base = 0;
    }
    bool push(std::vector<std::complex<float>> samps);

    void close();

    std::vector<std::complex<float>> const& get_block(size_t consumer_idx);

    void next_block(size_t consumer_idx);

private:
    std::deque<std::vector<std::complex<float>>> blocks;

    uint64_t base{};
    std::vector<uint64_t> consumer_positions;
    std::mutex lock;
    std::condition_variable new_block;
    std::vector<std::complex<float>> terminator_block;
    uint64_t stop_idx = -1;
};


#endif //GFSK_STREAMSPLITTER_H
