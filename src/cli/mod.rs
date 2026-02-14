mod base64;
mod csv;
mod genpass;

use std::path::Path;

use self::{csv::CvsOpts, genpass::GenPassOpts};
use clap::Parser;

pub use self::{
    base64::{Base64Format, Base64SubCommand},
    csv::OutputFormat,
};

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
    #[command(
        name = "csv",
        about = "Show CSV, or Convert CSV to Json or YAML or Toml formats"
    )]
    Csv(CvsOpts),

    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),

    #[command(subcommand)]
    Base64(Base64SubCommand),
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

#[cfg(test)]
mod tests {
    use super::verify_input_file;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("*"), Err("File does not exist"));
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_input_file("no-exist"), Err("File does not exist"));
    }
}
