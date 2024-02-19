#pragma once

#include<cstdint>

#define AO_FEC_DECODE_CRC_OK 0x80

uint8_t ao_fec_encode(const uint8_t *in, uint8_t len, uint8_t *out);
uint8_t ao_fec_decode(const uint8_t *in, uint16_t len, uint8_t *out, uint8_t out_len, uint16_t (*callback)(void));
uint16_t ao_fec_crc(const uint8_t *bytes, uint8_t len);