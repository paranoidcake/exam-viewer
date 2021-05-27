use select::document::Document;
use select::predicate::{Name, Attr};

use hyper::{Client, Uri};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    scrape_exams("https://theleavingcert.com/exam-papers/chemistry/").await?;
    
    Ok(())
}

async fn scrape_exams(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let uri = url.parse::<Uri>().unwrap();
    let https = hyper_tls::HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    // let client = Client::new();

    let res = client.get(uri).await?;
    println!("Status: {:?}", res.status());
    println!("Headers: {:?}", res.headers());

    let body = String::from_utf8(
        hyper::body::to_bytes(res.into_body())
            .await?
            .to_vec()
    ).unwrap();

    println!("{}", body);

    let document = Document::from(&*body);

    for node in document.nth(2) {// .unwrap().first_child() {
        println!("{:?}", node);
    }

    Ok(())
}
