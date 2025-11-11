use std::net::TcpListener;

use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let address = "127.0.0.1:0";
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
