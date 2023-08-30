mod flags;
use flags::FLAGS;

const ESCAPE_CHAR: char = '\x1b'; //'\033'

struct FlagDefinition {
    name: &'static str,
    ansii_pattern: ColorPattern_Ansii,
    color_pattern: ColorPattern_24bit,
}

// TODO? replace below struct with:
//type ansii_pattern_t = &'static [u8];
struct ColorPattern_Ansii(&'static [u8]);
impl ColorPattern_Ansii {
    fn codes_count(&self) -> usize {
        self.0.len()
    }
    fn ansii_codes(&self) -> &[u8] {
        self.0
    }
}

enum ColorPattern_24bit {
    Rainbow,
    Stripes(ColorStripes_24bit)
}

struct ColorStripes_24bit {
    stripes: &'static [u32],
    factor: f32,
}

fn lookup_pattern(name: &str) -> Option<&'static FlagDefinition> {
    FLAGS.iter().find(|f| f.name == name)
        .or_else(|| {
            let n:usize = str::parse(name).ok()
                .filter(|n| *n < FLAGS.len())?;
            Some(&FLAGS[n])
        })
}

fn print_version() {
    println!("queercat-rust version 1.0, (c) 2023 solarshado");
}

//fn build_helpstr() -> &'static str
fn build_helpstr() -> String {
    //
    // consider instead:
    // https://stackoverflow.com/a/32956193/
    //
    //use const_format::*;

    // TODO use Settings::DEFAULT_mumble directly instead of repeating the values
    let helpstr_head = concat![
        "Usage: queercat [OPTION...] [--] [FILE...]\n",
        "\n",
        "Concatenate FILE(s), or standard input, to standard output.\n",
        "With no FILE, or when FILE is -, read standard input.\n",
        "\n",
        "                --flag <d>, -f <d>: Choose colors to use (default: 0 (rainbow)):\n"
    ];

    let helpstr_indent = "                                      ";

    let helpstr_tail = concat![
        "--horizontal-frequency <d>, -h <d>: Horizontal rainbow frequency (default: 0.23)\n",
        "  --vertical-frequency <d>, -v <d>: Vertical rainbow frequency (default: 0.1)\n",
        "              --offset <d>, -o <d>: Offset of the start of the flag\n",
        "                 --force-color, -F: Force color even when stdout is not a tty\n",
//        "             --no-force-locale, -l: Use encoding from system locale instead of\n",
//        "                                    assuming UTF-8\n",
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
        "Report bugs to <https://github.com/solarshado/queercat-rust/issues>\n",
        "queercat-rust home page: <https://github.com/solarshado/queercat-rust/>\n",
        "base for code: <https://github.com/elsa002/queercat/>\n",
        "Original idea: <https://github.com/busyloop/lolcat/>\n"
    ];

    /* TODO from C version:
     * old version of what this generates, for reference:
     * "                                    [rainbow: 0, trans: 1, NB: 2, lesbian: 3,\n"
     * "                                    gay: 4, pan: 5, bi: 6, genderfluid: 7, asexual: 8,\n"
     * "                                    unlabeled: 9, aromantic: 10, aroace: 11]\n"
     * would be nice to have the dynamic word-wrap back, but that's
     * more clever than I currently feel like trying to be
     */

    let helpstr_flag_list =
        FLAGS.iter().enumerate()
        .map(|(i,e)| format!("{helpstr_indent}{0}: {i}\n",e.name))
        .collect::<String>();

    format!["{}{}{}", helpstr_head, helpstr_flag_list, helpstr_tail]
}

struct Color_24bit {
    red: u8,
    green: u8,
    blue: u8,
}

// TODO rewrite to return instead of use &mut
fn mix_colors(color1: u32, color2: u32, balance: f32, factor: f32, output_color: &mut Color_24bit) {
    let red_1   = ((color1 & 0xff0000) >> 16) as f32;
    let green_1 = ((color1 & 0x00ff00) >>  8) as f32;
    let blue_1  = ((color1 & 0x0000ff) >>  0) as f32;

    let red_2   = ((color2 & 0xff0000) >> 16) as f32;
    let green_2 = ((color2 & 0x00ff00) >>  8) as f32;
    let blue_2  = ((color2 & 0x0000ff) >>  0) as f32;

    let balance = balance.powf(factor);

    output_color.red = (red_1 * balance + red_2 * (1.0 - balance)).round() as u8;
    output_color.green = (green_1 * balance + green_2 * (1.0 - balance)).round() as u8;
    output_color.blue = (blue_1 * balance + blue_2 * (1.0 - balance)).round() as u8;
}

