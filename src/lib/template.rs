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