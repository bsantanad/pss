use rand::Rng;

use colored::*;
use structopt::clap;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(setting = clap::AppSettings::InferSubcommands, 
            about="password generator in xkcd style")]
struct Cli {
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
        default_value = "./words.txt",
        parse(from_os_str)
    )]
    file: std::path::PathBuf,
}

const CHARSET: &[u8] = b"0123456789)(*&^%$#@!~";

fn main() {
    // read args
    let args = Cli::from_args();
    let psswd_len = args.len;

    // convert file to vector of slices
    let words = std::fs::read_to_string(&args.file)
                        .expect("could not read file");
    let word_list = words.split('\n');
    let words_vec: Vec<&str> = word_list.collect();

    // select and append words to password
    let (tmp_psswd, words) = append_words(words_vec, psswd_len);
    if args.not_special {
        println!("{}", tmp_psswd);
        return;
    }

    // add special char to password
    let mut psswd = String::from(tmp_psswd);
    for _ in 0..words.len() * 2 {
        let idx_word = rand::thread_rng().gen_range(0..psswd.len());
        let idx_char = rand::thread_rng().gen_range(0..CHARSET.len());
        psswd = format!(
            "{}{}{}",
            &psswd[..idx_word],
            CHARSET[idx_char] as char,
            &psswd[idx_word..]
        );
    }
    println!("{}", psswd);
}

/// select and append random words form vector of string slices
fn append_words(words: Vec<&str>, times: u16) -> (String, Vec<&str>) {
    let mut word = String::new();
    let mut choosen_words: Vec<&str> = Vec::new();
    for _ in 0..times {
        let idx = rand::thread_rng().gen_range(0..words.len());
        word.push_str(&words[idx]);
        choosen_words.push(&words[idx]);
    }
    (word, choosen_words)
}
