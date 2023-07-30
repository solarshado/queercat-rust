/*
#define _XOPEN_SOURCE
#define _GNU_SOURCE

/* *** Includes ******************************************************/
#include <stdbool.h>
#include <ctype.h>
#include <err.h>
#include <errno.h>
#include <locale.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/time.h>
#include <unistd.h>
#include <wchar.h>
#include <time.h>
#include "math.h"
*/

use std::process::exit;

const NEWLINE:char = '\n';
const ESCAPE_CHAR:char = '\x1b'; //'\033'

/* Types */
#[derive(PartialEq)]
enum escape_state_e {
    ESCAPE_STATE_OUT,
    ESCAPE_STATE_IN,
    ESCAPE_STATE_LAST
}

/* Macros */
// #define UNUSED(var) ((void)(var))

// #define NEXT_CYCLIC_ELEMENT(array, index, array_size) \
//    (((index) + 1 == (array_size)) ? (array)[0] : (array)[((index) + 1)] )
/*
macro_rules! NEXT_CYCLIC_ELEMENT {
    ($array:expr, $index:expr, $array_size:expr) => {
        ( if (($index) + 1 == ($array_size)) { ($array)[0] } else { ($array)[(($index) + 1)] } )
    };
}
*/

fn next_cyclic_element<T>(container:&[T], curr_pos:usize) -> T
{
    let next_i = curr_pos + 1;
    if next_i > container.len() {
        container[0]
    }
    else {
        container[next_i]
    }
}

// #define IS_LETTER(c) (('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z'))
macro_rules! IS_LETTER {
    ($c:ident) => { (('a' <= $c && $c <= 'z') || ('A' <= $c && $c <= 'Z')) }
}

/* *** Constants *****************************************************/

const MAX_FLAG_STRIPES: usize = 6;
const MAX_ANSII_CODES_PER_STRIPE: usize = 5;
const MAX_ANSII_CODES_COUNT: usize = MAX_FLAG_STRIPES * MAX_ANSII_CODES_PER_STRIPE;
const MAX_FLAG_NAME_LENGTH: u32 = 64;

/* *** Types *********************************************************/
/* Colors. */
//typedef uint32_t hex_color_t;
type hex_color_t = u32;
//typedef unsigned char ansii_code_t;
type ansii_code_t = u8;

struct color_t {
    red: u8,
    green: u8,
    blue: u8,
}

/* Color type patterns. */
enum color_type_t {
    COLOR_TYPE_INVALID = -1,
    COLOR_TYPE_ANSII = 0,
    COLOR_TYPE_24_BIT,
    COLOR_TYPE_COUNT
}

struct ansii_pattern_t {
//    const unsigned int codes_count;
    codes_count : u32,
//    const unsigned char ansii_codes[MAX_ANSII_CODES_COUNT];
    ansii_codes : Vec<ansii_code_t>,
}

//#[derive(Default)]
struct color_pattern_t {
//    const uint8_t stripes_count;
    stripes_count : u8,
//    const uint32_t stripes_colors[MAX_FLAG_STRIPES];
    stripes_colors : Vec<hex_color_t>,
//    const float factor;
    factor : f32,
}

/* Get color function. */
//typedef void(get_color_f)(const color_pattern_t *color_pattern, float theta, color_t *color);
//type get_color_f = dyn Fn(&color_pattern_t,&f32,&color_t);
type get_color_f = fn(&color_pattern_t, f32, &mut color_t);

enum get_color_f_impl {
    Rainbow,
    Stripes
}

// inspo: https://stackoverflow.com/a/66714422/
impl std::ops::Deref for get_color_f_impl {
    type Target = get_color_f;
    fn deref(&self) -> &Self::Target {
        use get_color_f_impl::*;
        &(match self {
            Rainbow => get_color_rainbow,
            Stripes => get_color_stripes,
        })
    }
}

