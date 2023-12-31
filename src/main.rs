mod flags;
use flags::FLAGS;

const ESCAPE_CHAR: char = '\x1b'; //'\033'

struct FlagDefinition {
    name: &'static str,
    ansii_pattern: ColorPattern_Ansii,
    color_pattern: twenty_four_bit_color::ColorPattern,
}

// TODO? replace below struct with:
//type ansii_pattern_t = &'static [u8];
struct ColorPattern_Ansii(&'static [u8]);

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

mod twenty_four_bit_color {

    pub(super) enum ColorPattern {
        Rainbow,
        Stripes(ColorStripes)
    }

    impl ColorPattern {
        pub(super) fn get_color(&self, theta: f32) -> RGBColor {
            use ColorPattern::*;
            match self {
                Rainbow =>
                    get_color_rainbow(theta),
                Stripes(patt) =>
                    get_color_stripes(patt, theta),
            }
        }
    }

    pub(super) struct ColorStripes {
        pub stripes: &'static [u32],
        pub factor: f32,
    }

    pub(super) struct RGBColor {
        pub red: u8,
        pub green: u8,
        pub blue: u8,
    }

    fn mix_colors(color1: u32, color2: u32, balance: f32, factor: f32) -> RGBColor {
        let balance = balance.powf(factor);

        #[allow(clippy::identity_op)]
        fn to_components(color: u32) -> [f32; 3] {
            let red   = ((color & 0xff0000) >> 16) as f32;
            let green = ((color & 0x00ff00) >>  8) as f32;
            let blue  = ((color & 0x0000ff) >>  0) as f32;
            [red, green, blue]
        }

        fn mix(c1: f32, c2: f32, balance: f32) -> u8 {
            (c1 * balance + c2 * (1.0 - balance)).round() as u8
        }

        let [r1, g1, b1] = to_components(color1);
        let [r2, g2, b2] = to_components(color2);

        let (red, green, blue) = (
            mix(r1, r2, balance),
            mix(g1, g2, balance),
            mix(b1, b2, balance),
        );

        RGBColor { red, green, blue }
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

    fn get_color_rainbow(theta: f32) -> RGBColor {
        use std::f32::consts::PI;
        let theta = clamp_theta(theta);

        let gen_color_component = |offset_factor: f32| -> u8 {
            ((1.0 * (0.5 + 0.5 * (theta + offset_factor * PI / 3.0).sin())) * 255.0).round() as u8
        };

        /* Generate the color. */
        let red = gen_color_component(0.0);
        let green = gen_color_component(2.0);
        let blue = gen_color_component(4.0);

        RGBColor { red, green, blue }
    }

    fn get_color_stripes(color_pattern: &ColorStripes, theta: f32) -> RGBColor {
        use std::f32::consts::PI;
        let theta = clamp_theta(theta);

        let stripes = color_pattern.stripes;
        let stripe_count = stripes.len();

        // TODO figure out how to calcualte this directly, w/out the loop
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

                return mix_colors(stripes[i], next_color, balance, color_pattern.factor);
            }
        }
        panic!["never found a color"];
    }
}

fn print_color(settings: &Settings, char_index: u32, line_index: u32, rand_offset: i32) {
    use self::OutputColorType::*;
    use std::f32::{consts::PI, MAX as f32MAX};

    let Settings {
        flag,
        horiz_freq,
        vert_freq,
        horiz_offset,
        ..
    } = settings;

    // TODO can we make this less gross?
    let char_index_f: f32 = char_index as f32;
    let line_index_f: f32 = line_index as f32;

    match settings.color_type {
        TwentyFourBit => {
            let theta =
                char_index_f * horiz_freq / 5.0
                + line_index_f * vert_freq
                + (horiz_offset + 2.0 * rand_offset as f32 / f32MAX) * PI;

            let color = flag.color_pattern.get_color(theta);

            print!("{}[38;2;{};{};{}m", ESCAPE_CHAR, color.red, color.green, color.blue);
        },

        Ansii => {
            let pat_codes = flag.ansii_pattern.0;
            let pat_code_count = pat_codes.len();

            let ncc = ((horiz_offset * (pat_code_count as f32)).round() as i32)
                + ((char_index_f * horiz_freq + line_index_f * vert_freq).trunc() as i32);

            let code_index = (rand_offset + ncc) as usize % pat_code_count;
            print!("{}[38;5;{}m", ESCAPE_CHAR, pat_codes[code_index]);
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

            //struct timeval tv;
            //gettimeofday(&tv, NULL);
            //double offx = (tv.tv_sec % 300) / 300.0;
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

    //int rand_offset = 0;
    //if (random) {
    //    srand(time(NULL));
    //    rand_offset = rand();
    //}
    let rand_offset = if settings.enable_rand_offset {
        get_fake_random() as i32
    } else {
        0
    };

    /* Handle locale. */ // don't *think* we actually need/care about this?
    /*
    char* env_lang = getenv("LANG");
    if (force_locale && env_lang && !strstr(env_lang, "UTF-8")) {
        if (!setlocale(LC_ALL, "C.UTF-8")) { /* C.UTF-8 may not be available on all platforms */
            setlocale(LC_ALL, ""); /* Let's hope for the best */
        }
    } else {
        setlocale(LC_ALL, "");
    }
    */

    /* TODO? revisit this idea of colorizing via an iterator
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

        let mut line: String = Default::default();
        while let Ok(read) = reader.read_line(&mut line) {
            if read == 0 {
                break;
            }

            for (char_index, current_char) in line.chars().enumerate() {
                let char_index = char_index as u32;

                find_escape_sequences(current_char, &mut escape_state);

                if escape_state == EscapeState::Out {
                    print_color(&settings, char_index, line_index, rand_offset);
                }

                print!("{current_char}");

                if escape_state == EscapeState::Last {
                    print_color(&settings, char_index, line_index, rand_offset);
                }
            }

            line_index += 1;
            line.clear();
        }
        print!("{}[0m", ESCAPE_CHAR);
    }

    Ok(())
}