fn clamp_theta(mut theta: f32) -> f32 {
    use std::f32::consts::PI;
    while theta < 0.0 {
        theta += 2.0 * PI;
    }
    while theta >= 2.0 * PI {
        theta -= 2.0 * PI;
    }
    theta
}

// TODO rewrite to return instead of use &mut
fn get_color_rainbow(theta: f32, color: &mut Color_24bit) {
    use std::f32::consts::PI;
    let theta = clamp_theta(theta);

    /* Generate the color. */
    color.red   = ((1.0 * (0.5 + 0.5 * (theta + 0.0            ).sin())) * 255.0).round() as u8;
    color.green = ((1.0 * (0.5 + 0.5 * (theta + 2.0 * PI / 3.0 ).sin())) * 255.0).round() as u8;
    color.blue  = ((1.0 * (0.5 + 0.5 * (theta + 4.0 * PI / 3.0 ).sin())) * 255.0).round() as u8;
}

// TODO rewrite to return instead of use &mut
fn get_color_stripes(color_pattern: &ColorStripes_24bit, theta: f32, color: &mut Color_24bit) {
    use std::f32::consts::PI;
    let theta = clamp_theta(theta);

    let stripes = color_pattern.stripes;
    let stripe_count = stripes.len();

    // TODO? can this be calcualted directly w/out the loop?
    /* Find the stripe based on theta and generate the color. */
    let stripe_size = (2.0 * PI) / stripe_count as f32;
    for i in 0..stripe_count {
        let min_theta = i as f32 * stripe_size;
        let max_theta = (i + 1) as f32 * stripe_size;

        if min_theta <= theta && max_theta > theta {
            let balance = 1.0 - ((theta - min_theta) / stripe_size);

            let next_color = {
                let next_i = i + 1;
                if next_i >= stripe_count {
                    stripes[0]
                } else {
                    stripes[next_i]
                }
            };

            mix_colors(
                    stripes[i],
                    next_color,
                    balance,
                    color_pattern.factor,
                    color);
            return;
        }
    }
}

fn print_color(pattern: &FlagDefinition, color_type: &OutputColorType, char_index: u32, line_index: u32, freq_h: f32, freq_v: f32, offx: f32, rand_offset: i32)
{
    use self::OutputColorType::*;
    use std::f32::{consts::PI, MAX as f32MAX};

    // TODO can we make this less gross?
    let char_index_f: f32 = char_index as f32;
    let line_index_f: f32 = line_index as f32;
//    let offx_f: f32 = offx as f32;
    let rand_offset_f: f32 = rand_offset as f32;

    match color_type {
        TwentyFourBit => {
            let mut color = Color_24bit { red: 0, green: 0, blue:0 };
            let theta =
                char_index_f * freq_h / 5.0
                + line_index_f * freq_v
                + (offx + 2.0 * rand_offset_f / f32MAX) * PI;

            use ColorPattern_24bit::*;
            match &pattern.color_pattern {
                Rainbow =>
                    get_color_rainbow(theta, &mut color),
                Stripes(patt) =>
                    get_color_stripes(patt, theta, &mut color),
            }

            print!("{}[38;2;{};{};{}m", ESCAPE_CHAR, color.red, color.green, color.blue);
        },

        Ansii => {
            let pat_code_count = pattern.ansii_pattern.codes_count();
            let pat_codes = pattern.ansii_pattern.ansii_codes();

            let mut cc = -1;
            let ncc = ((offx * (pat_code_count as f32)).round() as i32)
                + ((char_index_f * freq_h + line_index_f * freq_v).trunc() as i32);

            if cc != ncc {
                cc = ncc;
                let code_index = ((rand_offset + cc) % pat_code_count as i32) as usize;
                print!("{}[38;5;{}m", ESCAPE_CHAR, pat_codes[code_index]);
            }
        }
    }
}

#[derive(PartialEq)]
enum EscapeState {
    Out,
    In,
    Last
}

// TODO rewrite to return instead of use &mut
fn find_escape_sequences(current_char: char, state: &mut EscapeState) {
    if current_char == ESCAPE_CHAR {
        *state = EscapeState::In;
    } else if *state == EscapeState::In {
        *state = if current_char.is_ascii_alphabetic() {
            EscapeState::Last
        } else {
            EscapeState::In
        };
    } else {
        *state = EscapeState::Out;
    }
}

