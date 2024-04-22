use base64::{engine::general_purpose::STANDARD, Engine};
use clap::Parser;
use rcli::{process_csv, process_genpass, Base64SubCommand, Opts, Subcommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    println!("{:?}", opts);
    match opts.cmd {
        Subcommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        Subcommand::GenPass(opts) => {
            process_genpass(&opts)?;
        }
        Subcommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                let encoded = STANDARD.encode(&opts.input);
                println!("Encode: {} => {}", &opts.input, encoded);
            }
            Base64SubCommand::Decode(opts) => {
                let decoded = STANDARD.decode(&opts.input)?;
                println!("Decode: {} => {}", &opts.input, String::from_utf8(decoded)?);
            }
        },
    }
    Ok(())
}
