use clap::Parser;
use infrastructure::exiftool::{get_image_exif_data, ExifToolResult};
use std::{fmt::Debug, path::PathBuf, process::Command};
use walkdir::WalkDir;

mod core;
mod infrastructure;

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
        let image = get_image_exif_data(entry.path());
        if let Some(i) = image {
            let date_str = if let Some(d) = i.original_create_date {
                d.to_string()
            } else {
                "no date".to_string()
            };
            println!("{} - {}", i.path, date_str);
        }
    }
    Ok(())
}
