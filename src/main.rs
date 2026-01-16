use clap::Parser;
use rcli::{process_csv, Opts, Subcommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(opts) => process_csv(&opts.input, &opts.output)?,
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
