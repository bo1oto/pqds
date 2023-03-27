use std::{
    fs, path::Path,
};

pub mod stribog;

use stribog::*;
use clap::Parser;

/// Compute hash using "Stribog" algorithm (GOST 34-11-12)
#[derive(Parser, Debug)]
struct Args {
    /// Path to file
    #[arg(short, long)]
    file: Option<String>,

    /// Text
    #[arg(short, long)]
    text: Option<String>,

    /// Hash length
    #[arg(short, long)]
    length: u8,
}

#[allow(dead_code)]
fn parse_args() -> (Vec<u8>, u8) {
    let args = Args::parse();
    (
        match (args.file, args.text) {
            (Some(_), Some(_)) => {
                eprintln!("You can hash only file or text");
                std::process::exit(1);
            },
            (Some(val), None) => {
                let path = Path::new(&val);
                if let Ok(bytes) = fs::read(path) {
                    bytes
                } else {
                    eprintln!("Can't open file at {}", val);
                    std::process::exit(1);
                }
            },
            (None, Some(text)) => text.into_bytes(),
            (None, None) => {
                eprintln!("You can hash only file or text");
                std::process::exit(1);
            },
        },
        args.length
    )
}

/*
    Implementation use file in BigEndian byte order, as in GOST 34.11
    streebog from RustCrypto lib use LittleEndian, so constants like hash are different
    To get the same hash you need `file.reverse()`
*/
fn main() {
    //let (args, hash_length) = parse_args();

    let file = fs::read("./README.md").unwrap();
    let mut hasher = Stribog::new(HashSize::L512);
    stribog(&mut hasher, &file, file.len());
    println!("H^512: ");
    hasher.print_512();

    hasher = Stribog::new(HashSize::L256);
    stribog(&mut hasher, &file, file.len());
    println!("H^256: ");
    hasher.print_256();
}
