type JsonTemplate = lib::template::Template<handlebars::JsonValue>;

pub fn index() -> BoxedFilter<(JsonTemplate,)> {
    warp::path::end()
        .map(move || {
            lib::template::Template::new("index", json!({
                "value": "test!"
            }))
        })
        .boxed()
}

pub fn exam_list() -> BoxedFilter<(JsonTemplate,)> {
    warp::path!("papers")
        .and(warp::path::end())
        .map(move || {
            lib::template::Template::new("exam_list", json!(
                get_exam_pdfs().unwrap_or_default()
            ))
        })
        .boxed()
}

pub fn styles() -> BoxedFilter<(warp::fs::File,)> {
    warp::path("styles")
        .and(warp::fs::dir(PathBuf::from("./src/styles")))
        .boxed()
}