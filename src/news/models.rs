use chrono::NaiveDateTime;

use crate::schema::*;

#[derive(Debug, PartialEq, Clone, Queryable, Identifiable, Serialize)]
#[table_name = "news"]
pub struct NewsStory {
    pub id: i32,
    pub happened_at: NaiveDateTime,
    pub title: String,
    pub description: String,
    pub color: Option<String>,
    pub announcement: bool,
}

#[derive(Debug, Default, Clone, FromForm, Insertable, AsChangeset)]
#[table_name = "news"]
pub struct NewNewsStory {
    pub happened_at: String,
    pub title: String,
    pub description: String,
    pub color: Option<String>,
    pub announcement: bool,
}

impl NewNewsStory {
    pub fn check_times(&self) -> Result<(), chrono::ParseError> {
        NaiveDateTime::parse_from_str(&self.happened_at, "%F %R")
            .or(NaiveDateTime::parse_from_str(&self.happened_at, "%F %T"))
            .and(Ok(()))
    }
}
