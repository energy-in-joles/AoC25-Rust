use clap::Parser;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct AoCArgs {
    /// Sample input
    #[arg(short)]
    pub s: bool,

    /// Part number (1 or 2)
    #[arg(short)]
    pub p: Option<u8>,
}

pub fn cli_parse() -> AoCArgs {
    AoCArgs::parse()
}

pub fn read_input_lines(is_sample: bool) -> io::Result<impl Iterator<Item = String>> {
    let exe_path = env::current_exe()?; // temporary lives here
    let exe_name = exe_path
        .file_stem()
        .and_then(|s| s.to_str())
        .expect("Failed to get exe name");

    let day: u8 = exe_name[3..]
        .parse()
        .expect("Executable name must be like dayXX");

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src");
    if is_sample {
        path.push("samples");
    } else {
        path.push("inputs");
    }
    path.push(format!("day{:02}.txt", day));
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(reader
        .lines()
        .map(|line| line.expect("Failed to read line")))
}