/* Pattern. */
struct pattern_t {
//    const char name[MAX_FLAG_NAME_LENGTH];
    name: &'static str,
//    const ansii_pattern_t ansii_pattern;
    ansii_pattern: ansii_pattern_t,
//    const color_pattern_t color_pattern;
    color_pattern: Option<color_pattern_t>,
//    get_color_f *get_color;
    get_color: get_color_f_impl,
}

/* *** A Single Global ***********************************************/
//const helpstr:&str = build_helpstr();

/* *** Pattern Functions *********************************************/
//get_color_f get_color_rainbow;
//get_color_f get_color_stripes;


/* shelved experiment...
macro_rules! arr {
    ($({ $v: expr, $s: expr }),* $(,)?) => {
        [
            $(Arr { v: $v, s: $s }),*
        ]
    };
}

//inspo: https://users.rust-lang.org/t/array-of-structs-quick-like-c/83681/3
macro_rules! patt_arry {
    ($({ $(.$fn:ident = $val:expr),+ }),* $(,)?) => {
        [
            $(pattern_t { $($fn = $val),+ } ),*
        ]
    };
}
*/ /*
const flergz:[pattern_t] = [
    pattern_t {
        name: "foo",
        ansii_pattern: ansii_pattern_t {
            codes_count: 1,
            ansii_codes: [42],
        },
        color_pattern: color_pattern_t {
            stripes_count: 0,
            stripes_colors:[],
            factor: 0.5,
        },
        get_color: get_color_rainbow,
    }
];
*/

