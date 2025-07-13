use lambda_http::http::StatusCode;
use lumx_axum::{axum::Json, extractor::Component};
use serde::Serialize;

use crate::components::ArticleQuery;

#[derive(Serialize)]
pub struct ArticlePage {}

pub async fn read_articles(
    Component(query): Component<ArticleQuery>,
) -> Result<Json<ArticlePage>, StatusCode> {
    let paged = query.find_all_paginated();

    Ok(Json(ArticlePage {}))
}
