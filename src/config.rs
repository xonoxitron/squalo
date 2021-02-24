use url::Url;

pub fn get_kraken_websockets_api_url(stream_type: String) -> Url {
    Url::parse(
        format!(
            "wss://ws{}.kraken.com",
            if stream_type == "private" {
                "-auth"
            } else {
                ""
            }
        )
        .as_str(),
    )
    .unwrap()
}
