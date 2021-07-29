use rand::Rng;

use colored::*;
use structopt::clap;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(setting = clap::AppSettings::InferSubcommands,
            about="password generator in xkcd style")]
struct Cli {
    #[structopt(short = "s",
                long = "silent",
                help = "don't list chosen words")]
    silent: bool,
    #[structopt(
        short = "c",
        long = "special-chars",
        help = "disable special characters"
    )]
    not_special: bool,
    #[structopt(
        short = "l",
        long = "len",
        help = "password length in words",
        default_value = "4"
    )]
    len: u16,
    #[structopt(
        short = "f",
        long = "file",
        help = "get words from specific file",
        default_value = "./words-en.txt",
        parse(from_os_str)
    )]
    file: std::path::PathBuf,
    #[structopt(long = "lang",
                help = "word language (en, es)",
                default_value = "en")]
    lang: String,
}

const CHARSET: &[u8] = b"0123456789)(*&^%$#@!~'_-:;.,{}[]";
const COLORS: [&str; 7] = [
                            "blue",
                            "green",
                            "yellow",
                            "red",
                            "cyan",
                            "purple",
                            "white"
                          ];
fn main() {
    // read args
    let args = Cli::from_args();
    let psswd_len = args.len;

    let mut words = std::fs::read_to_string(&args.file)
                             .expect("could not read file");

    if &args.lang == "es" {
        let mut word_file = std::path::PathBuf::new();
        word_file.push("./words-es.txt");
        words = std::fs::read_to_string(&word_file)
                         .expect("could not read file");
    }

    // convert file to vector of slices
    let word_list = words.split('\n');
    let words_vec: Vec<&str> = word_list.collect();

    // select and words to password
    let words = select_words(words_vec, psswd_len);

    // print words if not silent
    if !args.silent {
        println!("remeber this words :)");
        for (i, word) in words.iter().enumerate() {
            println!("{}. {}", i + 1, word);
        }
    }

    // paint andd append words to password
    let mut psswd = paint_psswd(words);

    if args.not_special {
        println!("{}", psswd);
        return;
    }

    // add special chars to password
    for _ in 0..psswd_len * 2 {
        let idx_word = rand::thread_rng().gen_range(0..psswd.len());
        let idx_char = rand::thread_rng().gen_range(0..CHARSET.len());
        let rndm_char = String::from(CHARSET[idx_char] as char);
        psswd = format!(
            "{}{}{}",
            &psswd[..idx_word],
            &rndm_char[..],
            &psswd[idx_word..]
        );
    }
    println!("{}", psswd);
}

/// select and random words form vector of string slices
/// returns -> n words in a vector of slices
fn select_words(words: Vec<&str>, times: u16) -> Vec<&str> {
    let mut choosen_words: Vec<&str> = Vec::new();
    for _ in 0..times {
        let idx = rand::thread_rng().gen_range(0..words.len());
        choosen_words.push(&words[idx]);
    }
    choosen_words
}

/// paint and append words sent in a vector of slices
/// returns -> string w/words appended an colored
fn paint_psswd(words: Vec<&str>) -> String {
    let mut psswd = String::new();
    let mut i: usize = 0;
    for word in words {
        if i == COLORS.len() - 1 {
            i = 0;
        }
        psswd = format!("{}{}", &psswd[..], word.color(COLORS[i]));
        i += 1;
    }
    psswd
}
