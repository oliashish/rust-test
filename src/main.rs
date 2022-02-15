use clap::Parser;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use hyper::{Method, StatusCode};
use std::fs;

use std::{convert::Infallible, net::SocketAddr};

async fn file(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(Body::from("hi"))),
        (&Method::GET, "/file.txt") => {
            let args = Args::parse();

            let file =
                fs::read_to_string(args.name).expect("something went wrong reading the file");
            Ok(Response::new(Body::from(file)))
        }

        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[derive(Parser, Debug)]
struct Args {
    name: String,
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let file_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(file)) });

    let server = Server::bind(&addr).serve(file_svc);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
