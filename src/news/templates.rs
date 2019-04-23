use super::models::*;
#[allow(unused_imports)]
use crate::templates::{filters, OptUser};

#[derive(Template)]
#[template(path = "news/news.html")]
pub struct NewsTemplate {
    pub logged_in: OptUser,
    pub stories: Vec<NewsStory>,
}

#[derive(Template)]
#[template(path = "news/newsstory.html")]
pub struct NewsStoryTemplate {
    pub logged_in: OptUser,
    pub story: NewsStory,
}

#[derive(Template)]
#[template(path = "news/new-newsstory.html")]
pub struct NewNewsStoryTemplate {
    pub logged_in: OptUser,
}

#[derive(Template)]
#[template(path = "news/edit-newsstory.html")]
pub struct EditNewsStoryTemplate {
    pub logged_in: OptUser,
    pub story: NewsStory,
}
