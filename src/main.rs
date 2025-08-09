use clap::Parser;
use clap_duration::duration_range_value_parse;
use duration_human::{DurationHuman, DurationHumanValidator};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(
        short,
        long,
        value_parser = duration_range_value_parse!(min: 1s, max: 1h)
    )]
    sample: Option<DurationHuman>,
    #[arg(short, long, default_value_t = false)]
    quiet: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Cli::parse();
    log::debug!("{:?}", args.sample);

    let mut partial: Vec<String> = vec![];
    let mut parser = ais::AisParser::new();
    for line in std::io::stdin().lines() {
        let line = line?;
        match parser.parse(line.as_bytes(), true) {
            Ok(ais::AisFragments::Complete(c)) => {
                if !args.quiet {
                    eprintln!("{c:#?}");
                }
                if c.is_fragment() {
                    for p in partial.drain(0..) {
                        println!("{p}");
                    }
                }
                println!("{line}");
            }
            Ok(ais::AisFragments::Incomplete(_)) => {
                partial.push(line);
            }
            Err(e) => {
                // Log error and reset parser
                log::error!("{e}");
                parser = Default::default();
            }
        }
    }

    Ok(())
}
