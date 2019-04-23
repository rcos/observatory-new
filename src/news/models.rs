use chrono::NaiveDateTime;

use crate::schema::*;

#[derive(Debug, PartialEq, Queryable, Identifiable, Serialize)]
#[table_name = "news"]
pub struct NewsStory {
    pub id: i32,
    pub happened_at: NaiveDateTime,
    pub title: String,
    pub description: String,
    pub color: Option<String>,
}

#[derive(Debug, Default, FromForm, Insertable, AsChangeset)]
#[table_name = "news"]
pub struct NewNewsStory {
    pub happened_at: String,
    pub title: String,
    pub description: String,
    pub color: Option<String>,
}
