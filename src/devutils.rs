use tokio;
pub async fn _read_file(path: &str)->String{
     let message = format!("Could not find the provided path: {path}");
     let json: String = tokio::fs::read_to_string(path).await.expect(&message);
    return json;
}