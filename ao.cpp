#include"ao.h"

#define NUM_STATE 8
#define AO_FEC_CRC_INIT 0xffff

using bits_t = uint32_t;

#define V_0		0xff
#define V_1		0x00
#define NUM_HIST 24

const uint8_t ao_fec_whiten_table[] = {
 /*   1 */ 0xff,
 /*   2 */ 0xe1,
 /*   3 */ 0x1d,
 /*   4 */ 0x9a,
 /*   5 */ 0xed,
 /*   6 */ 0x85,
 /*   7 */ 0x33,
 /*   8 */ 0x24,
 /*   9 */ 0xea,
 /*  10 */ 0x7a,
 /*  11 */ 0xd2,
 /*  12 */ 0x39,
 /*  13 */ 0x70,
 /*  14 */ 0x97,
 /*  15 */ 0x57,
 /*  16 */ 0x0a,
 /*  17 */ 0x54,
 /*  18 */ 0x7d,
 /*  19 */ 0x2d,
 /*  20 */ 0xd8,
 /*  21 */ 0x6d,
 /*  22 */ 0x0d,
 /*  23 */ 0xba,
 /*  24 */ 0x8f,
 /*  25 */ 0x67,
 /*  26 */ 0x59,
 /*  27 */ 0xc7,
 /*  28 */ 0xa2,
 /*  29 */ 0xbf,
 /*  30 */ 0x34,
 /*  31 */ 0xca,
 /*  32 */ 0x18,
 /*  33 */ 0x30,
 /*  34 */ 0x53,
 /*  35 */ 0x93,
 /*  36 */ 0xdf,
 /*  37 */ 0x92,
 /*  38 */ 0xec,
 /*  39 */ 0xa7,
 /*  40 */ 0x15,
 /*  41 */ 0x8a,
 /*  42 */ 0xdc,
 /*  43 */ 0xf4,
 /*  44 */ 0x86,
 /*  45 */ 0x55,
 /*  46 */ 0x4e,
 /*  47 */ 0x18,
 /*  48 */ 0x21,
 /*  49 */ 0x40,
 /*  50 */ 0xc4,
 /*  51 */ 0xc4,
 /*  52 */ 0xd5,
 /*  53 */ 0xc6,
 /*  54 */ 0x91,
 /*  55 */ 0x8a,
 /*  56 */ 0xcd,
 /*  57 */ 0xe7,
 /*  58 */ 0xd1,
 /*  59 */ 0x4e,
 /*  60 */ 0x09,
 /*  61 */ 0x32,
 /*  62 */ 0x17,
 /*  63 */ 0xdf,
 /*  64 */ 0x83,
 /*  65 */ 0xff,
 /*  66 */ 0xf0,
 /*  67 */ 0x0e,
 /*  68 */ 0xcd,
 /*  69 */ 0xf6,
 /*  70 */ 0xc2,
 /*  71 */ 0x19,
 /*  72 */ 0x12,
 /*  73 */ 0x75,
 /*  74 */ 0x3d,
 /*  75 */ 0xe9,
 /*  76 */ 0x1c,
 /*  77 */ 0xb8,
 /*  78 */ 0xcb,
 /*  79 */ 0x2b,
 /*  80 */ 0x05,
 /*  81 */ 0xaa,
 /*  82 */ 0xbe,
 /*  83 */ 0x16,
 /*  84 */ 0xec,
 /*  85 */ 0xb6,
 /*  86 */ 0x06,
 /*  87 */ 0xdd,
 /*  88 */ 0xc7,
 /*  89 */ 0xb3,
 /*  90 */ 0xac,
 /*  91 */ 0x63,
 /*  92 */ 0xd1,
 /*  93 */ 0x5f,
 /*  94 */ 0x1a,
 /*  95 */ 0x65,
 /*  96 */ 0x0c,
 /*  97 */ 0x98,
 /*  98 */ 0xa9,
 /*  99 */ 0xc9,
 /* 100 */ 0x6f,
 /* 101 */ 0x49,
 /* 102 */ 0xf6,
 /* 103 */ 0xd3,
 /* 104 */ 0x0a,
 /* 105 */ 0x45,
 /* 106 */ 0x6e,
 /* 107 */ 0x7a,
 /* 108 */ 0xc3,
 /* 109 */ 0x2a,
 /* 110 */ 0x27,
 /* 111 */ 0x8c,
 /* 112 */ 0x10,
 /* 113 */ 0x20,
 /* 114 */ 0x62,
 /* 115 */ 0xe2,
 /* 116 */ 0x6a,
 /* 117 */ 0xe3,
 /* 118 */ 0x48,
 /* 119 */ 0xc5,
 /* 120 */ 0xe6,
 /* 121 */ 0xf3,
 /* 122 */ 0x68,
 /* 123 */ 0xa7,
 /* 124 */ 0x04,
 /* 125 */ 0x99,
 /* 126 */ 0x8b,
 /* 127 */ 0xef,
 /* 128 */ 0xc1,
};

