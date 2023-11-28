use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Read, Write},
};

use clap::Parser;

type HeadResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
#[clap(version, about = "Rusty head")]
pub struct Config {
    #[arg(default_value = "-")]
    files: Vec<String>,
    #[arg(short = 'n', long, default_value = "10", help = "Line count")]
    lines: usize,
    #[arg(
        short = 'c',
        long,
        conflicts_with = "lines",
        default_value = "0",
        help = "Byte count"
    )]
    bytes: usize,
}

pub fn get_args() -> HeadResult<Config> {
    Ok(Config::parse())
}

fn open(path: &str) -> HeadResult<Box<dyn BufRead>> {
    match path {
        "-" => Ok(Box::new(io::stdin().lock())),
        _ => Ok(Box::new(BufReader::new(File::open(path)?))),
    }
}

pub fn run(cfg: Config) -> HeadResult<()> {
    let want_bytes = cfg.bytes > 0;
    let want_lines = cfg.lines > 0;
    let need_headers = cfg.files.len() > 1;
    let mut need_blank_line = false;
    let mut bytes = Vec::with_capacity(cfg.bytes);
    let mut line = String::new();

    for path in &cfg.files {
        match open(&path) {
            Err(err) => eprintln!("head: Failed to open {}: {}", path, err),
            Ok(mut reader) => {
                if need_headers {
                    println!(
                        "{}==> {} <==",
                        if need_blank_line {
                            "\n"
                        } else {
                            need_blank_line = true;
                            ""
                        },
                        if "-" == path { "standard input" } else { path }
                    );
                }

                if want_bytes {
                    let mut stdout = io::stdout().lock();

                    reader.take(cfg.bytes as u64).read_to_end(&mut bytes)?;
                    stdout.write_all(bytes.as_slice())?;
                    bytes.clear();
                } else if want_lines {
                    for _ in 0..cfg.lines {
                        if 0 == reader.read_line(&mut line)? {
                            break;
                        }

                        print!("{}", line);
                        line.clear();
                    }
                }
            }
        }
    }

    Ok(())
}
