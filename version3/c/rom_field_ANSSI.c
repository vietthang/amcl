#include "arch.h"
#include "fp_ANSSI.h"

/* ANNSI Curve */

#if CHUNK==16
const BIG_256 Modulus_ANSSI= {0x1C03,0x374,0x14F6,0x19E6,0x1E8F,0x536,0xF23,0x1795,0x1AD,0x19CB,0x10D6,0x1C84,0x1E8C,0x936,0x1C40,0x1AB1,0xB3A,0x1C60,0x1F45,0x1E3};
const chunk MConst_ANSSI=0x1155;
#endif

#if CHUNK==32
const BIG_256 Modulus_ANSSI= {0x186E9C03,0x7E79A9E,0x12329B7A,0x35B7957,0x435B396,0x16F46721,0x163C4049,0x1181675A,0xF1FD17};
const chunk MConst_ANSSI=0x164E1155;
#endif

#if CHUNK==64
const BIG_256 Modulus_ANSSI_ANNSI= {0xFCF353D86E9C03,0xADBCABC8CA6DE8,0xE8CE42435B3961,0xB3AD58F10126D,0xF1FD178C};
const chunk MConst_ANSSI_ANNSI=0x97483A164E1155;
#endif
