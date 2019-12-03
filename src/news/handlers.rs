use std::io::Cursor;

use diesel::prelude::*;
use diesel::{delete, insert_into, update};
use rocket::http::{ContentType, Status};
use rocket::request::Form;
use rocket::response::{Redirect, Response};

use rocket_contrib::json::Json;

use crate::guards::*;
use crate::templates::{is_reserved, FormError};
use crate::ObservDbConn;

use super::models::*;
use super::templates::*;

#[get("/news")]
pub fn news(conn: ObservDbConn, l: MaybeLoggedIn) -> NewsTemplate {
    use crate::schema::news::dsl::*;
    NewsTemplate {
        logged_in: l.user(),
        stories: news
            .order(happened_at.desc())
            .load(&*conn)
            .expect("Failed to get news from database"),
    }
}

#[get("/news.json")]
pub fn news_json(conn: ObservDbConn, _l: MaybeLoggedIn) -> Json<Vec<NewsStory>> {
    use crate::schema::news::dsl::*;
    Json(
        news.order(happened_at.desc())
            .load(&*conn)
            .expect("Failed to get news from database"),
    )
}

#[get("/news.xml")]
pub fn news_rss(conn: ObservDbConn) -> Response<'static> {
    use crate::schema::news::dsl::*;
    use rss;

    let all_news: Vec<NewsStory> = news.load(&*conn).expect("Failed to get news from database");
    let items: Vec<rss::Item> = all_news
        .iter()
        .map(|story| {
            let link = format!("https://rcos.io/news/{}", &story.id);
            let mut guid = rss::Guid::default();
            guid.set_value(link.clone());

            rss::ItemBuilder::default()
                .title(story.title.clone())
                .description({
                    use askama_filters::filters::markdown;
                    markdown(&story.description.clone()).unwrap()
                })
                .link(link)
                .guid(guid)
                .pub_date(story.happened_at.format("%a, %d %b %Y %T EST").to_string())
                .build()
                .expect("Failed to build RSS Item")
        })
        .collect();

    let xml = rss::ChannelBuilder::default()
        .title("RCOS News")
        .link("https://rcos.io")
        .description("News from the Rensselaer Center for Open Source")
        .items(items)
        .build()
        .expect("Failed to build RSS Channel")
        .to_string();

    Response::build()
        .status(Status::Ok)
        .header(ContentType::XML)
        .sized_body(Cursor::new(xml))
        .finalize()
}

#[get("/news/<nid>")]
pub fn story(conn: ObservDbConn, l: MaybeLoggedIn, nid: i32) -> NewsStoryTemplate {
    use crate::schema::news::dsl::*;
    NewsStoryTemplate {
        logged_in: l.user(),
        story: news
            .find(nid)
            .first(&*conn)
            .expect("Failed to get news event from database"),
    }
}

#[get("/news/new?<e>")]
pub fn story_new(_conn: ObservDbConn, l: AdminGuard, e: Option<FormError>) -> NewNewsStoryTemplate {
    NewNewsStoryTemplate {
        logged_in: Some(l.0),
        error: e,
    }
}

#[post("/news/new", data = "<newnewsstory>")]
pub fn story_new_post(
    conn: ObservDbConn,
    _l: AdminGuard,
    newnewsstory: Form<NewNewsStory>,
) -> Redirect {
    use crate::schema::news::dsl::*;

    let newnewsstory = newnewsstory.into_inner();
    if newnewsstory.check_times().is_err() {
        return Redirect::to(format!("/news/new?e={}", FormError::InvalidDate));
    }
    if let Err(e) = is_reserved(&newnewsstory.title) {
        return Redirect::to(format!("/news/new?e={}", e));
    }

    insert_into(news)
        .values(&newnewsstory)
        .execute(&*conn)
        .expect("Failed to insert news story into database");

    Redirect::to("/news")
}

#[get("/news/<nid>/edit?<e>")]
pub fn story_edit(
    conn: ObservDbConn,
    l: AdminGuard,
    nid: i32,
    e: Option<FormError>,
) -> EditNewsStoryTemplate {
    use crate::schema::news::dsl::*;
    EditNewsStoryTemplate {
        logged_in: Some(l.0),
        story: news
            .find(nid)
            .first(&*conn)
            .expect("Failed to load news story from database"),
        error: e,
    }
}

#[put("/news/<nid>", data = "<editnewsstory>")]
pub fn story_edit_put(
    conn: ObservDbConn,
    _l: AdminGuard,
    editnewsstory: Form<NewNewsStory>,
    nid: i32,
) -> Redirect {
    use crate::schema::news::dsl::*;

    let editnewsstory = editnewsstory.into_inner();
    if editnewsstory.check_times().is_err() {
        return Redirect::to(format!("/news/{}/edit?e={}", nid, FormError::InvalidDate));
    }
    if let Err(e) = is_reserved(&editnewsstory.title) {
        return Redirect::to(format!("/news/{}/edit?e={}", nid, e));
    }

    update(news.find(nid))
        .set(&editnewsstory)
        .execute(&*conn)
        .expect("Failed to update news story in database");

    Redirect::to(format!("/news/{}", nid))
}

#[delete("/news/<nid>")]
pub fn story_delete(conn: ObservDbConn, _l: AdminGuard, nid: i32) -> Redirect {
    use crate::schema::news::dsl::*;
    delete(news.find(nid))
        .execute(&*conn)
        .expect("Failed to delete news story from database");
    Redirect::to("/news")
}

#[get("/news/slides")]
pub fn news_slides(conn: ObservDbConn) -> SlidesTemplate {
    let (e, n) = news_summary(&*conn);
    SlidesTemplate { events: e, news: n }
}

use crate::models::Event;
pub fn news_summary(conn: &SqliteConnection) -> (Vec<Event>, Vec<NewsStory>) {
    (
        {
            use crate::schema::events::dsl::*;
            let now = chrono::offset::Local::now().format("%F %R").to_string();
            events
                .order(start.desc())
                .filter(start.gt(now))
                .limit(5)
                .load(&*conn)
                .expect("Failed to get news from database")
        },
        {
            use crate::schema::news::dsl::*;
            news.order(happened_at.desc())
                .limit(5)
                .load(&*conn)
                .expect("Failed to get news from database")
        },
    )
}
