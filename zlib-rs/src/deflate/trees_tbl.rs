#![forbid(unsafe_code)]

use crate::deflate::{
    Value, DIST_CODE_LEN, D_CODES, LENGTH_CODES, L_CODES, STD_MAX_MATCH, STD_MIN_MATCH,
};

const fn h(freq: u16, code: u16) -> Value {
    Value::new(freq, code)
}

#[rustfmt::skip]
pub const STATIC_LTREE: [Value; L_CODES + 2] = [
    h( 12,8), h(140,8), h( 76,8), h(204,8), h( 44,8),
    h(172,8), h(108,8), h(236,8), h( 28,8), h(156,8),
    h( 92,8), h(220,8), h( 60,8), h(188,8), h(124,8),
    h(252,8), h(  2,8), h(130,8), h( 66,8), h(194,8),
    h( 34,8), h(162,8), h( 98,8), h(226,8), h( 18,8),
    h(146,8), h( 82,8), h(210,8), h( 50,8), h(178,8),
    h(114,8), h(242,8), h( 10,8), h(138,8), h( 74,8),
    h(202,8), h( 42,8), h(170,8), h(106,8), h(234,8),
    h( 26,8), h(154,8), h( 90,8), h(218,8), h( 58,8),
    h(186,8), h(122,8), h(250,8), h(  6,8), h(134,8),
    h( 70,8), h(198,8), h( 38,8), h(166,8), h(102,8),
    h(230,8), h( 22,8), h(150,8), h( 86,8), h(214,8),
    h( 54,8), h(182,8), h(118,8), h(246,8), h( 14,8),
    h(142,8), h( 78,8), h(206,8), h( 46,8), h(174,8),
    h(110,8), h(238,8), h( 30,8), h(158,8), h( 94,8),
    h(222,8), h( 62,8), h(190,8), h(126,8), h(254,8),
    h(  1,8), h(129,8), h( 65,8), h(193,8), h( 33,8),
    h(161,8), h( 97,8), h(225,8), h( 17,8), h(145,8),
    h( 81,8), h(209,8), h( 49,8), h(177,8), h(113,8),
    h(241,8), h(  9,8), h(137,8), h( 73,8), h(201,8),
    h( 41,8), h(169,8), h(105,8), h(233,8), h( 25,8),
    h(153,8), h( 89,8), h(217,8), h( 57,8), h(185,8),
    h(121,8), h(249,8), h(  5,8), h(133,8), h( 69,8),
    h(197,8), h( 37,8), h(165,8), h(101,8), h(229,8),
    h( 21,8), h(149,8), h( 85,8), h(213,8), h( 53,8),
    h(181,8), h(117,8), h(245,8), h( 13,8), h(141,8),
    h( 77,8), h(205,8), h( 45,8), h(173,8), h(109,8),
    h(237,8), h( 29,8), h(157,8), h( 93,8), h(221,8),
    h( 61,8), h(189,8), h(125,8), h(253,8), h( 19,9),
    h(275,9), h(147,9), h(403,9), h( 83,9), h(339,9),
    h(211,9), h(467,9), h( 51,9), h(307,9), h(179,9),
    h(435,9), h(115,9), h(371,9), h(243,9), h(499,9),
    h( 11,9), h(267,9), h(139,9), h(395,9), h( 75,9),
    h(331,9), h(203,9), h(459,9), h( 43,9), h(299,9),
    h(171,9), h(427,9), h(107,9), h(363,9), h(235,9),
    h(491,9), h( 27,9), h(283,9), h(155,9), h(411,9),
    h( 91,9), h(347,9), h(219,9), h(475,9), h( 59,9),
    h(315,9), h(187,9), h(443,9), h(123,9), h(379,9),
    h(251,9), h(507,9), h(  7,9), h(263,9), h(135,9),
    h(391,9), h( 71,9), h(327,9), h(199,9), h(455,9),
    h( 39,9), h(295,9), h(167,9), h(423,9), h(103,9),
    h(359,9), h(231,9), h(487,9), h( 23,9), h(279,9),
    h(151,9), h(407,9), h( 87,9), h(343,9), h(215,9),
    h(471,9), h( 55,9), h(311,9), h(183,9), h(439,9),
    h(119,9), h(375,9), h(247,9), h(503,9), h( 15,9),
    h(271,9), h(143,9), h(399,9), h( 79,9), h(335,9),
    h(207,9), h(463,9), h( 47,9), h(303,9), h(175,9),
    h(431,9), h(111,9), h(367,9), h(239,9), h(495,9),
    h( 31,9), h(287,9), h(159,9), h(415,9), h( 95,9),
    h(351,9), h(223,9), h(479,9), h( 63,9), h(319,9),
    h(191,9), h(447,9), h(127,9), h(383,9), h(255,9),
    h(511,9), h(  0,7), h( 64,7), h( 32,7), h( 96,7),
    h( 16,7), h( 80,7), h( 48,7), h(112,7), h(  8,7),
    h( 72,7), h( 40,7), h(104,7), h( 24,7), h( 88,7),
    h( 56,7), h(120,7), h(  4,7), h( 68,7), h( 36,7),
    h(100,7), h( 20,7), h( 84,7), h( 52,7), h(116,7),
    h(  3,8), h(131,8), h( 67,8), h(195,8), h( 35,8),
    h(163,8), h( 99,8), h(227,8)
];