// probably good enough?
fn get_fake_random() -> u32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().subsec_millis()
}

enum ParseArgsFail {
    PrintUsage(String),
    PrintVersion,
}

struct Settings {
    file_names: Vec<String>, // "-" means "stdin"; default ["-"]
    flag: &'static FlagDefinition, // default flags[0] (rainbow)
    horiz_freq: f32, // default 0.23
    vert_freq: f32, // default 0.1
    horiz_offset: f32, // default from (time_of_day.now.sec) % 300 /300 (?)
    enable_color: bool, // default from is_a_tty(stdout)
//    force_locale: bool, // default true
    color_type: OutputColorType, // default ansii, flag for 24bit
    enable_rand_offset: bool,
    print_help: bool, // default false, ignores file_names if true
}

impl Settings {
    const DEFAULT_FLAG_INDEX: usize = 0;
    const DEFAULT_H_FREQ: f32 = 0.23;
    const DEFAULT_V_FREQ: f32 = 0.1;
    const DEFAULT_COLOR_TYPE: OutputColorType = OutputColorType::Ansii;
    const DEFAULT_ENABLE_RAND_OFFSET: bool = false;
}

impl Default for Settings {
    fn default() -> Self {
        use std::io::{stdout, IsTerminal};
        let color_default = stdout().is_terminal();
        Settings {
            file_names: Vec::new(),
            flag: &FLAGS[Settings::DEFAULT_FLAG_INDEX],
            horiz_freq: Settings::DEFAULT_H_FREQ,
            vert_freq: Settings::DEFAULT_V_FREQ,
            horiz_offset: ((get_fake_random() % 300) / 300) as f32, // magic numbers from original version
            enable_color: color_default,
//            force_locale: true,
            color_type: Settings::DEFAULT_COLOR_TYPE,
            enable_rand_offset: Settings::DEFAULT_ENABLE_RAND_OFFSET,
            print_help: false,
        }
    }
}

enum OutputColorType {
    Ansii,
    TwentyFourBit,
}

fn parse_args(mut args: impl Iterator<Item = String>) -> Result<Settings, ParseArgsFail> {
    let _ = args.next(); // discard exename in first element

    macro_rules! usage {
        ($($i:tt)*) => {
            PrintUsage(format![$($i)*])
        };
    }
    macro_rules! next_arg_for {
        ($flag:ident) => {
            args.next().ok_or(usage!["'{}' option requires an argument!", $flag])
        };
    }
    macro_rules! badval {
        ($val:expr,$flag:ident) => {
            usage!["Invalid {} value: {}", $flag, $val]
        };
    }

    let mut settings = Settings::default();

    // TODO support -o=val / --opt=value format
    // _maybe_ "-hvof 1 2 3 4" clustering too? sounds way harder
    //      but maybe could pre-process?

    while let Some(arg) = args.next() {
        use ParseArgsFail::*;
        match arg.as_str() {
            flag if arg.starts_with('-') => match flag {
                "-f" | "--flag" => {
                    let next = next_arg_for!(flag)?;
                    settings.flag = lookup_pattern(next.as_str())
                        .ok_or_else(|| badval![next,flag])?;
                }
                "-h" | "--horizontal-frequency" => {
                    let next = next_arg_for!(flag)?;
                    settings.horiz_freq = next.parse()
                        .map_err(|_| badval![next,flag])?;
                }
                "-v" | "--vertical-frequency" => {
                    let next = next_arg_for!(flag)?;
                    settings.vert_freq = next.parse()
                        .map_err(|_| badval![next,flag])?;
                }
                "-o" | "--offset" => {
                    let next = next_arg_for!(flag)?;
                    settings.horiz_offset = next.parse()
                        .map_err(|_| badval![next,flag])?;
                }
                "-F" | "--force-color" => {
                    settings.enable_color = true;
                }
//                "-l" | "--no-force-locale" => {
//                    settings.force_locale = false;
//                }
                "-r" | "--random" => {
                    settings.enable_rand_offset = true;
                }
                "-b" | "--24bit" => {
                    settings.color_type = OutputColorType::TwentyFourBit;
                }
                "--help" => {
                    settings.print_help = true;
                }
                "--version" => {
                    return Err(PrintVersion);
                }
                "-" => {
                    settings.file_names.push(arg);
                }
                "--" => {
                    settings.file_names.extend(args);
                    break; // above consumes the rest of args, and borrows args
                }
                _ => {
                    return Err(usage!["Unknown option: {flag}"]);
                }
            }
            _ => {
                settings.file_names.push(arg);
            }
        }
    }

    // read stdin if no files specified
    if settings.file_names.is_empty() {
        settings.file_names.push("-".into());
    }

    Ok(settings)
}

