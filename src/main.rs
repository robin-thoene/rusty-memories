use clap::Parser;
use infrastructure::exiftool::ExifToolResult;
use std::{fmt::Debug, path::PathBuf, process::Command};
use walkdir::WalkDir;

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
        let out = Command::new("exiftool")
            .arg("-json")
            .arg(entry.path())
            .output()
            .expect("TODO");
        if let Some(o) =
            serde_json::from_str::<Vec<ExifToolResult>>(&String::from_utf8_lossy(&out.stdout))
                .unwrap()
                .first()
        {
            println!(
                "file: {} - date: {} - ftype: {} - mtype: {}",
                entry.path().display(),
                o.original_create_date.is_some(),
                o.file_type,
                o.mime_type
            );
        }
    }
    Ok(())
}
