mod model;
mod state;
mod handler;
mod routes;
mod body;


use std::net::SocketAddr;

use hyper::service::service_fn;
use hyper::server::conn::http1;

use hyper_util::rt::TokioIo;

use tokio::net::TcpListener;


use state::AppState;
use handler::handler;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let addr = SocketAddr::from(([127, 0, 0, 1], 3002));
    let listener = TcpListener::bind(addr).await?;
    let state : AppState = Default::default();

    println!("BooksAPI [type:log] [from:main/server] -- Server running on 127.0.0.1:3002");

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        let state = state.clone();

        tokio::task::spawn(async move {

            let state = state.clone();
            if let Err(e) = http1::Builder::new().serve_connection(io, service_fn(move |req| handler(req, state.clone()))).await {
                eprintln!("Error {}", e);
            }

        });
        
    }

}