/* *** Flags *********************************************************/
//const flags:[pattern_t ] = patt_arry!{
static flags:&[pattern_t] = &[
    pattern_t {
        name: "rainbow",
        ansii_pattern: ansii_pattern_t {
            codes_count: 30,
            ansii_codes: vec![ 39, 38, 44, 43, 49, 48, 84, 83, 119, 118, 154, 148, 184, 178,
                214, 208, 209, 203, 204, 198, 199, 163, 164, 128, 129, 93, 99, 63, 69, 33 ]
        },
        color_pattern: None,
        get_color: get_color_f_impl::Rainbow
    },

    pattern_t {
        name: "transgender",
        ansii_pattern: ansii_pattern_t {
            codes_count: 10,
            ansii_codes: vec![81, 81, 217, 217,  231, 231,  217, 217,  81, 81]
        },
        color_pattern: color_pattern_t {
            stripes_count: 5,
            stripes_colors: vec![
                0x55cdfc, /* #55cdfc - Blue */
                0xf7a8b8, /* #f7a8b8 - Pink */
                0xffffff, /* #ffffff - White */
                0xf7a8b8, /* #f7a8b8 - Pink */
                0x55cdfc  /* #55cdfc - Blue */
            ],
            factor: 4.0
        }.into(),
        get_color: get_color_f_impl::Stripes
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

//const int FLAG_COUNT = sizeof(flags)/sizeof(flags[0]);
const FLAG_COUNT:usize = flags.len();

/*
/* *** Functions Declarations ****************************************/
/* Info */
static void usage(void);
static void version(void);

/* Helpers */
static void build_helpstr(void);
static void cleanup_helpstr(void);
static void find_escape_sequences(wint_t current_char, escape_state_t *state);
static wint_t helpstr_hack(FILE * _ignored);
static const pattern_t * lookup_pattern(const char *name);

/* Colors handling */
static void mix_colors(uint32_t color1, uint32_t color2, float balance, float factor, color_t *output_color);
static void print_color(const pattern_t *pattern, color_type_t color_type, int char_index, int line_index, double freq_h, double freq_v, double offx, int rand_offset, int cc);
*/

/* *** Functions *****************************************************/
fn usage()
{
    print!("Usage: queercat [-h horizontal_speed] [-v vertical_speed] [--] [FILES...]\n");
    exit(1);
}

fn version()
{
    todo!();
//    wprintf("queercat version 2.0, (c) 2022 elsa002\n");
    exit(0);
}

fn build_helpstr() -> &'static str
{
    //
    // consider instead:
    // https://stackoverflow.com/a/32956193/
    //
    //use const_format::*;

    let helpstr_head = concat!["\n",
        "Usage: queercat [-f flag_number][-h horizontal_speed] [-v vertical_speed] [--] [FILES...]\n",
        "\n",
        "Concatenate FILE(s), or standard input, to standard output.\n",
        "With no FILE, or when FILE is -, read standard input.\n",
        "\n",
        "                --flag <d>, -f <d>: Choose colors to use (default: 0 (rainbow)):\n"];

    let helpstr_indent = "                                      ";

    let helpstr_tail = concat![
        "--horizontal-frequency <d>, -h <d>: Horizontal rainbow frequency (default: 0.23)\n",
        "  --vertical-frequency <d>, -v <d>: Vertical rainbow frequency (default: 0.1)\n",
        "              --offset <d>, -o <d>: Offset of the start of the flag\n",
        "                 --force-color, -F: Force color even when stdout is not a tty\n",
        "             --no-force-locale, -l: Use encoding from system locale instead of\n",
        "                                    assuming UTF-8\n",
        "                      --random, -r: Random colors\n",
        "                       --24bit, -b: Output in 24-bit \"true\" RGB mode (slower and\n",
        "                                    not supported by all terminals)\n",
        "                         --version: Print version and exit\n",
        "                            --help: Show this message\n",
        "\n",
        "Examples:\n",
        "  queercat f - g      Output f's contents, then stdin, then g's contents.\n",
        "  queercat            Copy standard input to standard output.\n",
        "  fortune | queercat  Display a rainbow cookie.\n",
        "\n",
//        todo!["update this footer"]
        "Report queercat bugs to <https://github.com/elsa002/queercat/issues>\n",
        "queercat home page: <https://github.com/elsa002/queercat/>\n",
        "base for code: <https://github.com/jaseg/lolcat/>\n",
        "Original idea: <https://github.com/busyloop/lolcat/>\n"];

    /* old version of what this generates, for reference:
     * "                                    [rainbow: 0, trans: 1, NB: 2, lesbian: 3,\n"
     * "                                    gay: 4, pan: 5, bi: 6, genderfluid: 7, asexual: 8,\n"
     * "                                    unlabeled: 9, aromantic: 10, aroace: 11]\n"
     * would be nice to have the dynamic word-wrap back, but that's
     * more clever than I currently feel like trying to be
     */
    use std::iter::{once};

//    let helpstr_flag_list:String =
        once(helpstr_head).chain(
        flags.iter().enumerate().map(|(i,e)| format!("{helpstr_indent}{0}: {i}\n",e.name).as_str())
        ).chain(once(helpstr_tail)).collect::<String>().as_str()
//    ;
//    return "";

/*
    const int line_max_len = strlen(helpstr_indent) + MAX_FLAG_NAME_LENGTH + strlen(": 000\n") ;
    char lines[FLAG_COUNT][line_max_len];
    size_t lines_total_len = 0;

    for(int i = 0; i < FLAG_COUNT; ++i) {
        lines_total_len += snprintf(lines[i], line_max_len, "%s%s: %d\n", helpstr_indent, flags[i].name, i);
    }

    size_t helpstr_len = strlen(helpstr_head) + lines_total_len + strlen(helpstr_tail);

    char *out = malloc(helpstr_len);
    char *out_pos = out;

    out_pos = mempcpy(out, helpstr_head, strlen(helpstr_head));

    for(int i = 0; i < FLAG_COUNT; ++i) {
        char* this_line = lines[i];
        out_pos = mempcpy(out_pos, this_line, strlen(this_line));
    }
    */
}

/*
static void cleanup_helpstr(void)
{
    free(helpstr);
}
*/

fn find_escape_sequences(current_char:char, state:&mut escape_state_e)
{
    use escape_state_e::*;
    if current_char == ESCAPE_CHAR {
        *state = ESCAPE_STATE_IN;
    } else if *state == ESCAPE_STATE_IN {
        *state = if IS_LETTER!(current_char) { ESCAPE_STATE_LAST } else { ESCAPE_STATE_IN };
    } else {
        *state = ESCAPE_STATE_OUT;
    }
}

/* totally unneeded?
static wint_t helpstr_hack(FILE * _ignored)
{
    (void)_ignored;
    static size_t idx = 0;
    char c = helpstr[idx++];
    if (c)
        return c;
    idx = 0;
    return WEOF;
}
*/

fn lookup_pattern(name:&str) -> Option<&pattern_t>
{
    flags.iter().find(|f| f.name == name)
        .or_else(|| {
            let n:usize = str::parse(name).ok()
                .filter(|n| *n < FLAG_COUNT)?;
            Some(&flags[n])
        })
}

// TODO rewrite to return instead of use &mut
fn mix_colors(color1:u32, color2:u32, balance:f32, factor:f32, output_color:&mut color_t)
{
    let red_1   = ((color1 & 0xff0000) >> 16) as f32;
    let green_1 = ((color1 & 0x00ff00) >>  8) as f32;
    let blue_1  = ((color1 & 0x0000ff) >>  0) as f32;

    let red_2   = ((color2 & 0xff0000) >> 16) as f32;
    let green_2 = ((color2 & 0x00ff00) >>  8) as f32;
    let blue_2  = ((color2 & 0x0000ff) >>  0) as f32;

    let mut balance = balance.powf(factor);

    output_color.red = (red_1 * balance + red_2 * (1.0 - balance)).round() as u8;
    output_color.green = (green_1 * balance + green_2 * (1.0 - balance)).round() as u8;
    output_color.blue = (blue_1 * balance + blue_2 * (1.0 - balance)).round() as u8;
}

fn clamp_theta(mut theta:f32) -> f32
{
    use std::f32::consts::PI;
    while theta < 0.0 { theta += 2.0 * PI; }
    while theta >= 2.0 * PI { theta -= 2.0 * PI; }
    theta
}

// TODO rewrite to return instead of use &mut
fn get_color_rainbow(color_pattern:&color_pattern_t, theta:f32, color:&mut color_t )
{
    use std::f32::consts::PI;
    let theta = clamp_theta(theta);
    /* Unused variables. */
    //UNUSED(color_pattern);

    /* Get theta in range. */
//    while (theta < 0) { theta += 2.0f * PI; }
//    while (theta >= 2.0f * PI) { theta -= 2.0f * PI; }

    /* Generate the color. */
    color.red   = ((1.0 * (0.5 + 0.5 * (theta + 0.0            ).sin())) * 255.0).round() as u8;
    color.green = ((1.0 * (0.5 + 0.5 * (theta + 2.0 * PI / 3.0 ).sin())) * 255.0).round() as u8;
    color.blue  = ((1.0 * (0.5 + 0.5 * (theta + 4.0 * PI / 3.0 ).sin())) * 255.0).round() as u8;
}

// TODO rewrite to return instead of use &mut
fn get_color_stripes(color_pattern:&color_pattern_t, theta:f32, color:&mut color_t )
{
    use std::f32::consts::PI;
    let theta = clamp_theta(theta);

    /* Get theta in range. */
//    while (theta < 0) { theta += 2.0f * PI; }
//    while (theta >= 2.0f * PI) { theta -= 2.0f * PI; }

    /* Find the stripe based on theta and generate the color. */
    for i in 0..(color_pattern.stripes_count as usize) {
        let stripe_size = (2.0 * PI) / color_pattern.stripes_count as f32;
        let min_theta = i as f32 * stripe_size;
        let max_theta = (i + 1) as f32 * stripe_size;

        if min_theta <= theta && max_theta > theta {
            let balance = 1.0 - ((theta - min_theta) / stripe_size);
            mix_colors(
                    color_pattern.stripes_colors[i],
                    next_cyclic_element(&color_pattern.stripes_colors, i),
                    balance,
                    color_pattern.factor,
                    color);
            return;
        }
    }
}

fn print_color(pattern:&pattern_t, color_type:color_type_t, char_index:u32, line_index:u32, freq_h:f32, freq_v:f32, offx:f32, rand_offset:u32, cc:u32)
{
    use self::color_type_t::*;
    use std::f32::{MAX as f32MAX, consts::PI};

    // TODO can we make this less gross?
    let char_index_f:f32 = char_index as f32;
    let line_index_f:f32 = line_index as f32;
//    let offx_f:f32 = offx as f32;
    let rand_offset_f:f32 = rand_offset as f32;

    let mut theta:f32;
    let color:color_t = color_t { red: 0, green: 0, blue:0 };

    let mut ncc;

    match color_type {
        COLOR_TYPE_24_BIT => {
            theta = char_index_f * freq_h / 5.0 +
                line_index_f * freq_v +
                (offx + 2.0 * rand_offset_f / f32MAX) * PI;

            // TODO avoid unwrap() below
            (pattern.get_color)(&pattern.color_pattern.unwrap(), theta, &mut color);
            print!("{}[38;2;{};{};{}m", ESCAPE_CHAR, color.red, color.green, color.blue);
        },

        COLOR_TYPE_ANSII => {
            let pat_code_count = pattern.ansii_pattern.codes_count;
            ncc = ((offx * (pat_code_count as f32)).round() as u32) +
                ((char_index_f * freq_h + line_index_f * freq_v).trunc() as u32);
            if cc != ncc {
                cc = ncc;
                print!("{}[38;5;{}m", ESCAPE_CHAR,
                       pattern.ansii_pattern.ansii_codes[((rand_offset + cc) % pat_code_count) as usize]);
            }
        }
        _ => { exit(1); }
    }
}

fn main() //-> u32
{
    let args = std::env::args().collect();

    char* default_argv[] = { "-" };
    int cc = -1;
    int i = 0;
    int char_index = 0;
    int line_index = 0;
    wint_t current_char = '\0';
    bool print_colors = isatty(STDOUT_FILENO);
    bool force_locale = true;
    bool random = false;
    color_type_t color_type = COLOR_TYPE_ANSII;
    double freq_h = 0.23;
    double freq_v = 0.1;
    char* flag_type = "rainbow";

    struct timeval tv;
    gettimeofday(&tv, NULL);
    double offx = (tv.tv_sec % 300) / 300.0;

    build_helpstr();

    /* Handle flags. */
    for (i = 1; i < argc; i++) {
        char* endptr;
        if (!strcmp(argv[i], "-f") || !strcmp(argv[i], "--flag")) {
            if ((++i) < argc) {
                flag_type = argv[i];
            } else {
                usage();
            }
        } else if (!strcmp(argv[i], "-h") || !strcmp(argv[i], "--horizontal-frequency")) {
            if ((++i) < argc) {
                freq_h = strtod(argv[i], &endptr);
                if (*endptr)
                    usage();
            } else {
                usage();
            }
        } else if (!strcmp(argv[i], "-o") || !strcmp(argv[i], "--offset")) {
            if ((++i) < argc) {
                offx = strtod(argv[i], &endptr);
                if (*endptr)
                    usage();
            } else {
                usage();
            }
        } else if (!strcmp(argv[i], "-v") || !strcmp(argv[i], "--vertical-frequency")) {
            if ((++i) < argc) {
                freq_v = strtod(argv[i], &endptr);
                if (*endptr)
                    usage();
            } else {
                usage();
            }
        } else if (!strcmp(argv[i], "-F") || !strcmp(argv[i], "--force-color")) {
            print_colors = true;
        } else if (!strcmp(argv[i], "-l") || !strcmp(argv[i], "--no-force-locale")) {
            force_locale = false;
        } else if (!strcmp(argv[i], "-r") || !strcmp(argv[i], "--random")) {
            random = true;
        } else if (!strcmp(argv[i], "-b") || !strcmp(argv[i], "--24bit")) {
            color_type = COLOR_TYPE_24_BIT;
        } else if (!strcmp(argv[i], "--version")) {
            version();
        } else {
            if (!strcmp(argv[i], "--"))
                i++;
            break;
        }
    }

    /* Get pattern. */
    const pattern_t *pattern = lookup_pattern(flag_type);
    if (pattern == NULL) {
        fprintf(stderr, "Invalid flag: %s\n", flag_type);
        exit(1);
    }

    /* Handle randomness. */
    int rand_offset = 0;
    if (random) {
        srand(time(NULL));
        rand_offset = rand();
    }

    /* Get inputs. */
    char** inputs = argv + i;
    char** inputs_end = argv + argc;
    if (inputs == inputs_end) {
        inputs = default_argv;
        inputs_end = inputs + 1;
    }

    /* Handle locale. */
    char* env_lang = getenv("LANG");
    if (force_locale && env_lang && !strstr(env_lang, "UTF-8")) {
        if (!setlocale(LC_ALL, "C.UTF-8")) { /* C.UTF-8 may not be available on all platforms */
            setlocale(LC_ALL, ""); /* Let's hope for the best */
        }
    } else {
        setlocale(LC_ALL, "");
    }

    /* For file in inputs. */
    for (char** filename = inputs; filename < inputs_end; filename++) {
        wint_t (*this_file_read_wchar)(FILE*); /* Used for --help because fmemopen is universally broken when used with fgetwc */
        FILE* f;
        escape_state_t escape_state = ESCAPE_STATE_OUT;

        /* Handle "--help", "-" (STDIN) and file names. */
        if (!strcmp(*filename, "--help")) {
            this_file_read_wchar = &helpstr_hack;
            f = 0;

        } else if (!strcmp(*filename, "-")) {
            this_file_read_wchar = &fgetwc;
            f = stdin;

        } else {
            this_file_read_wchar = &fgetwc;
            f = fopen(*filename, "r");
            if (!f) {
                fwprintf(stderr, "Cannot open input file \"%s\": %s\n", *filename, strerror(errno));
                return 2;
            }
        }

        /* While there are chars to read. */
        while ((current_char = this_file_read_wchar(f)) != WEOF) {

            /* If set to print colors, handle the colors. */
            if (print_colors) {

                /* Skip escape sequences. */
                find_escape_sequences(current_char, &escape_state);
                if (escape_state == ESCAPE_STATE_OUT) {

                    /* Handle newlines. */
                    if (current_char == '\n') {
                        line_index++;
                        char_index = 0;
                    } else {
                        char_index += wcwidth(current_char);
                        print_color(pattern, color_type, char_index, line_index, freq_h, freq_v, offx, rand_offset, cc);
                    }
                }
            }

            /* Print the char. */
            putwchar(current_char);

            if (escape_state == ESCAPE_STATE_LAST) {  /* implies "print_colors" */
                print_color(pattern, color_type, char_index, line_index, freq_h, freq_v, offx, rand_offset, cc);
            }
        }

        if (print_colors)
            wprintf("\033[0m");

        cc = -1;

        if (f) {
            if (ferror(f)) {
                fwprintf(stderr, "Error reading input file \"%s\": %s\n", *filename, strerror(errno));
                fclose(f);
                return 2;
            }

            if (fclose(f)) {
                fwprintf(stderr, "Error closing input file \"%s\": %s\n", *filename, strerror(errno));
                return 2;
            }
        }
    }
}
