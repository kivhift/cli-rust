use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::Parser;
use utf8parse::{Parser as Utf8Parser, Receiver};

type WcResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser)]
#[clap(version, about = "Rusty wc")]
pub struct Config {
    #[arg(default_value = "-", help = "Input file(s)")]
    files: Vec<String>,
    #[arg(short, long, help = "Print newline count(s)")]
    lines: bool,
    #[arg(short, long, help = "Print word count(s)")]
    words: bool,
    #[arg(short = 'c', long, help = "Print bytes count(s)")]
    bytes: bool,
    #[arg(short = 'm', long, help = "Print character count(s)")]
    chars: bool,
}

struct Counts {
    lines: usize,
    words: usize,
    bytes: usize,
    chars: usize,
}

impl Counts {
    fn new() -> Self {
        Self {
            lines: 0,
            words: 0,
            bytes: 0,
            chars: 0,
        }
    }

    fn accumulate(&mut self, other: &Self) {
        self.lines += other.lines;
        self.words += other.words;
        self.bytes += other.bytes;
        self.chars += other.chars;
    }
}

struct State {
    in_word: bool,
    count: Counts,
}

impl State {
    fn new() -> Self {
        Self {
            in_word: false,
            count: Counts::new(),
        }
    }
}

impl Receiver for State {
    fn codepoint(&mut self, c: char) {
        self.count.chars += 1;

        if '\n' == c {
            self.count.lines += 1;
        }

        if c.is_whitespace() {
            if self.in_word {
                self.in_word = false;
            }
        } else {
            if !self.in_word {
                self.in_word = true;
                self.count.words += 1;
            }
        }
    }

    fn invalid_sequence(&mut self) {}
}

fn count(reader: impl BufRead) -> WcResult<Counts> {
    let mut state = State::new();
    let mut parser = Utf8Parser::new();

    for byte in reader.bytes() {
        state.count.bytes += 1;
        parser.advance(&mut state, byte?);
    }

    Ok(state.count)
}

pub fn get_args() -> WcResult<Config> {
    let mut cfg = Config::parse();

    // If none of the flags are set, then count lines/words/bytes.
    if !(cfg.lines || cfg.words || cfg.bytes || cfg.chars) {
        cfg.lines = true;
        cfg.words = true;
        cfg.bytes = true;
    }

    Ok(cfg)
}

fn open(path: &str) -> WcResult<Box<dyn BufRead>> {
    match path {
        "-" => Ok(Box::new(io::stdin().lock())),
        _ => Ok(Box::new(BufReader::new(File::open(path)?))),
    }
}

pub fn run(cfg: Config) -> WcResult<()> {
    let mut total = Counts::new();
    let fmt = |cnt: Counts, name: &str| {
        let mut parts = Vec::with_capacity(5);

        if cfg.lines {
            parts.push(format!("{:>8}", cnt.lines));
        }
        if cfg.words {
            parts.push(format!("{:>8}", cnt.words));
        }
        if cfg.chars {
            parts.push(format!("{:>8}", cnt.chars));
        }
        if cfg.bytes {
            parts.push(format!("{:>8}", cnt.bytes));
        }

        parts.push(name.to_string());

        parts.join(" ")
    };

    for path in &cfg.files {
        match open(&path) {
            Err(err) => eprintln!("wc: Failed to open {}: {}", path, err),
            Ok(reader) => {
                if let Ok(counts) = count(reader) {
                    total.accumulate(&counts);
                    println!("{}", fmt(counts, path));
                } else {
                    eprintln!("Had trouble reading file");
                }
            }
        }
    }

    if cfg.files.len() > 1 {
        println!("{}", fmt(total, "total"));
    }

    Ok(())
}
