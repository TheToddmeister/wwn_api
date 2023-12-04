use tokio;

pub async fn _read_file(path: &str) -> Result<String, tokio::io::Error> {
    let json: String = tokio::fs::read_to_string(path).await?;
    Ok(json)
}