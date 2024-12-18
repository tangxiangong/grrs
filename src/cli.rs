use std::fs;
use std::io::{self, Write, BufWriter, BufRead, BufReader};
use std::path::PathBuf;
use structopt::StructOpt;
use anyhow::{Context, Result};


/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
pub struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

pub fn work(args: &Cli) -> Result<()> {
    let path = args.path.to_str().unwrap();
    // println!("Searching '{}' in file '{}'...", args.pattern, path);
    let stdout = io::stdout();
    let mut handle = BufWriter::new(stdout.lock());
    // let file = fs::File::open(&args.path).expect("could not open the file");

    let file = fs::File::open(&args.path)
        .with_context(|| format!("could not open file {}", path))?;
    let reader = BufReader::new(file);
    let mut counter = 0;
    for (num, result) in reader.lines().enumerate() {
        let line = result.with_context(|| format!("could not read file {}", path))?;
        if line.contains(&args.pattern) {
            counter += 1;
            writeln!(handle, "No.{}, Line {}: \"{}\"", counter, num+1, line)
                .with_context(|| "error")?;
        }
    }
    handle.flush()?;
    Ok(())
}