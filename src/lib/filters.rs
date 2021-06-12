use std::path::PathBuf;

use serde_json::json;

use warp::Filter;
use warp::filters::BoxedFilter;

type JsonTemplate = super::template::Template<handlebars::JsonValue>;

pub fn index() -> BoxedFilter<(JsonTemplate,)> {
    warp::path::end()
        .map(move || {
            super::template::Template::new("index", json!({
                "value": "test!"
            }))
        })
        .boxed()
}

pub fn exam_list() -> BoxedFilter<(JsonTemplate,)> {

    use std::error::Error;
    use std::collections::HashMap;
    use std::fs::read_dir;

    #[cached::proc_macro::cached(size = 1, result = true, time = 5)]
    fn get_exam_pdfs() -> Result<HashMap<String, Vec<(String, Vec<PathBuf>)>>, Box<dyn Error>> {
        let root_path = PathBuf::from("./assets/papers");
        // let mut hash_map: HashMap<String, Vec<(String, Vec<PathBuf>)>> = HashMap::new();

        return Ok(
            read_dir(root_path)?.into_iter()
            .map(|entry| {
                let entry = entry.unwrap();
                (entry.path(), entry.file_name())
            })
            .map(|(path, subject)| {
                let mut vec = read_dir(path).unwrap().into_iter().map(|level| {
                    let level = level.unwrap();

                    let mut years = read_dir(level.path()).unwrap().map(|pdf| {
                        pdf.unwrap().path()
                    }).collect::<Vec<_>>();

                    years.sort_unstable();
                    years.reverse();

                    (level.file_name().into_string().unwrap(), years)
                }).collect::<Vec<_>>();

                vec.sort_unstable();

                (subject.into_string().unwrap(), vec)
            }).collect()
        )
    }

    warp::path!("papers")
        .and(warp::path::end())
        .map(move || {
            super::template::Template::new("exam/subject_list", json!(
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