use clap::Parser;
use rcli::{process_csv, process_genpass, Opts, Subcommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(
                &opts.input,
                opts.header,
                &opts.delimiter,
                output,
                opts.format,
            )?
        }
        Subcommand::GenPass(opts) => {
            let _ = process_genpass(
                opts.length,
                !opts.no_upper,
                !opts.no_lower,
                !opts.no_digits,
                !opts.no_symbol,
            );
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 2 + 2;
        // The assert_eq! macro checks if two values are equal
        assert_eq!(result, 4);
    }
}
