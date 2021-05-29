use std::env;
use std::str;
use tokio;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_proxy;

#[tokio::main]
async fn main() {
    let mut s = match env::var("all_proxy") {
        Ok(proxy) => tokio_proxy::connect("httpbin.org:80", proxy).await.unwrap(),
        Err(_) => TcpStream::connect("httpbin.org:80").await.unwrap(),
    };

    s.write_all(
        b"GET /get HTTP/1.1\r\n\
        Host: httpbin.org\r\n\r\n",
    )
    .await
    .unwrap();

    let mut buf = [0; 512];
    s.read(&mut buf).await.unwrap();

    for line in str::from_utf8(&buf).unwrap().lines() {
        println!("{}", line);
    }
}
