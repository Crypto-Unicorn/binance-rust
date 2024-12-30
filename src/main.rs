use http_body_util::BodyExt;
use http_body_util::Empty;
use hyper::body::Bytes;
use hyper::client::conn::http1;
use hyper::Request;
use hyper_util::rt::TokioIo;
use tokio::io::{self, AsyncWriteExt as _};
use tokio::{net::TcpStream, task};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = "https://api.binance.com/api/v3/ping".parse::<hyper::Uri>()?;

    let host = url.host().expect("uri has no host");
    let port = url.port_u16().unwrap_or(80);
    let address = format!("{}:{}", host, port);

    let body = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;

    Ok(())
}
