use std::env;
use std::fmt::format;
use std::fs::read;
use std::io;
use reqwest::Error;

async fn get_request(address: &str, port: &str, path: String) -> Result<(), Error> {
    let response = reqwest::get(format!("https://{address}:{port}/{path}")).await?;
    println!("Status: {}", response.status());

    let body = response.text().await?;
    println!("Body:\n{}", body);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    //let address = &args[0];
    //let port = &args[1];
    let address = "127.0.0.1";
    let port = "7878";
    loop {
        let mut path = String::new();
        println!("Enter path: ");
        io::stdin().read_line(&mut path);

        get_request(address, port, path).await?;
    }

    Ok(())
}

