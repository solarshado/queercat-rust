use crate::{pattern_t, ansii_pattern_t, color_pattern_t, color_pattern};

pub(crate) const flags:&[pattern_t] = &[
    pattern_t {
        name: "rainbow",
        ansii_pattern: ansii_pattern_t (
             &[ 39, 38, 44, 43, 49, 48, 84, 83, 119, 118, 154, 148, 184, 178,
                214, 208, 209, 203, 204, 198, 199, 163, 164, 128, 129, 93, 99, 63, 69, 33 ]
        ),
        color_pattern: color_pattern::Rainbow
    },

    pattern_t {
        name: "transgender",
        ansii_pattern: ansii_pattern_t (
            &[81, 81, 217, 217,  231, 231,  217, 217,  81, 81]
        ),
        color_pattern: color_pattern::Stripes(color_pattern_t {
            stripes_colors: &[
                0x55cdfc, /* #55cdfc - Blue */
                0xf7a8b8, /* #f7a8b8 - Pink */
                0xffffff, /* #ffffff - White */
                0xf7a8b8, /* #f7a8b8 - Pink */
                0x55cdfc  /* #55cdfc - Blue */
            ],
            factor: 4.0
        }),
    },
/* todo! finish converting...
    {
        .name = "nonbinary",
        .ansii_pattern = {
            .codes_count = 8,
            .ansii_codes = {226, 226, 255, 255, 93, 93, 234, 234}
        },
        .color_pattern = {
            .stripes_count = 4,
            .stripes_colors = {
                0xffff00, /* #ffff00 - Yellow */
                0xb000ff, /* #b000ff - Purple */
                0xffffff, /* #ffffff - White */
                0x000000  /* #000000 - Black */
            },
            .factor = 4.0f
        },
        .get_color = get_color_stripes
    },

    {
        .name = "lesbian",
        .ansii_pattern = {
            .codes_count = 5,
            .ansii_codes = {196, 208, 255, 170, 128}
        },
        .color_pattern = {
            .stripes_count = 5,
            .stripes_colors = {
                0xff0000, /* #ff0000 - Red */
                0xff993f, /* #ff993f - Orange */
                0xffffff, /* #ffffff - White */
                0xff8cbd, /* #ff8cbd - Pink */
                0xff4284  /* #ff4284 - Purple */
            },
            .factor = 2.0f
        },
        .get_color = get_color_stripes
    },

    {
        .name = "gay",
        .ansii_pattern = {
            .codes_count = 7,
            .ansii_codes = {36, 49, 121, 255, 117, 105, 92}
        },
        .color_pattern = {
            .stripes_count = 5,
            .stripes_colors = {
                0x00b685, /* #00b685 - Teal */
                0x6bffb6, /* #6bffb6 - Green */
                0xffffff, /* #ffffff - White */
                0x8be1ff, /* #8be1ff - Blue */
                0x8e1ae1  /* #8e1ae1 - Purple */
            },
            .factor = 6.0f
        },
        .get_color = get_color_stripes
    },

    {
        .name = "pansexual",
        .ansii_pattern = {
            .codes_count = 9,
            .ansii_codes = {200, 200, 200,  227, 227, 227,  45, 45, 45}
        },
        .color_pattern = {
            .stripes_count = 3,
            .stripes_colors = {
                0xff3388, /* #ff3388 - Pink */
                0xffea00, /* #ffea00 - Yellow */
                0x00dbff  /* #00dbff - Cyan */
            },
            .factor = 8.0f
        },
        .get_color = get_color_stripes
    },

    {
        .name = "bisexual",
        .ansii_pattern = {
            .codes_count = 8,
            .ansii_codes = {162, 162, 162,  129, 129, 27, 27, 27}
        },
        .color_pattern = {
            .stripes_count = 5,
            .stripes_colors = {
                0xff3b7b, /* #ff3b7b - Pink */
                0xff3b7b, /* #ff3b7b - Pink */
                0xd06bcc, /* #d06bcc - Purple */
                0x3b72ff, /* #3b72ff - Blue */
                0x3b72ff  /* #3b72ff - Blue */
            },
            .factor = 4.0f
        },
        .get_color = get_color_stripes
    },

    {
        .name = "gender_fluid",
        .ansii_pattern = {
            .codes_count = 10,
            .ansii_codes = {219, 219, 255, 255, 128, 128, 234, 234, 20, 20}
        },
        .color_pattern = {
            .stripes_count = 5,
            .stripes_colors = {
                0xffa0bc, /* #ffa0bc - Pink */
                0xffffff, /* #ffffff - White */
                0xc600e4, /* #c600e4 - Purple */
                0x000000, /* #000000 - Black */
                0x4e3cbb  /* #4e3cbb - Blue */
            },
            .factor = 2.0f
        },
        .get_color = get_color_stripes
    },

    {
        .name = "asexual",
        .ansii_pattern = {
            .codes_count = 8,
            .ansii_codes = {233, 233, 247, 247, 255, 255, 5, 5}
        },
        .color_pattern = {
            .stripes_count = 4,
            .stripes_colors = {
                0x000000, /* #000000 - Black */
                0xa3a3a3, /* #a3a3a3 - Gray */
                0xffffff, /* #ffffff - White */
                0x800080  /* #800080 - Purple */
            },
            .factor = 4.0f
        },
        .get_color = get_color_stripes
    },

    {
        .name = "unlabeled",
        .ansii_pattern = {
            .codes_count = 8,
            .ansii_codes = {194, 194, 255, 255, 195, 195, 223, 223}
        },
        .color_pattern = {
            .stripes_count = 4,
            .stripes_colors = {
                0xe6f9e3, /* #e6f9e3 - Green */
                0xfdfdfb, /* #fdfdfb - White */
                0xdeeff9, /* #deeff9 - Blue */
                0xfae1c2  /* #fae1c2 - Orange */
            },
            .factor = 4.0f
        },
        .get_color = get_color_stripes
    },

    {
        .name = "aromantic",
        .ansii_pattern = {
            .codes_count = 10,
            .ansii_codes = {
                34, 34,
                120, 120,
                255, 255,
                247, 247,
                233, 233
            }
        },
        .color_pattern = {
            .stripes_count = 5,
            .stripes_colors = {
                0x3da542, /* #3da542 - Green        */
                0xa8d379, /* #a8d379 - Light green  */
                0xffffff, /* #ffffff - White        */
                0xa9a9a9, /* #a9a9a9 - Grey         */
                0x000000  /* #000000 - Black        */
            },
            .factor = 1.0f
        },
        .get_color = get_color_stripes
    },

    {
        .name = "aroace",
        .ansii_pattern = {
            .codes_count = 10,
            .ansii_codes = {
                208, 208,
                220, 220,
                255, 255,
                75, 75,
                62, 62
            },
        },
        .color_pattern = {
            .stripes_count = 5,
            .stripes_colors = {
                0xe28d00, /* #e28d00 - Orange     */
                0xeccd00, /* #eccd00 - Yellow     */
                0xffffff, /* #ffffff - White      */
                0x62afdd, /* #62afdd - Light blue */
                0x203756  /* #203756 - Blue       */
            },
            .factor = 1.0f
        },
        .get_color = get_color_stripes
    },
    /* Add new flags above this line. */
*/
];
