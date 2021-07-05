use std::path::PathBuf;

use std::error::Error;
use std::collections::HashMap;
use std::fs::read_dir;

#[cached::proc_macro::cached(size = 1, result = true)]
pub fn get_exam_pdfs() -> Result<HashMap<String, Vec<(String, Vec<PathBuf>)>>, Box<dyn Error>> {
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