static const uint8_t ao_fec_decode_table[NUM_STATE*2] = {
	V_0, V_0,	/* 000 */
	V_0, V_1,	/* 001 */
	V_1, V_1,	/* 010 */
	V_1, V_0,	/* 011 */
	V_1, V_1,	/* 100 */
	V_1, V_0,	/* 101 */
	V_0, V_0,	/* 110 */
	V_0, V_1	/* 111 */
};

static const uint8_t ao_interleave_order[] = {
	0x1e, 0x16, 0x0e, 0x06,
	0x1c, 0x14, 0x0c, 0x04,
	0x1a, 0x12, 0x0a, 0x02,
	0x18, 0x10, 0x08, 0x00
};



#define AO_FEC_PREPARE_EXTRA 4
#define AO_FEC_TRELLIS_TERMINATOR 0x0b

static inline uint16_t ao_interleave_index(uint16_t i) {
	return (uint16_t) ((i & ~0x1e) | ao_interleave_order[(i & 0x1e) >> 1]);
}

static inline uint16_t
ao_fec_crc_byte(uint8_t byte, uint16_t crc)
{
	uint8_t	bit;

	for (bit = 0; bit < 8; bit++) {
		if (((crc & 0x8000) >> 8) ^ (byte & 0x80))
			crc = (uint16_t) ((crc << 1) ^ 0x8005);
		else
			crc = (crc << 1);
		byte = (uint8_t) (byte << 1);
	}
	return crc;
}

uint16_t
ao_fec_crc(const uint8_t *bytes, uint8_t len)
{
	uint16_t	crc = AO_FEC_CRC_INIT;

	while (len--)
		crc = ao_fec_crc_byte(*bytes++, crc);
	return crc;
}

/*
 * Compute CRC and trellis-terminator/interleave-pad bytes
 */
static uint8_t
ao_fec_prepare(const uint8_t *in, uint8_t len, uint8_t *extra)
{
	uint16_t	crc = ao_fec_crc (in, len);
	uint8_t		i = 0;
	uint8_t		num_fec;

	/* Append CRC */
	extra[i++] = (uint8_t) (crc >> 8);
	extra[i++] = (uint8_t) crc;

	/* Append FEC -- 1 byte if odd, two bytes if even */
	num_fec = 2 - (i & 1);
	while (num_fec--)
		extra[i++] = AO_FEC_TRELLIS_TERMINATOR;
	return i;
}



