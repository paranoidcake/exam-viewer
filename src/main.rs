use std::fs::File;

use select::document::Document;
use select::predicate::{Name, Attr, Class};

const FIREFOX_USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:88.0) Gecko/20100101 Firefox/88.0";
const RETRY_DELAY_MS: u64 = 2000;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get_paper_from_page(File::open("./examPages/chemistry").unwrap()).await?;
    scrape_exams("https://theleavingcert.com/exam-papers/", "./assets/papers").await?;
    
    Ok(())
}

async fn scrape_exams(url: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let res = client.get(url).header(
        reqwest::header::USER_AGENT, FIREFOX_USER_AGENT
    ).send().await?;

    let res_text: &str = &res.text().await?;

    let document = Document::from(res_text);

    let mut subjects = Vec::<String>::new();
    let mut urls = Vec::<String>::new();

    std::fs::create_dir_all(output_path)?;

    for node in document.find(Class("entry-content")) {
        for list_item in node.find(Name("li")) {
            for link in list_item.find(Name("a")) {
                let subject = link.first_child().unwrap().as_text().unwrap().trim().to_string();
                let page_url = link.attr("href").unwrap().to_string();

                subjects.push(subject);
                urls.push(page_url);
            }
        }
    }

    let total = urls.len();
    println!("Making initial requests...");
    let responses = futures::future::join_all(subjects.into_iter().zip(urls).into_iter().enumerate().map(|(index, (subject, url))| {
        let client = client.clone();
        let body: tokio::task::JoinHandle<Result<_, reqwest::Error>> = tokio::spawn(async move {
            let mut res: reqwest::Response;
            loop {
                res = client.get(&url).header(
                    reqwest::header::USER_AGENT, FIREFOX_USER_AGENT
                ).send().await?;
    
                if res.status() == 200 {
                    break
                } else {
                    println!("Retrying request...");
                    tokio::time::sleep(tokio::time::Duration::from_millis(RETRY_DELAY_MS)).await;
                }
            }
            Ok((subject, res.text().await?))
        });
        body
    })).await;
    println!("Initial requests done");

    let total = responses.len();
    for (index, response) in responses.into_iter().enumerate() {
        match response {
            Ok(res) => {
                println!("Writing files: {:.2}%", (index as f32 / total as f32) * 100 as f32);

                let res = res.unwrap();
                let subject = res.0;
                let document = Document::from(&*res.1);

                for column in document.find(Attr("class", "wp-block-column")) {
                    for link in column.find(Name("a")) {
                        let year = link.inner_html();

                        let mut res: reqwest::Response;
                        loop {
                            res = client.get(link.attr("href").unwrap()).send().await?;

                            if res.status() == 200 {
                                break
                            } else {
                                println!("Retrying request...");
                                tokio::time::sleep(tokio::time::Duration::from_millis(RETRY_DELAY_MS)).await;
                            }
                        }

                        let output_path = output_path.to_string();
                        let subject = subject.to_string();

                        let path = format!("{}/{}", &output_path, &subject);
                        std::fs::create_dir_all(&path).unwrap();

                        let pdf = res.bytes().await?;

                        // println!("Output path: {:?}, subject: {}, year: {}", &output_path, &subject, &year);

                        // let client = client.clone();
                        let body: tokio::task::JoinHandle<tokio::io::Result<_>> = tokio::spawn(async move {
                            tokio::fs::write(format!("./{}/{}/{}.pdf", &output_path, &subject, &year), &pdf).await
                        });
                    }
                }
            },
            Err(e) => eprintln!("Got an error while writing files: {}", e)
        }
    }

    Ok(())
}