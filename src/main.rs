mod scraper;

use std::sync::Arc;
use std::path::PathBuf;

use warp::Filter;
use handlebars::Handlebars;

use serde_json::json;

mod renderer {
    use std::sync::Arc;
    use handlebars::Handlebars;
    use serde::Serialize;

    pub struct Template<T: Serialize> {
        name: &'static str,
        value: T
    }

    impl<T: Serialize> Template<T> {
        pub fn new(name: &'static str, value: T) -> Template<T> {
            Template {
                name: name,
                value: value 
            }
        }
    }
    
    pub fn reply_with_template<T: Serialize>(template: Template<T>, hb: Arc<Handlebars>) -> impl warp::Reply {
        let render = hb.render(template.name, &template.value)
            .unwrap_or_else(|err| err.to_string());
        Ok(warp::reply::html(render))
    }
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // scraper::scrape_exams("./assets/papers").await?;

    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("index", "./src/templates/index.hbs")?;

    let data = json!({
        "value": "test!"
    });

    let handlebars = Arc::new(handlebars);
    let render_filter = move |template| renderer::reply_with_template(template, Arc::clone(&handlebars));

    let templates = warp::path::end()
        .map(move || {
            renderer::Template::new("index", data.clone())
        })
        .map(render_filter);

    let styles = warp::path("styles")
        .and(warp::fs::dir(PathBuf::from("./src/styles")));
    
    let routes = styles
        .or(templates);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 8888))
        .await;
    
    Ok(())
}