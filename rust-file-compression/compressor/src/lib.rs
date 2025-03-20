use std::fs::File;
use std::io::{Read, Write};
use flate2::write::GzEncoder;
use flate2::Compression;

fn compress_file(input_path: &str, output_path: &str) -> std::io::Result<()> {
    let mut input_file = File::open(input_path)?;
    let mut output_file = File::create(output_path)?;
    let mut encoder = GzEncoder::new(output_file, Compression::default());

    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;
    encoder.write_all(&buffer)?;
    
    Ok(())
}

fn main() {
    let input_file = "example.txt";
    let output_file = "example.txt.gz";

    match compress_file(input_file, output_file) {
        Ok(_) => println!("File compressed successfully!"),
        Err(e) => eprintln!("Error: {}", e),
    }
}