#[rustfmt::skip]
pub const STATIC_DTREE: [Value; D_CODES] = [
    h( 0,5), h(16,5), h( 8,5), h(24,5), h( 4,5),
    h(20,5), h(12,5), h(28,5), h( 2,5), h(18,5),
    h(10,5), h(26,5), h( 6,5), h(22,5), h(14,5),
    h(30,5), h( 1,5), h(17,5), h( 9,5), h(25,5),
    h( 5,5), h(21,5), h(13,5), h(29,5), h( 3,5),
    h(19,5), h(11,5), h(27,5), h( 7,5), h(23,5)
];

#[rustfmt::skip]
pub const DIST_CODE: [u8; DIST_CODE_LEN] = [
     0,  1,  2,  3,  4,  4,  5,  5,  6,  6,  6,  6,  7,  7,  7,  7,  8,  8,  8,  8,
     8,  8,  8,  8,  9,  9,  9,  9,  9,  9,  9,  9, 10, 10, 10, 10, 10, 10, 10, 10,
    10, 10, 10, 10, 10, 10, 10, 10, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
    11, 11, 11, 11, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12,
    12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 13, 13, 13, 13,
    13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
    13, 13, 13, 13, 13, 13, 13, 13, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
    14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
    14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
    14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,  0,  0, 16, 17,
    18, 18, 19, 19, 20, 20, 20, 20, 21, 21, 21, 21, 22, 22, 22, 22, 22, 22, 22, 22,
    23, 23, 23, 23, 23, 23, 23, 23, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
    24, 24, 24, 24, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25,
    26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26,
    26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 27, 27, 27, 27, 27, 27, 27, 27,
    27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27,
    27, 27, 27, 27, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28,
    28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28,
    28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28,
    28, 28, 28, 28, 28, 28, 28, 28, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29,
    29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29,
    29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29,
    29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29
];

#[rustfmt::skip]
pub const LENGTH_CODE: [u8; STD_MAX_MATCH-STD_MIN_MATCH+1] = [
     0,  1,  2,  3,  4,  5,  6,  7,  8,  8,  9,  9, 10, 10, 11, 11, 12, 12, 12, 12,
    13, 13, 13, 13, 14, 14, 14, 14, 15, 15, 15, 15, 16, 16, 16, 16, 16, 16, 16, 16,
    17, 17, 17, 17, 17, 17, 17, 17, 18, 18, 18, 18, 18, 18, 18, 18, 19, 19, 19, 19,
    19, 19, 19, 19, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 22, 22, 22, 22,
    22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 23, 23, 23, 23, 23, 23, 23, 23,
    23, 23, 23, 23, 23, 23, 23, 23, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
    24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
    25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25,
    25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 26, 26, 26, 26, 26, 26, 26, 26,
    26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26,
    26, 26, 26, 26, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27,
    27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 28
];

pub const BASE_LENGTH: [u8; LENGTH_CODES] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 10, 12, 14, 16, 20, 24, 28, 32, 40, 48, 56, 64, 80, 96, 112, 128,
    160, 192, 224, 0,
];

#[rustfmt::skip]
pub const BASE_DIST: [u16; D_CODES] = [
    0,     1,     2,     3,     4,     6,     8,    12,    16,    24,
   32,    48,    64,    96,   128,   192,   256,   384,   512,   768,
 1024,  1536,  2048,  3072,  4096,  6144,  8192, 12288, 16384, 24576
];