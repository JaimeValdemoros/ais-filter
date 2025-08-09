use std::collections::HashSet;
use std::io::BufRead;
use std::time::{Duration, Instant};

use ais::sentence::{AisFragments, AisSentence};
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
    #[command(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
    #[arg(short, long, default_value = "false")]
    decode: bool,
}

struct Sample {
    duration: Duration,
    filter: HashSet<Vec<u8>>,
    next_tick: Instant,
}

impl Sample {
    fn new(duration: &DurationHuman) -> Self {
        let duration = duration.into();
        Sample {
            duration,
            filter: Default::default(),
            next_tick: Instant::now() + duration,
        }
    }

    fn check(&mut self, sentence: AisSentence) -> bool {
        if self.next_tick < Instant::now() {
            log::debug!("Resetting sample filter");
            self.filter.clear();
            self.next_tick += self.duration;
        }
        !self.filter.insert(sentence.data)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbosity.log_level_filter())
        .init();

    let mut sample_cfg = args.sample.as_ref().map(Sample::new);

    let mut partial: Vec<String> = vec![];
    let mut parser = ais::AisParser::new();

    let mut line = String::new();
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    loop {
        line.clear();
        if handle.read_line(&mut line)? == 0 {
            break;
        }
        match parser.parse(line.as_bytes(), args.decode) {
            Ok(AisFragments::Complete(c)) => {
                log::debug!("{c:?}");
                let is_fragment = c.is_fragment();
                if sample_cfg.as_mut().map_or(false, |s| s.check(c)) {
                    log::debug!("Duplicate message, skipping");
                    partial.clear();
                    continue;
                }
                if is_fragment {
                    for p in partial.drain(0..) {
                        println!("{}", p.trim_end());
                    }
                }
                println!("{}", line.trim_end());
            }
            Ok(AisFragments::Incomplete(_)) => {
                partial.push(std::mem::take(&mut line));
            }
            Err(e) => {
                // Log error and reset parser
                log::error!("{} -> {e}", line.trim_end());
                parser = Default::default();
                partial.clear();
            }
        }
    }

    Ok(())
}
