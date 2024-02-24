#include <stdint.h>

void hackrf_get_sample_rate(const double freq, uint32_t* freq_hz, uint32_t* divider)
{
	const int MAX_N = 32;
	double freq_frac = 1.0 + freq - (int) freq;
	uint64_t a, m;
	int i, e;

	union {
		uint64_t u64;
		double d;
	} v;

	v.d = freq;

	e = (v.u64 >> 52) - 1023;

	m = ((1ULL << 52) - 1);

	v.d = freq_frac;
	v.u64 &= m;

	m &= ~((1 << (e + 4)) - 1);

	a = 0;

	for (i = 1; i < MAX_N; i++) {
		a += v.u64;
		if (!(a & m) || !(~a & m))
			break;
	}

	if (i == MAX_N)
		i = 1;

	*freq_hz = (uint32_t) (freq * i + 0.5);
	*divider = i;
}