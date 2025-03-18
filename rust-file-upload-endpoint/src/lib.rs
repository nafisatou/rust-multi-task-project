use axum::extract::Multipart;
use axum::http::StatusCode;
use std::fs::{create_dir_all, File};
use std::io::Write;

pub async fn upload_file(mut multipart: Multipart) -> Result<String, StatusCode> {
    if create_dir_all("./uploads").is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap_or("unknown").to_string();
        let data = field.bytes().await.unwrap();

        let file_path = format!("./uploads/{}", file_name);
        let mut file = File::create(&file_path).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        file.write_all(&data)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        return Ok(format!("File '{}' uploaded successfully!", file_name));
    }

    Err(StatusCode::BAD_REQUEST)
}
