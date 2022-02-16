use clap::Parser;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use hyper::{Method, StatusCode};
use std::fs as ft;
use std::net::SocketAddr;
use std::path::Path;
use tokio::fs;

async fn file(req: Request<Body>) -> Result<Response<Body>, std::io::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            let args = Args::parse();
            let mut _path = args.name.as_str();
            let con_type = match ft::metadata(&args.name) {
                Ok(file) => file,
                Err(_) => {
                    return Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::from("File not found"))
                        .unwrap());
                }
            };
            let mut _filename = "";
            if con_type.is_file() {
                let file = Path::new(&args.name).file_name().unwrap();
                _filename = file.to_str().unwrap();
            }

            let mut file = Vec::new();
            let mut contents = Vec::new();
            if con_type.is_dir() {
                let mut directory = fs::read_dir(&args.name).await?;
                while let Some(dir) = directory.next_entry().await? {
                    contents.push(dir.file_name())
                }
                // return directory contents of message saying this is a directory
            } else {
                file = fs::read(_path).await?;
            }
            Ok(Response::new(Body::from(file)))
        }

        (&Method::GET, _filename) => {
            let args = Args::parse();
            let mut _path = args.name.as_str();
            let mut base_path = Path::new(_path);
            if _path.contains(".") {
                base_path = base_path.parent().unwrap();
            }
            let mut _filename = _filename;
            let mut abs_path = format!("{}{}", base_path.to_str().unwrap(), _filename);

            let con_type = match ft::metadata(&abs_path) {
                Ok(file) => file,
                Err(_) => {
                    return Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::from("File not found"))
                        .unwrap());
                }
            };

            abs_path = format!("{}{}", base_path.to_str().unwrap(), _filename);

            let mut file = Vec::new();

            let mut contents = Vec::new();
            if con_type.is_dir() {
                let mut directory = fs::read_dir(&abs_path).await?;
                while let Some(dir) = directory.next_entry().await? {
                    contents.push(dir.file_name())
                }
                // return directory contents of message saying this is a directory
            } else {
                file = fs::read(&abs_path).await?;
            }

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

    let file_svc = make_service_fn(|_conn| async { Ok::<_, std::io::Error>(service_fn(file)) });

    let server = Server::bind(&addr).serve(file_svc);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
