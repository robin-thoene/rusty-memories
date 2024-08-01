use chrono::{Date, DateTime, NaiveDate, NaiveDateTime, Utc};
use clap::Parser;
use serde::Deserialize;
use std::{fmt::Debug, path::PathBuf, process::Command};
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

#[derive(Deserialize, Debug)]
struct ExifToolResult {
    #[serde(rename(deserialize = "FileType"))]
    file_type: String,
    #[serde(rename(deserialize = "MIMEType"))]
    mime_type: String,
    #[serde(
        rename(deserialize = "DateTimeOriginal"),
        with = "date_formatter",
        default
    )]
    original_create_date: Option<DateTime<Utc>>,
}

mod date_formatter {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &str = "%Y:%m:%d %H:%M:%S";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = Option::<String>::deserialize(deserializer)?;
        if let Some(s) = s {
            let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
            Ok(Some(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc)))
        } else {
            Ok(None)
        }
    }
}
