use chrono::{DateTime, Utc};

pub struct Image {
    pub path: String,
    pub original_create_date: Option<DateTime<Utc>>,
}
