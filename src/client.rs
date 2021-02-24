use std::thread;
use tungstenite::{connect, Message};

pub async fn initialize_websockets_stream(callback: fn(&str), payload: String) {
    thread::spawn(move || {
        let (mut socket, _) = connect(crate::config::get_kraken_websockets_api_url(
            crate::utils::derive_stream_type(payload.to_owned()),
        ))
        .expect(r#"{{"error":"unable to connect"}}"#);
        socket.write_message(Message::Text(payload)).unwrap();
        loop {
            match socket.read_message() {
                Ok(response) => callback(response.to_text().unwrap()),
                Err(error) => panic!(error),
            };
        }
    })
    .join()
    .unwrap();
}
