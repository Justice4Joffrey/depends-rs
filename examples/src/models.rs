use std::io::{Error, ErrorKind};

use chrono::{DateTime, Utc};
use csv::StringRecord;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub id: i64,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Post {
    pub id: i64,
    #[serde(with = "date_format")]
    pub ts: DateTime<Utc>,
    pub content: String,
    pub submitted_id: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Comment {
    pub id: i64,
    #[serde(with = "date_format")]
    pub ts: DateTime<Utc>,
    pub content: String,
    pub submitted_id: i64,
    pub parent_id: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Like {
    pub user_id: i64,
    pub comment_id: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Friend {
    pub user_1_id: i64,
    pub user_2_id: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExpectedResult {
    pub view: String,
    pub changeset: u32,
    pub iteration: u32,
    pub phase_name: String,
    pub metric_value: String,
}

#[derive(Debug, Clone)]
pub enum Update {
    Users(User),
    Posts(Post),
    Comments(Comment),
    Likes(Like),
    Friends(Friend),
}

impl TryFrom<StringRecord> for Update {
    type Error = csv::Error;

    fn try_from(value: StringRecord) -> Result<Self, Self::Error> {
        let mut iter = value.into_iter();
        let type_str = iter
            .next()
            .ok_or(Error::new(ErrorKind::InvalidData, "Missing type string"))?;
        let record = StringRecord::from_iter(iter);
        Ok(match type_str {
            "Users" => Update::Users(record.deserialize(None)?),
            "Posts" => Update::Posts(record.deserialize(None)?),
            "Comments" => Update::Comments(record.deserialize(None)?),
            "Likes" => Update::Likes(record.deserialize(None)?),
            "Friends" => Update::Friends(record.deserialize(None)?),
            type_str => {
                Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Unknown type string {}", type_str),
                ))?
            }
        })
    }
}

mod date_format {
    use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, FORMAT)
            .map(|dt| Utc.from_utc_datetime(&dt))
            .map_err(serde::de::Error::custom)
    }
}
