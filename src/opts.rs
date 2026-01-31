use std::{fmt, path::Path, str::FromStr};

use clap::{ArgAction, Parser};

#[derive(Debug, Parser)]
#[command(
    disable_help_flag = true,
    author,
    version,
    about,
    long_about = None,
    help_template = "\
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage}

{all-args}{after-help}
"
)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
    Toml,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(
        name = "csv",
        about = "Show CSV, or Convert CSV to Json or YAML or Toml formats"
    )]
    Csv(CvsOpts),

    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
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

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
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
