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

use crate::calendar::models::smart_time_parse;
impl NewNewsStory {
    pub fn fix_times(&mut self) -> Option<()> {
        self.happened_at = smart_time_parse(&self.happened_at)?
            .format("%F %R")
            .to_string();
        Some(())
    }
}
