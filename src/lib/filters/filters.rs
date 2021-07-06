use super::super::template;
use super::util;

use std::path::PathBuf;

use serde_json::json;

use warp::Filter;

type JsonTemplate = template::Template<handlebars::JsonValue>;

pub fn index() -> impl Filter<Extract = (JsonTemplate,), Error = warp::Rejection> + Clone + Send + Sync + 'static {
    warp::get().and(warp::path::end())
    .map(move || {
        template::Template::new("index", json!({
            "value": "test!"
        }))
    })
}

pub fn exam_list() -> impl Filter<Extract = (JsonTemplate,), Error = warp::Rejection> + Clone + Send + Sync + 'static {
    warp::get().and(warp::path!("papers"))
        .and(warp::path::end())
        .map(move || {
            template::Template::new("exam/subject_list", json!(
                util::get_exam_pdfs().unwrap_or_default()
            ))
        })
}

pub fn exam_subject() -> impl Filter<Extract = (JsonTemplate,), Error = warp::Rejection> + Clone + Send + Sync + 'static {
    warp::get().and(warp::path!("papers" / String))
        .and(warp::path::end())
        .map(|param: String| {
            let param = param.replace("-", " ");

            // Gonna rely on caching for performance here
            let pdfs = util::get_exam_pdfs().unwrap_or_default();
            let key = pdfs.keys().find(|&key: &&std::string::String| {
                if key.to_lowercase() == param.to_lowercase() {
                    true
                } else {
                    false
                }
            }).expect("Key not found!");

            template::Template::new("exam/subject_full", json!({
                "heading": key,
                "contents": pdfs.get(key).unwrap()
            }))
        })
}

pub fn styles() -> impl Filter<Extract = (warp::fs::File,), Error = warp::Rejection> + Clone + Send + Sync + 'static {
    warp::path("styles")
        .and(warp::fs::dir(PathBuf::from("./src/styles")))
}

pub fn scripts() -> impl Filter<Extract = (warp::fs::File,), Error = warp::Rejection> + Clone + Send + Sync + 'static {
    warp::path("scripts")
        .and(warp::fs::dir(PathBuf::from("./src/scripts")))
}

pub fn assets() -> impl Filter<Extract = (warp::fs::File,), Error = warp::Rejection> + Clone + Send + Sync + 'static {
    warp::path("assets")
        .and(warp::fs::dir(PathBuf::from("./assets/")))
}