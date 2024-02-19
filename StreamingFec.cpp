#include"StreamingFec.h"
#include<cstring>
#include<iostream> 
#include"ao.h"

constexpr char SYNC_PATTERN[] = "1010101010101010101101001110010001";
static_assert(sizeof(SYNC_PATTERN) == SYNC_PATTERN_LEN + 1, "Bad Sync Size");

StreamingFecDecoder::StreamingFecDecoder(std::function<void(Packet)> out): out(std::move(out)){

}

void StreamingFecDecoder::next(bool sample){
    if(state == FecDecoderState::Syncing){
        std::memmove(sync_buffer,  sync_buffer + 1, SYNC_PATTERN_LEN - 1);
        sync_buffer[SYNC_PATTERN_LEN-1] = sample ? '1' : '0';

        if(memcmp(sync_buffer, SYNC_PATTERN, SYNC_PATTERN_LEN) == 0){
            state = FecDecoderState::Buffering;
        }
    } else if(state == FecDecoderState::Buffering){
        message_buffer[message_head++] = sample ? 0x00 : 0xff;
        if(message_head == MESSAGE_LEN){
            process_message();
            state = FecDecoderState::Syncing;
            message_head = 0;
        }
    }
}

void StreamingFecDecoder::process_message(){
    uint8_t decoded[MESSAGE_LEN / 16 - 2]{};
    ao_fec_decode(message_buffer, MESSAGE_LEN, decoded, sizeof(decoded), nullptr);
    bool crc_match = decoded[sizeof(decoded)-1] == AO_FEC_DECODE_CRC_OK;
    out(Packet{crc_match, std::vector<uint8_t>(decoded, decoded + sizeof(decoded))});
}