use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::Parser;

type CatResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
#[clap(version, about = "Rusty cat")]
pub struct Config {
    #[arg(default_value = "-")]
    files: Vec<String>,
    #[arg(short, long, help = "Number lines")]
    number: bool,
    #[arg(short = 'b', long, help = "Number non-blank lines")]
    number_nonblank: bool,
}

pub fn get_args() -> CatResult<Config> {
    Ok(Config::parse())
}

fn open(path: &str) -> CatResult<Box<dyn BufRead>> {
    match path {
        "-" => Ok(Box::new(io::stdin().lock())),
        _ => Ok(Box::new(BufReader::new(File::open(path)?))),
    }
}

pub fn run(cfg: Config) -> CatResult<()> {
    let mut count = 0;
    let mut count_string = || {
        count += 1;
        format!("{:>6}\t", count)
    };
    let mut line = String::new();

    for path in &cfg.files {
        match open(&path) {
            Err(err) => eprintln!("cat: Failed to open {}: {}", path, err),
            Ok(mut reader) => {
                // Use .read_line() to preserve line endings.
                while 0 < reader.read_line(&mut line)? {
                    print!(
                        "{}{}",
                        if cfg.number_nonblank {
                            if "\n" == line || "\r\n" == line {
                            "".to_string()
                            } else {
                                count_string()
                            }
                        } else if cfg.number {
                            count_string()
                        } else {
                            "".to_string()
                        },
                        line
                    );

                    line.clear();
                }
            }
        }
    }

    Ok(())
}

// [>[>+>+<<-]>[<+>-]<<-]
