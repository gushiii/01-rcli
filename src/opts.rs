use std::path::Path;

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

#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "Show CSV, or Convert CSV to other formats")]
    Csv(CvsOpts),
}

#[derive(Debug, Parser)]
pub struct CvsOpts {
    /// Input file path
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,

    /// Output file path
    #[arg(short, long, default_value = "output.json")]
    pub output: String,

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
