use clap::Parser;

#[derive(Parser, Debug)]
pub struct AoCArgs {
    /// Sample input
    #[arg(short)]
    pub s: bool,

    /// Part number (1 or 2)
    #[arg(short)]
    pub p: u8,
}

pub fn parse() -> AoCArgs {
    AoCArgs::parse()
}
