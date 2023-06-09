mod executor;
mod error;

use std::{env, fs};
use std::io::{Read, stdin};
use std::process::exit;
use crate::executor::{execute};
use crate::error::{Error, Result};

const HELP: &'static str = r#"Usage:
    bf (-h | --help)
    bf (-f | --file) <file>
    bf <code>...
    echo <code> | bf"#;

fn run() -> Result<()> {
    let mut args = env::args().skip(1);
    let program = match args.next().as_deref() {
        Some("-f" | "--file") => {
            let f = args.next().unwrap_or_else(|| {
                eprintln!("{}", HELP);
                exit(1);
            });
            fs::read(&f).map_err(|e| Error::FileRead(f, e))?
        }
        Some("-h" | "--help") => {
            println!("{}", HELP);
            exit(0)
        }
        Some(code) => {
            let mut vec = Vec::from(code);
            args.for_each(|a| vec.extend(a.as_bytes()));
            vec
        }
        None => {
            let mut vec = vec![];
            stdin().read_to_end(&mut vec).map_err(|e| Error::StdinError(e))?;
            vec
        }
    };

    execute(&program, &mut [0; 10000])
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        exit(1);
    }
}