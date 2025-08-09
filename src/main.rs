use std::collections::HashSet;
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
    #[arg(short, long, default_value_t = false)]
    quiet: bool,
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
            filter: Default::default() ,
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
    env_logger::init();
    let args = Cli::parse();

    let mut sample_cfg = args.sample.as_ref().map(Sample::new);

    let mut partial: Vec<String> = vec![];
    let mut parser = ais::AisParser::new();

    for line in std::io::stdin().lines() {
        let line = line?;
        match parser.parse(line.as_bytes(), true) {
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
                        println!("{p}");
                    }
                }
                println!("{line}");
            }
            Ok(AisFragments::Incomplete(_)) => {
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
