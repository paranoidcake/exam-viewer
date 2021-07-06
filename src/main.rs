mod lib;

use std::sync::Arc;

use warp::Filter;

use handlebars::Handlebars;

fn register_new_handlebars() -> Result<Handlebars<'static>, Box<dyn std::error::Error>> {
    let mut handlebars = Handlebars::new();

    handlebars.set_dev_mode(true);
    handlebars.register_templates_directory(".hbs", "./src/templates/")?;

    lib::register_helpers(&mut handlebars);

    Ok(handlebars)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // lib::scraper::scrape_exams("./assets/papers").await?;

    let handlebars = Arc::new(register_new_handlebars()?);
    let render_filter = move |template| {
        lib::template::reply_with_template(template, Arc::clone(&handlebars))
    };

    // Serving handlebars templates
    let index = lib::filters::index().map(render_filter.clone());
    let exam_list = lib::filters::exam_list().map(render_filter.clone());
    let exam_subject = lib::filters::exam_subject().map(render_filter);

    let routes = index
        .or(exam_list)
        .or(exam_subject)
        .or(lib::filters::styles())
        .or(lib::filters::assets())
        .or(lib::filters::scripts());

    warp::serve(routes)
        .run(([127, 0, 0, 1], 8888))
        .await;

    Ok(())
}