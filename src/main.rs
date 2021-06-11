mod lib;

use std::sync::Arc;
use std::path::PathBuf;

use warp::Filter;
use warp::filters::BoxedFilter;

use handlebars::Handlebars;
use serde_json::json;

use std::error::Error;
use std::collections::HashMap;
use std::fs::read_dir;

#[cached::proc_macro::cached(size = 1, result = true)]
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
            (subject.into_string().unwrap(),
            read_dir(path).unwrap().into_iter().map(|level| {
                let level = level.unwrap();
                
                (
                    level.file_name().into_string().unwrap(),
                    read_dir(level.path()).unwrap().map(|pdf| {
                        pdf.unwrap().path()
                    }).collect::<Vec<_>>()
                )
            }).collect::<Vec<_>>())
        }).collect()
    )
}

// TODO: Maybe come back to this
// struct Page {
//     tpl_path: &'static str, // Path relative to `src/templates/`
//     filter: BoxedFilter<()>
// }

// impl Page {
//     fn new(path: &'static str, filter: BoxedFilter<()>) -> Page {
//         Page {
//             tpl_path: path,
//             filter: filter
//         }
//     }

//     fn build(self: &'static Self, data: Arc<handlebars::JsonValue>) -> BoxedFilter<(template::Template<Arc<handlebars::JsonValue>>,)> {
//         self.filter.clone().map(move || {
//             template::Template::new(self.tpl_path, Arc::clone(&data))
//         }).boxed()
//     }
// }

fn rendered_templates() -> Result<BoxedFilter<(impl warp::Reply,)>, Box<dyn std::error::Error>> {
    let mut handlebars = Handlebars::new();

    handlebars.set_dev_mode(true);
    handlebars.register_templates_directory(".hbs", "./src/templates/")?;

    lib::register_helpers(&mut handlebars);

    let handlebars = Arc::new(handlebars);

    let render_filter = move |template| {
        lib::template::reply_with_template(template, Arc::clone(&handlebars))
    };

    return Ok(
        lib::filters::index().map(render_filter.clone()).or(
            lib::filters::exam_list().map(render_filter)
        ).boxed()
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // lib::scraper::scrape_exams("./assets/papers").await?;
    
    // TODO: Maybe come back to this
    // let pages = vec![
    //     Page::new(
    //         "index",
    //         warp::path::end().boxed()
    //     ),
    //     Page::new(
    //         "exam_list",
    //         warp::path("papers").boxed()
    //     )
    // ];

    let templates = rendered_templates()?;

    let routes = templates
        .or(lib::filters::styles());

    warp::serve(routes)
        .run(([127, 0, 0, 1], 8888))
        .await;
    
    Ok(())
}