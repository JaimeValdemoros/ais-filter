fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut parser = ais::AisParser::new();
    for line in std::io::stdin().lines() {
        let line = line?;
        let parsed = parser.parse(line.as_bytes(), true)?;
        eprintln!("{parsed:#?}");
        println!("{line}");
    }
    Ok(())
}
