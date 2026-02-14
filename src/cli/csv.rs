use clap::{ArgAction, Parser};
use std::{fmt, str::FromStr};

use super::verify_input_file;

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
    Toml,
}

#[derive(Debug, Parser)]
pub struct CvsOpts {
    /// Input file path
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,

    /// Output file path
    #[arg(short, long)]
    pub output: Option<String>,

    /// Output file Format
    #[arg(short, long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,

    /// Delimiter
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    /// CSV has header or not
    #[arg(short, long, default_value_t = true)]
    pub header: bool,

    /// Print help (uses --help only)
    #[arg(long, action = ArgAction::Help, help = "Print help information")]
    pub help: Option<bool>,
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}

impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            OutputFormat::Toml => "toml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(format: &str) -> Result<Self, Self::Err> {
        match format {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            "toml" => Ok(OutputFormat::Toml),
            v => anyhow::bail!("Unsupported format; {}", v),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