uint8_t
ao_fec_decode(const uint8_t *in, uint16_t len, uint8_t *out, uint8_t out_len, uint16_t (*callback)(void))
{
	static uint32_t	cost[2][NUM_STATE];		/* path cost */
	static bits_t	bits[2][NUM_STATE];		/* save bits to quickly output them */

	uint16_t	i;				/* input byte index */
	uint16_t	b;				/* encoded symbol index (bytes/2) */
	uint16_t	o;				/* output bit index */
	uint8_t		p;				/* previous cost/bits index */
	uint8_t		n;				/* next cost/bits index */
	uint8_t		state;				/* state index */
	const uint8_t	*whiten = ao_fec_whiten_table;
	uint16_t	interleave;			/* input byte array index */
	uint8_t		s0, s1;
	uint16_t	avail;
	uint16_t	crc = AO_FEC_CRC_INIT;
#if AO_PROFILE
	uint32_t	start_tick;
#endif

	p = 0;
	for (state = 0; state < NUM_STATE; state++) {
		cost[0][state] = 0x7fffffff;
		bits[0][state] = 0;
	}
	cost[0][0] = 0;

	if (callback)
		avail = 0;
	else
		avail = len;

#if AO_PROFILE
	if (!avail) {
		avail = callback();
		if (!avail)
			return 0;
	}
	start_tick = ao_profile_tick();
#endif
	o = 0;
	for (i = 0; i < len; i += 2) {
		b = i/2;
		n = p ^ 1;

		if (!avail) {
			avail = callback();
			if (!avail)
				return 0;
		}

		/* Fetch one pair of input bytes, de-interleaving
		 * the input.
		 */
		interleave = ao_interleave_index(i);
		s0 = in[interleave];
		s1 = in[interleave+1];

		avail -= 2;

		/* Compute path costs and accumulate output bit path
		 * for each state and encoded bit value. Unrolling
		 * this loop is worth about > 30% performance boost.
		 * Decoding 76-byte remote access packets is reduced
		 * from 14.700ms to 9.3ms. Redoing the loop to
		 * directly compare the two pasts for each future state
		 * reduces this down to 5.7ms
		 */

		/* Ok, of course this is tricky, it's optimized.
		 *
		 * First, it's important to realize that we have 8
		 * states representing the combinations of the three
		 * most recent bits from the encoder. Flipping any
		 * of these three bits flips both output bits.
		 *
		 * 'state<<1' represents the target state for a new
		 * bit value of 0. '(state<<1)+1' represents the
		 * target state for a new bit value of 1.
		 *
		 * 'state' is the previous state with an oldest bit
		 * value of 0. 'state + 4' is the previous state with
		 * an oldest bit value of 1. These two states will
		 * either lead to 'state<<1' or '(state<<1)+1', depending
		 * on whether the next encoded bit was a zero or a one.
		 *
		 * m0 and m1 are the cost of coming to 'state<<1' from
		 * one of the two possible previous states 'state' and
		 * 'state + 4'.
		 *
		 * Because we know the expected values of each
		 * received bit are flipped between these two previous
		 * states:
		 * 
		 * 	bitcost(state+4) = 510 - bitcost(state)
		 *
		 * With those two total costs in hand, we then pick
		 * the lower as the cost of the 'state<<1', and compute
		 * the path of bits leading to that state.
		 *
		 * Then, do the same for '(state<<1) + 1'. This time,
		 * instead of computing the m0 and m1 values from
		 * scratch, because the only difference is that we're
		 * expecting a one bit instead of a zero bit, we just
		 * flip the bitcost values around to match the
		 * expected transmitted bits with some tricky
		 * arithmetic which is equivalent to:
		 *
		 *	m0 = cost[p][state] + (510 - bitcost);
		 *	m1 = cost[p][state+4] + bitcost
		 *
		 * Then, the lowest cost and bit trace of the new state
		 * is saved.
		 */

#define DO_STATE(state) {						\
			uint32_t	bitcost;			\
									\
			uint32_t	m0;				\
			uint32_t	m1;				\
			uint32_t	bit;				\
									\
			bitcost = ((uint32_t) (s0 ^ ao_fec_decode_table[(state<<1)]) + \
				   (uint32_t) (s1 ^ ao_fec_decode_table[(state<<1)|1])); \
									\
			m0 = cost[p][state] + bitcost;			\
			m1 = cost[p][state+4] + (510 - bitcost);	\
			bit = m0 > m1;					\
			cost[n][state<<1] = bit ? m1 : m0;		\
			bits[n][state<<1] = (bits[p][state + (bit<<2)] << 1) | (state&1); \
									\
			m0 -= (bitcost+bitcost-510);			\
			m1 += (bitcost+bitcost-510);			\
			bit = m0 > m1;					\
			cost[n][(state<<1)+1] = bit ? m1 : m0;		\
			bits[n][(state<<1)+1] = (bits[p][state + (bit<<2)] << 1) | (state&1); \
		}

		DO_STATE(0);
		DO_STATE(1);
		DO_STATE(2);
		DO_STATE(3);

#if 0
		printf ("bit %3d symbol %2x %2x:", i/2, s0, s1);
		for (state = 0; state < NUM_STATE; state++) {
			printf (" %8u(%08x)", cost[n][state], bits[n][state]);
		}
		printf ("\n");
#endif
		p = n;

		/* A loop is needed to handle the last output byte. It
		 * won't have any bits of future data to perform full
		 * error correction, but we might as well give the
		 * best possible answer anyways.
		 */
		while ((b - o) >= (8 + NUM_HIST) || (i + 2 >= len && b > o)) {

			/* Compute number of bits to the end of the
			 * last full byte of data. This is generally
			 * NUM_HIST, unless we've reached
			 * the end of the input, in which case
			 * it will be seven.
			 */
			int8_t		dist = (int8_t) (b - (o + 8));	/* distance to last ready-for-writing bit */
			uint32_t	min_cost;		/* lowest cost */
			uint8_t		min_state;		/* lowest cost state */
			uint8_t		byte;

			/* Find the best fit at the current point
			 * of the decode.
			 */
			min_cost = cost[p][0];
			min_state = 0;
			for (state = 1; state < NUM_STATE; state++) {
				if (cost[p][state] < min_cost) {
					min_cost = cost[p][state];
					min_state = state;
				}
			}

			/* The very last byte of data has the very last bit
			 * of data left in the state value; just smash the
			 * bits value in place and reset the 'dist' from
			 * -1 to 0 so that the full byte is read out
			 */
			if (dist < 0) {
				bits[p][min_state] = (bits[p][min_state] << 1) | (min_state & 1);
				dist = 0;
			}

#if 0
			printf ("\tbit %3d min_cost %5d old bit %3d old_state %x bits %02x whiten %0x\n",
				i/2, min_cost, o + 8, min_state, (bits[p][min_state] >> dist) & 0xff, *whiten);
#endif
			byte = (uint8_t) ((bits[p][min_state] >> dist) ^ *whiten++);
			*out++ = byte;
			if (out_len > 2)
				crc = ao_fec_crc_byte(byte, crc);

			if (!--out_len) {
				if ((out[-2] == (uint8_t) (crc >> 8)) &&
				    out[-1] == (uint8_t) crc)
					out[-1] = AO_FEC_DECODE_CRC_OK;
				else
					out[-1] = 0;
				out[-2] = 0;
				goto done;
			}
			o += 8;
		}
	}
