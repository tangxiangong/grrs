use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;
use structopt::StructOpt;

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

    let file =
        fs::File::open(&args.path).with_context(|| format!("could not open file {}", path))?;
    let reader = BufReader::new(file);
    let mut counter = 0;
    let red_text = args.pattern.clone().red().to_string();

    for (num, result) in reader.lines().enumerate() {
        let line = result.with_context(|| format!("could not read file {}", path))?;
        if line.contains(&args.pattern) {
            let color_line = line.replace(&args.pattern, &red_text);
            counter += 1;
            writeln!(
                handle,
                "No.{}, Line {}: \"{}\"",
                counter,
                num + 1,
                color_line
            )
            .with_context(|| "error")?;
            writeln!(handle, "\t")?;
        }
    }
    handle.flush()?;
    Ok(())
}
