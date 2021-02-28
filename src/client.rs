use async_tungstenite::async_std::connect_async;
use async_tungstenite::tungstenite::protocol::Message;
use futures::{future, pin_mut, StreamExt};

pub async fn spawn_websockets_async_stream(
    callback: fn(&str),
    stream_type: String,
    receiver: futures_channel::mpsc::UnboundedReceiver<Message>,
) {
    let (socket, _) = connect_async(crate::config::get_kraken_websockets_api_url(String::from(
        stream_type,
    )))
    .await
    .expect(r#"{{"error":"unable to connect"}}"#);
    let (write, read) = socket.split();
    let rx_to_ws = receiver.map(Ok).forward(write);
    let ws_to_cb = read.for_each(|message| async {
        let data = message.unwrap().to_string();
        callback(data.as_str());
    });
    pin_mut!(rx_to_ws, ws_to_cb);
    future::select(rx_to_ws, ws_to_cb).await;
}