use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
   #[arg(short, long)]
    sample: Option<String>
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Cli { sample: _sample } = Cli::parse();

    let mut parser = ais::AisParser::new();
    for line in std::io::stdin().lines() {
        let line = line?;
        let parsed = parser.parse(line.as_bytes(), true)?;
        eprintln!("{parsed:#?}");
        println!("{line}");
    }

    Ok(())
}
