use std::{path::Path, process::Command};

use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::core::models::Image;

/// Extracts exif data from an image using the **exiftool** and returns the domain representation
/// of the image.
///
/// # Arguments
///
/// * `path` - The path to the file for that the **exiftool** shall extract the exif data.
///
/// # Examples
///
/// ```
/// [TODO:write some example code]
/// ```
pub fn get_image_exif_data(path: &Path) -> Option<Image> {
    // Invoke the exiftool on the given path and get it's output in the JSON format.
    let exiftool_output = Command::new("exiftool")
        .arg("-json")
        .arg(path)
        .output()
        .expect("TODO");
    // Get the string value of the output.
    let output_str = &String::from_utf8_lossy(&exiftool_output.stdout);
    // Extract the data and convert it to the domain image model.
    // TODO: Improve error handling.
    serde_json::from_str::<Vec<ExifToolResult>>(output_str)
        .unwrap()
        .first()
        .map(|x| Image {
            path: path.display().to_string(),
            original_create_date: x.original_create_date,
        })
}

/// Structure to parse the output of the **exiftool**.
#[derive(Deserialize, Debug)]
pub struct ExifToolResult {
    #[serde(rename(deserialize = "FileType"))]
    pub file_type: String,
    #[serde(rename(deserialize = "MIMEType"))]
    pub mime_type: String,
    #[serde(
        rename(deserialize = "DateTimeOriginal"),
        with = "date_formatter",
        default
    )]
    pub original_create_date: Option<DateTime<Utc>>,
}

/// Custom module to provide the date formatter to parse **exiftool** date output.
mod date_formatter {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{Deserialize, Deserializer};

    /// The format in which the exiftool returns the date data.
    const FORMAT: &str = "%Y:%m:%d %H:%M:%S";

    /// Custom deserialize function to parse a date from the **exiftool** cli output.
    ///
    /// # Arguments
    ///
    /// * `deserializer` - The given **serde** deserializer.
    ///
    /// # Errors
    ///
    /// Potential **serde** deserialization error.
    ///
    /// # Examples
    ///
    /// ```
    /// [TODO:write some example code]
    /// ```
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
