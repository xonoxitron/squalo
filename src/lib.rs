//! ![squalo](https://raw.githubusercontent.com/xonoxitron/squalo/master/squalo-logo.png)
//! =
//!
//! ### 🦈 Minimal, elegant, fast, async Kraken exchange WebSockets API client | Written in Rust
//!
//! # Description
//!
//! **```squalo```** library aims to interface your software with the [Kraken exchange WebSockets API](https://support.kraken.com/hc/en-us/sections/360003493672-WebSocket-API) in no time.
//!
//! # Prerequisites
//!
//! The [Kraken exchange](https://kraken.com) allows **```WebSockets API```** interaction with both **```public```** and **```private```** data.
//!
//! Only for the **```private```** one, you need to issue an **```API-Key```** and an **```API-Secret```** to the **```squalo```** library, in order to [generate authentication token](https://www.kraken.com/features/api#ws-auth) for signed subscriptions on **```WebSockets```** endpoints.
//!
//! If you are not familiar with, please have a look at [WebSockets API - FAQ](https://support.kraken.com/hc/en-us/articles/360022326871-Kraken-WebSocket-API-Frequently-Asked-Questions#1) for a general overview, or at [WebSockets API](https://docs.kraken.com/websockets/) document.
//!
//! # Implementation
//!
//! Add this to your **```Cargo.toml```**:
//! ```toml
//! [dependencies]
//! squalo = {version = 0.1.2}
//! ```
//!
//! and then add this to your **```code```**:
//! ```rust
//! use squalo;
//! ```
//!
//! # Methods
//!
//! ```rust
//! squalo::print_crate_info();
//! ```
//! **Description**: prints **```crate```** information (name, description, version, author and repository).
//!
//! ```rust
//! squalo::set_kraken_api_credentials(api_key: String, api_secret: String);
//! ```
//!
//! **Description**: sets **```Kraken API```** credentials for **```private```** data access.
//!
//! **Required**:
//!
//! * *api_key*: ```String```
//! * *api_secret*: ```String```
//!
//! ```rust
//! squalo::get_websockets_token().await;
//! ```
//!
//! **Description**: retrieves a **```WebSockets API token```** with a **```signed```** request from a **```Kraken REST API```** endpoint.
//!
//! **Output**: returns a **```WebSockets API token```** in the **```String```** format.
//!
//! ```rust
//! polipo::attach_websockets_stream(callback: fn(&str), stringified_json_message).await;
//! ```
//!
//! **Description**: spawns a **```thread```** which initializes a **```WebSocket client```** and writes the **```stringified_json_message```**.
//!
//! **Required**:
//!
//! * *callback*: ```fn``` (eg: *fn callback(data: &str) { println("data: {}", data); }*)
//! * *stringified_json_message*: ```String``` (eg: *{"event":"subscribe", "subscription":{"name":"trade"}, "pair":["XBT/USD"]}*)
//!
//! **Output**: any incoming message forwared from the **```WebSockets stream```** to the issued **```callback```** comes in stringified **```JSON```** format (parse accordingly with the outcome shape).
//!
//! # Example
//!
//! The example below shows how easy is to implement **```squalo```** from zero knowledge.
//!
//! ```rust
//! use squalo;
//!
//! fn callback(data: &str) {
//!     println!("data: {}", data);
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     
//!     // printing crate information
//!     squalo::print_crate_info();
//!
//!     // enveloping a message pointing on public endpoint
//!     let payload =
//!         r#"{"event":"subscribe", "subscription":{"name":"trade"}, "pair":["XRP/EUR", "ETH/USD"]}"#.to_string();
//!
//!     // attaching websockets to public data stream
//!     squalo::attach_websockets_stream(callback, payload).await;
//!
//!     // issuing credentials enables private data interaction
//!     squalo::set_kraken_api_credentials(
//!         "YOUR_KRAKEN_API_KEY_HERE".to_string(),
//!         "YOUR_KRAKEN_API_SECRET".to_string(),
//!     );
//!
//!     // requesting a websockets token
//!     let token = squalo::get_websockets_token().await;
//!
//!     // enveloping a message pointing on private endpoint
//!     let payload = format!(
//!         r#"{{"event":"subscribe", "subscription":{{"name":"ownTrades", "token":"{}"}}}}"#,
//!         token
//!     );
//!
//!     // attaching websockets to private data stream
//!     squalo::attach_websockets_stream(callback, payload).await;
//! }
//! ```
//!
//! # Disclaimer
//!
//! This software comes without any kind of warranties.
//!
//! I will not be liable for any damages related to the use or the misuse of this software.
//!
//! You are the sole responsible.

mod client;
mod config;
mod info;
mod utils;

use polipo;

pub fn print_crate_info() {
    println!("-- CRATE INFO --\r\n{}", info::get_crate_info());
}

pub fn set_kraken_api_credentials(api_key: String, api_secret: String) {
    polipo::set_kraken_api_credentials(api_key, api_secret);
}

pub async fn get_websockets_token() -> String {
    utils::extract_token(
        polipo::get_kraken_api_response("GetWebSocketsToken".to_string(), "".to_string()).await,
    )
}

pub async fn attach_websockets_stream(callback: fn(&str), stringified_json_message: String) {
    client::initialize_websockets_stream(callback, stringified_json_message).await;
}
