use tokio::time::Duration;

use select::document::Document;
use select::predicate::{Name, Attr, Class};

const FIREFOX_USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:88.0) Gecko/20100101 Firefox/88.0";
const RETRY_DELAY_MS: u64 = 2000;
const REQUEST_DELAY_MS: u64 = 1000;

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

    std::fs::create_dir_all(output_path)?;
    let urls = subject_url_tuples(&document)?;

    println!("Making initial requests...");
    let responses = fetch_urls(urls, &client).await?;
    println!("Initial requests done");

    // TODO: Have this recover itself if execution is stopped early
    println!("Writing files, this may take a long time...");
    let total = responses.len();
    for (index, response) in responses.into_iter().enumerate() {
        match response {
            Ok(res) => {
                let res = res.unwrap();
                let subject = res.0;
                let document = Document::from(&*res.1);

                println!("Writing files ({}): {:.2}%", subject, (index as f32 / total as f32) * 100 as f32);

                if subject == "Chemistry" { // TODO: Remove this
                    write_pdfs_from_document(&client, &document, &subject, &output_path).await?;
                }
            },
            Err(e) => eprintln!("Got an error while writing files: {}", e)
        }
    }
    println!("Done writing files");

    Ok(())
}

async fn fetch_urls(urls: Vec<(String, String)>, client: &reqwest::Client) -> Result<Vec<Result<Result<(String, String), reqwest::Error>, tokio::task::JoinError>>, Box<tokio::task::JoinError>> {
    let futures = futures::future::join_all(urls.into_iter().map(|(subject, url)| {
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
    
    Ok(futures)
}

fn subject_url_tuples(document: &Document) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let mut tuples = Vec::<(String, String)>::new();

    for node in document.find(Class("entry-content")) {
        for list_item in node.find(Name("li")) {
            for link in list_item.find(Name("a")) {
                let subject = link.first_child().unwrap().as_text().unwrap().trim().to_string();
                let page_url = link.attr("href").unwrap().to_string();

                tuples.push((subject, page_url));
            }
        }
    }

    Ok(tuples)
}

async fn write_pdfs_from_document(client: &reqwest::Client, document: &Document, subject: &String, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    for column in document.find(Attr("class", "wp-block-column")) {
        let title = column.find(Name("strong")).next().unwrap().inner_html();
        for link in column.find(Name("a")) {
            let year = link.inner_html();

            async fn retry_delay() {
                println!("Request failed, retrying...");
                tokio::time::sleep(Duration::from_millis(RETRY_DELAY_MS)).await;
            }

            let res: reqwest::Response = loop { // ! There were some errors here with IncompleteMessage that I have yet to test if this fixes
                let res = client.get(link.attr("href").unwrap()).send().await;

                match res {
                    Ok(ok_res) => {
                        if ok_res.status() == 200 {
                            break ok_res;
                        } else {
                            retry_delay().await;
                        }
                    }
                    _ => {
                        retry_delay().await;
                    }
                }
            };

            let output_path = output_path.to_string();
            let subject = subject.to_string();
            let title = title.to_string();

            let path = format!("{}/{}/{}", &output_path, &subject, &title);
            std::fs::create_dir_all(&path).unwrap();

            let pdf = res.bytes().await?;

            let _: tokio::task::JoinHandle<tokio::io::Result<_>> = tokio::spawn(async move {
                tokio::fs::write(format!("./{}/{}/{}/{}.pdf", &output_path, &subject, &title, &year), &pdf).await
            });

            tokio::time::sleep(Duration::from_millis(REQUEST_DELAY_MS)).await;
        }
    }
    Ok(())
}