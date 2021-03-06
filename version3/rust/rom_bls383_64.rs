/*
Licensed to the Apache Software Foundation (ASF) under one
or more contributor license agreements.  See the NOTICE file
distributed with this work for additional information
regarding copyright ownership.  The ASF licenses this file
to you under the Apache License, Version 2.0 (the
"License"); you may not use this file except in compliance
with the License.  You may obtain a copy of the License at

  http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing,
software distributed under the License is distributed on an
"AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
KIND, either express or implied.  See the License for the
specific language governing permissions and limitations
under the License.
*/

use bls383::big::NLEN;
use arch::Chunk;

// Base Bits= 58
// bls383 Modulus
pub const MODULUS:[Chunk;NLEN]=[0x2ACAAB52AAD556B,0x6EC051D7DD75E,0x16CF73083D5D752,0xC814C6083E67AC,0x3852C01355A68EA,0x27DD718417154A9,0x7AC52080A];
pub const R2MODP:[Chunk;NLEN]=[0x21D559796C5EBFB,0x8FDEC2DAAFDED6,0x104E5B315F6AF66,0x34B0FB71D7537D3,0xE63A3C6309B4FC,0x3CC89B133BB5EFA,0x1D66B7E04];
pub const MCONST:Chunk=0xA59AB3B123D0BD;
pub const FRA:[Chunk;NLEN]=[0x32BA59A92B4508B,0x118F6DE81BBBCD0,0x1340341CB1DFBC7,0x35058E7A74CB557,0x398B19B3F05CC36,0x19CBCC8FB9361AA,0x5A5FB198];
pub const FRB:[Chunk;NLEN]=[0x381051A97F904E0,0x2EDF5269BC21A8D,0x38F3EEB8B7DB8A,0x177BBDE60F1B255,0x3EC7A65F6549CB3,0xE11A4F45DDF2FE,0x751F25672];

// bls383 Curve
pub const CURVE_A:isize = 0;
pub const CURVE_B_I:isize = 9;
pub const CURVE_B:[Chunk;NLEN]=[0x9,0x0,0x0,0x0,0x0,0x0,0x0];
pub const CURVE_ORDER:[Chunk;NLEN]=[0xFFF80000FFF001,0x32FF7801C3F9E00,0x3A3000049C5EDF1,0x13310001FE44001,0x1464100,0x0,0x0];
pub const CURVE_GX:[Chunk;NLEN]=[0xD59B348D10786B,0x2CD1DF038FD52B4,0x25BF25B734578B9,0x2713DAB001EEDBD,0x2EFD5830FF57E,0x3262B6E7E23EDBB,0xB08CEE4B];
pub const CURVE_GY:[Chunk;NLEN]=[0x5DA023D145DDB,0x3C4FD46317FBDF3,0x1F56EC3462B2A66,0xFA5BCC0671EA49,0x259906104798122,0x19C412042B63D6F,0x1F3909337];
pub const CURVE_BNX:[Chunk;NLEN]=[0x1000000040,0x44,0x0,0x0,0x0,0x0,0x0];
pub const CURVE_COF:[Chunk;NLEN]=[0x2A00000052B,0x155582AAAAACB28,0x605,0x0,0x0,0x0,0x0];
pub const CURVE_CRU:[Chunk;NLEN]=[0x1A3AAC4EDA155A9,0x177CBFA1D87978F,0x26BCDFAADE63262,0x20D448C4A34C0D6,0x190DBF3A2BBEAD6,0x27DD717EAC81090,0x7AC52080A];
pub const CURVE_PXA:[Chunk;NLEN]=[0x16059885BAC9472,0x1DF134C778B70DB,0x3CBDC90C308C88A,0x19CA7C065C71A23,0xBF3693539C43F1,0x1AFF607969587AE,0x4D50722B5];
pub const CURVE_PXB:[Chunk;NLEN]=[0x9B4BD7A272AB23,0x11EBC6753D1373A,0x3B3F6F7B93206A3,0x3B95C774F8AA067,0x1263A2BA3B635D7,0x396EB0A31E03068,0xEE3617C3];
pub const CURVE_PYA:[Chunk;NLEN]=[0x281D230977BD4FD,0x302D981C837F7F1,0x3A41FC9590C89A0,0x194B87EF3E1E0A1,0x25E011C23014EEE,0x170A21E205AECC,0x8F40859A];
pub const CURVE_PYB:[Chunk;NLEN]=[0xA5E20A252C4CE6,0xD641E9D2BFD032,0x1941760A42448EF,0x127FFBE0AC3F686,0x17BA0F29A18D4EA,0x304AB1FDEE1B926,0x1DCABBA88];
pub const CURVE_W:[[Chunk;NLEN];2]=[[0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0]];
pub const CURVE_SB:[[[Chunk;NLEN];2];2]=[[[0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0]],[[0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0]]];
pub const CURVE_WB:[[Chunk;NLEN];4]=[[0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0]];
pub const CURVE_BB:[[[Chunk;NLEN];4];4]=[[[0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0]],[[0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0]],[[0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0]],[[0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0]]];

pub const USE_GLV:bool = true;
pub const USE_GS_G2:bool = true;
pub const USE_GS_GT:bool = true;
pub const GT_STRONG:bool = false;
