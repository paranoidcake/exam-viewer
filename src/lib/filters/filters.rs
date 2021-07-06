use super::super::template;
use super::util;

use std::path::PathBuf;

use serde_json::json;

use warp::Filter;
use warp::filters::BoxedFilter;

type JsonTemplate = template::Template<handlebars::JsonValue>;

pub fn index() -> BoxedFilter<(JsonTemplate,)> {
    warp::get().and(warp::path::end())
        .map(move || {
            template::Template::new("index", json!({
                "value": "test!"
            }))
        })
        .boxed()
}

pub fn exam_list() -> BoxedFilter<(JsonTemplate,)> {
    warp::get().and(warp::path!("papers"))
        .and(warp::path::end())
        .map(move || {
            template::Template::new("exam/subject_list", json!(
                util::get_exam_pdfs().unwrap_or_default()
            ))
        })
        .boxed()
}

pub fn exam_subject() -> BoxedFilter<(JsonTemplate,)> {
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
        .boxed()
}

pub fn styles() -> BoxedFilter<(warp::fs::File,)> {
    warp::path("styles")
        .and(warp::fs::dir(PathBuf::from("./src/styles")))
        .boxed()
}

pub fn scripts() -> BoxedFilter<(warp::fs::File,)> {
    warp::path("scripts")
        .and(warp::fs::dir(PathBuf::from("./src/scripts")))
        .boxed()
}

pub fn assets() -> BoxedFilter<(warp::fs::File,)> {
    warp::path("assets")
        .and(warp::fs::dir(PathBuf::from("./assets/")))
        .boxed()
}