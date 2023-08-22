use crate::{FlagDefinition, ColorStripes_24bit, ColorPattern_24bit, ColorPattern_Ansii};

pub(crate) const FLAGS:&[FlagDefinition] = &[
    FlagDefinition {
        name: "rainbow",
        ansii_pattern: ColorPattern_Ansii (
             &[ 39, 38, 44, 43, 49, 48, 84, 83, 119, 118, 154, 148, 184, 178,
                214, 208, 209, 203, 204, 198, 199, 163, 164, 128, 129, 93, 99, 63, 69, 33 ]
        ),
        color_pattern: ColorPattern_24bit::Rainbow
    },

    FlagDefinition {
        name: "transgender",
        ansii_pattern: ColorPattern_Ansii (
            &[81, 81, 217, 217,  231, 231,  217, 217,  81, 81]
        ),
        color_pattern: ColorPattern_24bit::Stripes(ColorStripes_24bit {
            stripes: &[
                0x55cdfc, /* #55cdfc - Blue */
                0xf7a8b8, /* #f7a8b8 - Pink */
                0xffffff, /* #ffffff - White */
                0xf7a8b8, /* #f7a8b8 - Pink */
                0x55cdfc  /* #55cdfc - Blue */
            ],
            factor: 4.0
        }),
    },

    FlagDefinition {
        name: "nonbinary",
        ansii_pattern: ColorPattern_Ansii (
            &[226, 226, 255, 255, 93, 93, 234, 234]
        ),
        color_pattern: ColorPattern_24bit::Stripes(ColorStripes_24bit {
            stripes: &[
                0xffff00, /* #ffff00 - Yellow */
                0xb000ff, /* #b000ff - Purple */
                0xffffff, /* #ffffff - White */
                0x000000  /* #000000 - Black */
            ],
            factor: 4.0
        }),
    },

    FlagDefinition {
        name: "lesbian",
        ansii_pattern: ColorPattern_Ansii (
            &[196, 208, 255, 170, 128]
        ),
        color_pattern: ColorPattern_24bit::Stripes(ColorStripes_24bit {
            stripes: &[
                0xff0000, /* #ff0000 - Red */
                0xff993f, /* #ff993f - Orange */
                0xffffff, /* #ffffff - White */
                0xff8cbd, /* #ff8cbd - Pink */
                0xff4284  /* #ff4284 - Purple */
            ],
            factor: 2.0
        }),
    },

    FlagDefinition {
        name: "gay",
        ansii_pattern: ColorPattern_Ansii (
            &[36, 49, 121, 255, 117, 105, 92]
        ),
        color_pattern: ColorPattern_24bit::Stripes(ColorStripes_24bit {
            stripes: &[
                0x00b685, /* #00b685 - Teal */
                0x6bffb6, /* #6bffb6 - Green */
                0xffffff, /* #ffffff - White */
                0x8be1ff, /* #8be1ff - Blue */
                0x8e1ae1  /* #8e1ae1 - Purple */
            ],
            factor: 6.0
        }),
    },

    FlagDefinition {
        name: "pansexual",
        ansii_pattern: ColorPattern_Ansii (
            &[200, 200, 200,  227, 227, 227,  45, 45, 45]
        ),
        color_pattern: ColorPattern_24bit::Stripes(ColorStripes_24bit {
            stripes: &[
                0xff3388, /* #ff3388 - Pink */
                0xffea00, /* #ffea00 - Yellow */
                0x00dbff  /* #00dbff - Cyan */
            ],
            factor: 8.0
        }),
    },

    FlagDefinition {
        name: "bisexual",
        ansii_pattern: ColorPattern_Ansii (
            &[162, 162, 162,  129, 129, 27, 27, 27]
        ),
        color_pattern: ColorPattern_24bit::Stripes(ColorStripes_24bit {
            stripes: &[
                0xff3b7b, /* #ff3b7b - Pink */
                0xff3b7b, /* #ff3b7b - Pink */
                0xd06bcc, /* #d06bcc - Purple */
                0x3b72ff, /* #3b72ff - Blue */
                0x3b72ff  /* #3b72ff - Blue */
            ],
            factor: 4.0
        }),
    },

    FlagDefinition {
        name: "gender_fluid",
        ansii_pattern: ColorPattern_Ansii (
            &[219, 219, 255, 255, 128, 128, 234, 234, 20, 20]
        ),
        color_pattern: ColorPattern_24bit::Stripes(ColorStripes_24bit {
            stripes: &[
                0xffa0bc, /* #ffa0bc - Pink */
                0xffffff, /* #ffffff - White */
                0xc600e4, /* #c600e4 - Purple */
                0x000000, /* #000000 - Black */
                0x4e3cbb  /* #4e3cbb - Blue */
            ],
            factor: 2.0
        }),
    },

    FlagDefinition {
        name: "asexual",
        ansii_pattern: ColorPattern_Ansii (
            &[233, 233, 247, 247, 255, 255, 5, 5]
        ),
        color_pattern: ColorPattern_24bit::Stripes(ColorStripes_24bit {
            stripes: &[
                0x000000, /* #000000 - Black */
                0xa3a3a3, /* #a3a3a3 - Gray */
                0xffffff, /* #ffffff - White */
                0x800080  /* #800080 - Purple */
            ],
            factor: 4.0
        }),
    },

    FlagDefinition {
        name: "unlabeled",
        ansii_pattern: ColorPattern_Ansii (
            &[194, 194, 255, 255, 195, 195, 223, 223]
        ),
        color_pattern: ColorPattern_24bit::Stripes(ColorStripes_24bit {
            stripes: &[
                0xe6f9e3, /* #e6f9e3 - Green */
                0xfdfdfb, /* #fdfdfb - White */
                0xdeeff9, /* #deeff9 - Blue */
                0xfae1c2  /* #fae1c2 - Orange */
            ],
            factor: 4.0
        }),
    },

    FlagDefinition {
        name: "aromantic",
        ansii_pattern: ColorPattern_Ansii (
            &[
                34, 34,
                120, 120,
                255, 255,
                247, 247,
                233, 233
            ]
        ),
        color_pattern: ColorPattern_24bit::Stripes(ColorStripes_24bit {
            stripes: &[
                0x3da542, /* #3da542 - Green        */
                0xa8d379, /* #a8d379 - Light green  */
                0xffffff, /* #ffffff - White        */
                0xa9a9a9, /* #a9a9a9 - Grey         */
                0x000000  /* #000000 - Black        */
            ],
            factor: 1.0
        }),
    },

    FlagDefinition {
        name: "aroace",
        ansii_pattern: ColorPattern_Ansii (
            &[
                208, 208,
                220, 220,
                255, 255,
                75, 75,
                62, 62
            ],
        ),
        color_pattern: ColorPattern_24bit::Stripes(ColorStripes_24bit {
            stripes: &[
                0xe28d00, /* #e28d00 - Orange     */
                0xeccd00, /* #eccd00 - Yellow     */
                0xffffff, /* #ffffff - White      */
                0x62afdd, /* #62afdd - Light blue */
                0x203756  /* #203756 - Blue       */
            ],
            factor: 1.0
        }),
    },
    /* Add new flags above this line. */
];
