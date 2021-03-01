![squalo](./squalo-logo.png)
=

### 🦈 Minimal, elegant, fast, async Kraken exchange WebSockets API client | Written in Rust

# Description

**```squalo```** library aims to interface your software with the [Kraken exchange WebSockets API](https://support.kraken.com/hc/en-us/sections/360003493672-WebSocket-API) in no time.

# Prerequisites

The [Kraken exchange](https://kraken.com) allows **```WebSockets API```** interaction with both **```public```** and **```private```** data.

Only for the **```private```** one, you need to issue an **```API-Key```** and an **```API-Secret```** to the **```squalo```** library, in order to [generate authentication token](https://www.kraken.com/features/api#ws-auth) for signed subscriptions on **```WebSockets```** endpoints.

If you are not familiar with, please have a look at [WebSockets API - FAQ](https://support.kraken.com/hc/en-us/articles/360022326871-Kraken-WebSocket-API-Frequently-Asked-Questions#1) for a general overview, or at [WebSockets API](https://docs.kraken.com/websockets/) document.


# Implementation

Add this to your **```Cargo.toml```**:

```toml
[dependencies]
squalo = {version = 0.1.5}
```

and then add this to your **```code```**:

```rust
use squalo;
```

# Methods

```rust
squalo::print_crate_info();
```

**Description**: prints **```crate```** information (name, description, version, author and repository).

```rust
squalo::set_kraken_api_credentials(api_key: String, api_secret: String);
```

**Description**: sets **```Kraken API```** credentials for **```private```** data access.

**Required**:

* *api_key*: ```String```
* *api_secret*: ```String```

```rust
squalo::get_websockets_token().await;
```

**Description**: retrieves a **```WebSockets API token```** with a **```signed```** request from a **```Kraken REST API```** endpoint.

**Output**: returns a **```WebSockets API token```** in the **```String```** format.

```rust
squalo::create_communication_channel();
```

**Description**: returns a couple of objects, an **```UnboundedSender```** *(tx)* and an **```UnboundedReceiver```** *(rx)* both used for bidirectional interoperation with the **```WebSocket client```**.

```rust
squalo::attach_websockets_stream(callback: fn(&str), stream_type: String, receiver: UnboundedReceiver<Message>);
```

**Description**: spawns a **```thread```** which initializes a **```WebSocket client```** (accordingly with the **```stream_type```**). The **```receiver```** bridges the gap with the incoming data **```stream```** and the **```callback```**.

**Required**:

* *callback*: ```fn``` (eg: *fn callback(data: &str) { println("data: {}", data); }*)
* *stream_type*: ```String``` (eg *"public"* or *"private"*)
* *receiver*: ```UnboundedReceiver<Message>``` (retrieved from the **```squalo::create_communication_channel()```** method)

**Output**: any incoming message forwared from the **```WebSockets stream```** to the issued **```callback```** comes in stringified **```JSON```** format (parse accordingly with the outcome shape).

```rust
squalo::send_message(transmitter: UnboundedSender<Message>, payload: String);
```

**Description**: fowards a **```message```** to the spawned **```thread```** handling the **```WebSocket client```** that will write the **```payload```** to the stream.

# Example

The example below shows how easy is to implement **```squalo```** from zero knowledge.

```rust
use squalo;

fn callback(data: &str) {
    println!("data: {}", data);
}

#[tokio::main]
async fn main() {
    
    // printing crate information
    squalo::print_crate_info();

    // enveloping payload pointing at public endpoint
    let payload1 =
        r#"{"event":"subscribe", "subscription":{"name":"trade"}, "pair":["XRP/EUR", "ETH/USD"]}"#
            .to_string();

    // creating communication channel for the websockets client
    let (tx1, rx1) = squalo::create_communication_channel();

    // attaching websockets to the public data stream
    squalo::attach_websockets_stream(callback, "public".to_string(), rx1);

    // transmitting payload to the websockets client
    squalo::send_message(tx1.to_owned(), payload1);

    // issuing credentials that enables private data interaction
    squalo::set_kraken_api_credentials(
        "YOUR_KRAKEN_API_KEY_HERE".to_string(),
        "YOUR_KRAKEN_API_SECRET".to_string(),
    );

    // requesting a websockets token
    let token = squalo::get_websockets_token().await;

    // enveloping message pointing at a private endpoint
    let payload2 = format!(
        r#"{{"event":"subscribe", "subscription":{{"name":"ownTrades", "token":"{}"}}}}"#,
        token
    );

    // creating communication channel for the websockets client
    let (tx2, rx2) = squalo::create_communication_channel();

    // attaching websockets to private data stream
    squalo::attach_websockets_stream(callback, "private".to_string(), rx2);

    // transmitting payload to the websockets client
    squalo::send_message(tx2.to_owned(), payload2);

    // enveloping the "ping" payload
    let ping = r#"{"event":"ping"}"#.to_string();

    // holding the main thread execution
    loop {
        // sleeping thread for 1 second
        std::thread::sleep(std::time::Duration::from_millis(1000));
        // transmitting ping payload to the first client (public)
        squalo::send_message(tx1.to_owned(), ping.to_owned());
        // transmitting ping payload to the second client (private)
        squalo::send_message(tx2.to_owned(), ping.to_owned());
    }
}
```

# Disclaimer

This software comes without any kind of warranties.

I will not be liable for any damages related to the use or the misuse of this software.

You are the sole responsible.
