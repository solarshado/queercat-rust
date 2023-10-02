# queercat-rust
A rewrite of [queercat](https://github.com/Elsa002/queercat) (a version of lolcat with some lgbtq+ pride flags options) in Rust.

## Usage
`$ queercat [-f flag] [-h horizontal_speed] [-v vertical_speed] [--] [FILES...]`

Run `queercat --help` to see full help and supported pride flags.
(Note: `--help` output is colorized and can be used to experiment with options if you don't have a file handy!)

## Adding a flag

### Step 1: Define the pattern
To add a flag, first add an instance of `FlagDefinition` for it to the `FLAGS` array in `src/flags.rs`.
Find `/* Add new flags above this line. */` near the bottom of the file.

The order of flags is important! For the sake of backwards compatibility, you should only add to the end.

Example:
``` rust
    },

    FlagDefinition {
        name: "aroace",
        ansii_pattern: ColorPattern_Ansii (
            &[ 208, 220, 255, 75, 62, ],
        ),
        color_pattern: ColorPattern::Stripes(ColorStripes {
            stripes: &[
                0xe28d00, /* #e28d00 - Orange     */
                0xeccd00, /* #eccd00 - Yellow     */
                0xffffff, /* #ffffff - White      */
                0x62afdd, /* #62afdd - Light blue */
                0x203756, /* #203756 - Blue       */
            ],
            factor: 1.0
        }),
    },
    /* Add new flags above this line. */
];
```

### Step 2: Pull request :)

## Credits
Base for code: <https://github.com/Elsa002/queercat>  
Prior art: <https://github.com/jaseg/lolcat/>  
Original idea: <https://github.com/busyloop/lolcat/>
