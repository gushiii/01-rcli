use clap::{ArgAction, Parser};

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    /// Password length
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,

    /// Disable uppercase letters [A-Z]
    #[arg(long)]
    pub no_upper: bool,

    /// Disable lowercase letters [a-z]
    #[arg(long)]
    pub no_lower: bool,

    /// Disable digits [0-9]
    #[arg(long)]
    pub no_digits: bool,

    /// Disable special symbol [!@#$%^&*_.?]
    #[arg(long)]
    pub no_symbol: bool,

    /// Print help (uses --help only)
    #[arg(long, action = ArgAction::Help, help = "Print help information")]
    pub help: Option<bool>,
}
