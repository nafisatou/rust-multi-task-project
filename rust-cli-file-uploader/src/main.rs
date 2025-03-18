use std::fs::File;
use std::io::Read;
use reqwest::blocking::Client;
use reqwest::blocking::multipart::{Form, Part};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    file_path: String,
}

fn main() {
    let args = Cli::parse();
    let file_path = args.file_path;

    println!(" File to upload: {}", file_path);

    // Read the file
    let mut file = File::open(&file_path).expect("⚠️ Failed to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("⚠️ Failed to read file");

    // Create multipart form
    let form = Form::new()
        .part("file", Part::bytes(buffer).file_name(file_path.clone()));

    let client = Client::new();
    let url = "http://127.0.0.1:3000/upload";  // Update with actual server URL

    println!("Uploading to: {}", url);

    // Send request
    let response = client.post(url)
        .multipart(form)
        .send();

    match response {
        Ok(resp) => println!(" Response: {:?}", resp.text().unwrap()),
        Err(err) => println!("Error: {:?}", err),
    }
}
