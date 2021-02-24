pub fn extract_token(payload: String) -> String {
    match payload.contains("token") {
        true => {
            let chunks: Vec<&str> = payload.split("\"").collect();
            return chunks[7].to_string();
        }
        false => String::new(),
    }
}

pub fn derive_stream_type(payload: String) -> String {
    let contains_substring =
        |vector: Vec<&str>| -> bool { vector.iter().any(|&event| payload.contains(event)) };
    match contains_substring(vec!["ping", "trade", "book", "ticker", "spread", "ohlc"]) {
        true => "public".to_string(),
        false => match contains_substring(vec![
            "ownTrades",
            "openOrders",
            "addOrder",
            "cancelOrder",
            "cancelAll",
            "cancelAllOrdersAfter",
        ]) {
            true => "private".to_string(),
            false => "invalid".to_string(),
        },
    }
}
