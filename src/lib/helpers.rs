use handlebars::{Handlebars, RenderContext, Helper, Context, JsonRender, HelperResult, Output, RenderError};
use std::path::PathBuf;

pub fn register_helpers(hb: &mut Handlebars) {
    hb.register_helper("file-stem", Box::new(file_stem));
}

fn file_stem(helper: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
    let param = helper.param(0).ok_or(RenderError::new("file-stem: Needs 1 parameter"))?
        .value().as_str().ok_or(RenderError::new("file-stem: Parameter must be a string"))?;

    let path = PathBuf::from(param);
    let stem = path.file_stem();

    match stem {
        Some(result) => {
            out.write(result.to_str().unwrap())?;
            Ok(())
        },
        None => {
            Err(RenderError::new("file-stem: Invalid path"))
        }
    }
}