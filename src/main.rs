use structopt::StructOpt;
use anyhow::Result;
use grrs::*;

fn main() -> Result<()> {
    let args = Cli::from_args();
    
    work(&args)?;
    
    Ok(())
    
    // 使用 BuffReader : BuffRead 可以高效地按行读取
    // for line in reader.lines() {
    //     match line {
    //         Ok(line_content) => {
    //             if line_content.contains(&args.pattern) {
    //                 println!("{}", line_content);
    //             }
    //         }
    //         Err(e) => {
    //             eprintln!("{}", e);
    //             exit(1);
    //         }
    //     }
    // }


    // let content = fs::read_to_string(&args.path).expect("could not read file");
    //
    // for line in content.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("{}", line);
    //     }
    // }

}
