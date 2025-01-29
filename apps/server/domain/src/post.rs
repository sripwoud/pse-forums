use crate::content::Content;
use chrono::{DateTime, FixedOffset};

pub struct Post {
    pub id: i32,
    pub content: Content,
    pub created_at: DateTime<FixedOffset>,
    pub gid: Option<i32>,
    pub uid: Option<i32>,
    pub title: String,
    pub tags: Vec<String>,
}

impl From<db::post::Data> for Post {
    fn from(data: db::post::Data) -> Self {
        Self {
            id: data.id,
            content: data.content.try_into().unwrap(), // if it is in DB, it's already validated
            created_at: data.created_at,
            gid: data.gid,
            uid: data.uid,
            title: data.title.try_into().unwrap(), // if it is in DB, it's already validated
            tags: data.tags,
        }
    }
}
