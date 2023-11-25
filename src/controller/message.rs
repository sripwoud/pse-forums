use askama::Template;
use axum::{
    extract::Path,
    headers::Cookie,
    response::{IntoResponse, Redirect},
    Form, TypedHeader,
};
use serde::Deserialize;

use crate::{controller::fmt::clean_html, error::AppError, DB};

use super::{
    db_utils::u32_to_ivec,
    meta_handler::{into_response, PageData},
    Claim, SiteConfig,
};

/// Page data: `message.html`
#[derive(Template)]
#[template(path = "message.html", escape = "none")]
struct PageMessage<'a> {
    page_data: PageData<'a>,
    pub_key: String,
    receiver_id: u32,
}

/// `GET /message/:uid`
pub(crate) async fn message(
    cookie: Option<TypedHeader<Cookie>>,
    Path(uid): Path<u32>,
) -> Result<impl IntoResponse, AppError> {
    let cookie = cookie.ok_or(AppError::NonLogin)?;
    let site_config = SiteConfig::get(&DB)?;
    let claim = Claim::get(&DB, &cookie, &site_config).ok_or(AppError::NonLogin)?;

    if DB
        .open_tree("pub_keys")?
        .get(u32_to_ivec(claim.uid))?
        .is_none()
    {
        return Err(AppError::Custom(
            "You have not generated key pairs. In order to receive<a href= '/key'>Generate Key Pairs</a>".to_string(),
        ));
    }

    let Some(pub_key) = DB.open_tree("pub_keys")?.get(u32_to_ivec(uid))? else {
        return Err(AppError::Custom(
            "User has not generated key pairs".to_string(),
        ));
    };

    let title = format!("Sending e2ee Message to {}", uid);

    let page_message = PageMessage {
        receiver_id: uid,
        page_data: PageData::new(&title, &site_config, Some(claim), false),
        pub_key: String::from_utf8_lossy(&pub_key).to_string(),
    };

    Ok(into_response(&page_message))
}

/// Page data: `key.html`
#[derive(Template)]
#[template(path = "key.html", escape = "none")]
struct PageKey<'a> {
    page_data: PageData<'a>,
    pub_key: String,
}

/// `GET /key`
pub(crate) async fn key(
    cookie: Option<TypedHeader<Cookie>>,
) -> Result<impl IntoResponse, AppError> {
    let cookie = cookie.ok_or(AppError::NonLogin)?;
    let site_config = SiteConfig::get(&DB)?;
    let claim = Claim::get(&DB, &cookie, &site_config).ok_or(AppError::NonLogin)?;

    let pub_key = DB
        .open_tree("pub_keys")?
        .get(u32_to_ivec(claim.uid))?
        .map(|r| String::from_utf8_lossy(&r).to_string())
        .unwrap_or_default();

    let page_key = PageKey {
        page_data: PageData::new("Generate Key Pairs", &site_config, Some(claim), false),
        pub_key,
    };

    Ok(into_response(&page_key))
}

/// Form data: `/key`
#[derive(Deserialize)]
pub(crate) struct FormKey {
    pub_key: String,
}

/// `POST /key`
pub(crate) async fn key_post(
    cookie: Option<TypedHeader<Cookie>>,
    Form(input): Form<FormKey>,
) -> Result<impl IntoResponse, AppError> {
    let cookie = cookie.ok_or(AppError::NonLogin)?;
    let site_config = SiteConfig::get(&DB)?;
    let claim = Claim::get(&DB, &cookie, &site_config).ok_or(AppError::NonLogin)?;

    let pub_key = clean_html(&input.pub_key);

    DB.open_tree("pub_keys")?
        .insert(u32_to_ivec(claim.uid), pub_key.as_str())?;

    Ok(Redirect::to("/key"))
}