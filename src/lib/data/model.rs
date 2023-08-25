use crate::data::DbId;
use crate::{ClipError, ShortCode, Time};
use chrono::{NaiveDateTime, Utc};
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Debug, sqlx::FromRow)]
pub struct Clip {
    pub(in crate::data) clip_id: String,
    pub(in crate::data) shortcode: String,
    pub(in crate::data) content: String,
    pub(in crate::data) title: Option<String>,
    pub(in crate::data) posted: NaiveDateTime,
    pub(in crate::data) expires: Option<NaiveDateTime>,
    pub(in crate::data) password: Option<String>,
    pub(in crate::data) hits: i64,
}

impl TryFrom<Clip> for crate::domain::clip::Clip {
    type Error = ClipError;
    fn try_from(value: Clip) -> Result<Self, Self::Error> {
        use crate::domain::clip::field;
        Ok(Self {
            clip_id: field::ClipId::new(DbId::from_str(value.clip_id.as_str())?),
            shortcode: field::ShortCode::from(value.shortcode),
            content: field::Content::new(value.content.as_str())?,
            title: field::Title::new(value.title),
            posted: field::Posted::new(Time::from_naive_utc(value.posted)),
            expires: field::Expires::new(value.expires.map(Time::from_naive_utc)),
            password: field::Password::new(value.password.unwrap_or_default())?,
            hits: field::Hits::new(u64::try_from(value.hits)?),
        })
    }
}
