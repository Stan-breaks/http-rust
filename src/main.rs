use reqwest::{self, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "http://www.rustinaction.com/";
    let response = reqwest::get(url).await?.text().await.unwrap();
    println!("{}", response);
    Ok(())
}
