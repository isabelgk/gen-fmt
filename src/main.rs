use clap::Parser;
use std::{
    fs,
    io::{self, Read, Write},
    path::PathBuf,
};

#[derive(Parser)]
#[command(about = "Format .genexpr files")]
struct Args {
    /// Files to format (reads stdin if omitted)
    files: Vec<PathBuf>,
    /// Write result back to file in place
    #[arg(short = 'i', long)]
    in_place: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.files.is_empty() {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input)?;
        let formatted = gen_fmt::format_str(&input)?;
        io::stdout().write_all(formatted.as_bytes())?;
    } else {
        for path in &args.files {
            let input = fs::read_to_string(path)?;
            let formatted = gen_fmt::format_str(&input)?;
            if args.in_place {
                fs::write(path, &formatted)?;
            } else {
                io::stdout().write_all(formatted.as_bytes())?;
            }
        }
    }

    Ok(())
}
