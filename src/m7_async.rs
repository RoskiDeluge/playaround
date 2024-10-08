async fn my_async_call(url: &str) -> Result<serde_json::Value, reqwest::Error> {
    let response: serde_json::Value = reqwest::get(url).await?.json::<serde_json::Value>().await?;
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_calls_async_fn() {
        let api_url = "http://numbersapi.com/random/year?json";
        let my_res = my_async_call(api_url).await;
        match my_res {
            Ok(r) => {
                dbg!(r);
            }
            Err(_) => {
                panic!("Failed to make request");
            }
        };
    }
}
