#pragma once

#include<cstdint>
#include<vector>
#include<functional>
#include<array>

constexpr size_t SYNC_PATTERN_LEN = 34;
constexpr size_t MESSAGE_LEN = 72*8;

enum class FecDecoderState {
    Syncing,
    Buffering,
};

struct Packet {
    bool crc_match;
    std::array<uint8_t, 34> data;
};

class StreamingFecDecoder {
public:
    StreamingFecDecoder(std::function<void(Packet)> out);

    void next(bool sample);
private:
    void process_message();

    char sync_buffer[SYNC_PATTERN_LEN]{};
    uint8_t message_buffer[MESSAGE_LEN]{};
    size_t message_head = 0;
    FecDecoderState state = FecDecoderState::Syncing;
    std::function<void(Packet)> out;
};