enum QueercatFatalError {
    BadCommandLine(String),
    IoError(std::io::Error)
}

impl From<std::io::Error> for QueercatFatalError {
    fn from(value: std::io::Error) -> Self {
        QueercatFatalError::IoError(value)
    }
}

impl std::fmt::Debug for QueercatFatalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use QueercatFatalError::*;
        match self {
            BadCommandLine(msg) => {
                writeln!(f, "{}", msg)?;
                writeln!(f, "Try 'queercat --help' for more information.")
            }
            IoError(e) => e.fmt(f),
        }
    }
}

fn main() -> Result<(), QueercatFatalError> {
    let settings = match parse_args(std::env::args()) {
        Ok(s) => s,
        Err(ParseArgsFail::PrintUsage(msg)) =>
            return Err(QueercatFatalError::BadCommandLine(msg)),
        Err(ParseArgsFail::PrintVersion) => {
            print_version();
            return Ok(());
        }
    };

    let rand_offset = if settings.enable_rand_offset {
        get_fake_random() as i32
    } else {
        0
    };

    //struct timeval tv;
    //gettimeofday(&tv, NULL);
    //double offx = (tv.tv_sec % 300) / 300.0;

    /* Handle randomness. */
    //int rand_offset = 0;
    //if (random) {
    //    srand(time(NULL));
    //    rand_offset = rand();
    //}

    /* Handle locale. */
    /* // don't *think* we actually need/care about this?
    char* env_lang = getenv("LANG");
    if (force_locale && env_lang && !strstr(env_lang, "UTF-8")) {
        if (!setlocale(LC_ALL, "C.UTF-8")) { /* C.UTF-8 may not be available on all platforms */
            setlocale(LC_ALL, ""); /* Let's hope for the best */
        }
    } else {
        setlocale(LC_ALL, "");
    }
    */

    /* TODO -- will require modification of print_color
    fn colorizer(src: impl Iterator<Item = char>, settings: Settings) -> impl Iterator<Item = char>
    {
        let mut n = 0;
        src.flat_map(move |c| {
            n += 1;
            format!["{c}{n}"] .chars().into_iter()
        })
    }
    */

    use std::fs::File;
    use std::io::{self, Read};

    let files: Box<dyn Iterator<Item = io::Result<Box<dyn Read>>>> =
        if settings.print_help
        {
            let r: Box<dyn Read> = Box::new(io::Cursor::new(build_helpstr()));
            Box::new(std::iter::once(Ok(r)))
        }
        else
        {
            let file_iterator = settings.file_names.iter().map(|filename| -> io::Result<Box<dyn Read>> {
                match filename.as_str() {
                    "-" => Ok(Box::new(io::stdin())),
                    _ => Ok(Box::new(File::open(filename)?))
                }
            });
            Box::new(file_iterator)
        };

    for file in files {
        if !settings.enable_color {
            let mut reader = file?;
            let _ = io::copy(&mut reader, &mut io::stdout())?;
            continue;
        }

        use std::io::{BufRead, BufReader};

        let mut reader = BufReader::new(file?);
        let mut line_index = 0;
        let mut escape_state = EscapeState::Out;

        let Settings {
            flag: pattern,
            horiz_freq: freq_h,
            vert_freq: freq_v,
            horiz_offset: offx,
            ref color_type,
            ..
        } = settings;

        let mut line: String = Default::default();
        while let Ok(read) = reader.read_line(&mut line) {
            if read == 0 {
                break;
            }

            for (char_index, current_char) in line.chars().enumerate() {
                let char_index = char_index as u32;

                find_escape_sequences(current_char, &mut escape_state);

                if escape_state == EscapeState::Out {
                    print_color(pattern, color_type, char_index, line_index, freq_h, freq_v, offx, rand_offset);
                }

                print!("{current_char}");

                if escape_state == EscapeState::Last {
                    print_color(pattern, color_type, char_index, line_index, freq_h, freq_v, offx, rand_offset);
                }
            }

            line_index += 1;
            line.clear();
        }
        print!("{}[0m", ESCAPE_CHAR);
    }

    Ok(())
}
