pub fn extract_token(payload: String) -> String {
    match payload.contains("token") {
        true => {
            let chunks: Vec<&str> = payload.split("\"").collect();
            return chunks[7].to_string();
        }
        false => String::new(),
    }
}
