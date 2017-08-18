#include "arch.h"
#include "fp_512PM.h"

/* NUMS 512-bit modulus */


#if CHUNK==16

#error Not supported

#endif

#if CHUNK==32
// Base Bits= 29
const BIG_512_29 Modulus_512PM= {0x1FFFFDC7,0x1FFFFFFF,0x1FFFFFFF,0x1FFFFFFF,0x1FFFFFFF,0x1FFFFFFF,0x1FFFFFFF,0x1FFFFFFF,0x1FFFFFFF,0x1FFFFFFF,0x1FFFFFFF,0x1FFFFFFF,0x1FFFFFFF,0x1FFFFFFF,0x1FFFFFFF,0x1FFFFFFF,0x1FFFFFFF,0x7FFFF};
const BIG_512_29 R2modp_512PM= {0xB100000,0x278,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0};
const chunk MConst_512PM= 0x239;
#endif

#if CHUNK==64
// Base Bits= 56
const BIG_512_56 Modulus_512PM= {0xFFFFFFFFFFFDC7L,0xFFFFFFFFFFFFFFL,0xFFFFFFFFFFFFFFL,0xFFFFFFFFFFFFFFL,0xFFFFFFFFFFFFFFL,0xFFFFFFFFFFFFFFL,0xFFFFFFFFFFFFFFL,0xFFFFFFFFFFFFFFL,0xFFFFFFFFFFFFFFL,0xFFL};
const BIG_512_56 R2modp_512PM= {0x0L,0xF0B10000000000L,0x4L,0x0L,0x0L,0x0L,0x0L,0x0L,0x0L,0x0L};
const chunk MConst_512PM= 0x239L;
#endif