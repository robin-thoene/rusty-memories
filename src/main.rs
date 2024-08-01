use clap::Parser;
use std::{fmt::Debug, path::PathBuf};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "TODO")]
    path: PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    for entry in WalkDir::new(args.path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        println!("{}", entry.path().display());
    }
    Ok(())
}
