pub async fn get_difficulties() -> Result<String, reqwest::Error> {
    let body = reqwest::get("https://kenkoooo.com/atcoder/resources/problem-models.json")
        .await?
        .text()
        .await?;
    Ok(body)
}