done:
#if AO_PROFILE
	ao_fec_decode_start = start_tick;
	ao_fec_decode_end = ao_profile_tick();
#endif
	return 1;
}


static const uint8_t ao_fec_encode_table[16] = {
/* next 0  1	  state */
	0, 3,	/* 000 */
	1, 2,	/* 001 */
	3, 0,	/* 010 */
	2, 1,	/* 011 */
	3, 0,	/* 100 */
	2, 1,	/* 101 */
	0, 3,	/* 110 */
	1, 2	/* 111 */
};

uint8_t
ao_fec_encode(const uint8_t *in, uint8_t len, uint8_t *out)
{
	uint8_t		extra[AO_FEC_PREPARE_EXTRA];
	uint8_t 	extra_len;
	uint32_t	encode, interleave;
	uint8_t		pair, byte, bit;
	uint16_t	fec = 0;
	const uint8_t	*whiten = ao_fec_whiten_table;

	extra_len = ao_fec_prepare(in, len, extra);
	for (pair = 0; pair < len + extra_len; pair += 2) {
		encode = 0;
		for (byte = 0; byte < 2; byte++) {
			if (pair + byte == len)
				in = extra;
			fec |= (uint16_t) (*in++ ^ *whiten++);
			for (bit = 0; bit < 8; bit++) {
				encode = encode << 2 | ao_fec_encode_table[fec >> 7];
				fec = (fec << 1) & 0x7ff;
			}
		}

		interleave = 0;
		for (bit = 0; bit < 4 * 4; bit++) {
			uint8_t	byte_shift = (bit & 0x3) << 3;
			uint8_t	bit_shift = (bit & 0xc) >> 1;

			interleave = (interleave << 2) | ((encode >> (byte_shift + bit_shift)) & 0x3);
		}
		*out++ = (uint8_t) (interleave >> 24);
		*out++ = (uint8_t) (interleave >> 16);
		*out++ = (uint8_t) (interleave >> 8);
		*out++ = (uint8_t) (interleave >> 0);
	}
	return (uint8_t) ((len + extra_len) * 2);
}
