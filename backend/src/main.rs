use http_body_util::Full;
use hyper::server::conn::http1;
use hyper::{Request, Response};
use hyper::body::Bytes;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use serde::{Deserialize, Serialize};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::time::{timeout, Duration};

use surrealdb::engine::any::connect;
use surrealdb::opt::auth::Root;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("Server running on 127.0.0.1:8080");

    let listener = TcpListener::bind(addr).await?;

    println!("Serving on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;

        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(serve_static_files))
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}

async fn serve_static_files(req: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    let path = req.uri().path();
    println!("Serving file for path: {}", path);

    if path.starts_with("/api") {
        return api_handler(path).await;
    }

    let file_path = match path {
        "/" => "static/index.html",
        _ => &format!("static{}", path),
    };

    println!("Requesting file: {}", file_path);

    let mut file = match File::open(file_path).await {
        Ok(file) => file,
        //Err(_) => return Ok(not_found()),
        Err(e) => {
            eprintln!("Error OPENING file {}: {:?}", file_path, e);
            return Ok(Response::new(Full::new(Bytes::from("Error one!"))));
        }
    };

    let mut contents = vec![];
    if let Err(e) = file.read_to_end(&mut contents).await {
        //return Ok(not_found());
        eprintln!("Error READING file {}: {:?}", file_path, e);
        return Ok(Response::new(Full::new(Bytes::from("Error two!"))));
    }

    let mime_type = get_mime_type(file_path);
    Ok(Response::builder()
        .header("Content-Type", mime_type)
        .body(Full::from(contents))
        .unwrap())
}

fn get_mime_type(path: &str) -> &'static str {
    if path.ends_with(".wasm") {
        "application/wasm"
    } else if path.ends_with(".html") {
        "text/html"
    } else if path.ends_with(".js") {
        "application/javascript"
    } else if path.ends_with(".css") {
        "text/css"
    } else {
        "text/plain"
    }
}

async fn api_handler(path: &str) -> Result<Response<Full<Bytes>>, Infallible> {
    match path {
        "/api/data" => {
            println!("Connecting to the database...");
            let db = match connect("http://127.0.0.1:8008/rpc").await {
                Ok(db) => db,
                Err(e) => {
                    eprintln!("Error connecting to the database: {:?}", e);
                    return Ok(Response::builder()
                        .status(500)
                        .header("Content-Type", "application/json")
                        .body(Full::new(Bytes::from(r#"{"error": "Failed to connect to database"}"#)))
                        .unwrap());
                }
            };
            
            println!("Logging in to the database...");
            let result = timeout(Duration::from_secs(5), db.signin(Root { username: "root", password: "root" })).await;

            match result {
                Ok(Ok(_)) => {
                    println!("Login successful");
                }
                Ok(Err(e)) => {
                    eprintln!("Error signing into the database: {:?}", e);
                    return Ok(Response::builder()
                        .status(500)
                        .header("Content-Type", "application/json")
                        .body(Full::new(Bytes::from(r#"{"error": "Failed to sign in to database"}"#)))
                        .unwrap());
                }
                Err(_) => {
                    eprintln!("Database login timed out");
                    return Ok(Response::builder()
                        .status(500)
                        .header("Content-Type", "application/json")
                        .body(Full::new(Bytes::from(r#"{"error": "Database login timed out"}"#)))
                        .unwrap());
                }
            }
            
            println!("Selecting namespace and database...");
            if let Err(e) = db.use_ns("wasmfrp").use_db("sdb").await {
                eprintln!("Error selecting namespace and database: {:?}", e);
                return Ok(Response::builder()
                    .status(500)
                    .header("Content-Type", "application/json")
                    .body(Full::new(Bytes::from(r#"{"error": "Failed to select namespace and database"}"#)))
                    .unwrap());
            }

            println!("Fetching data from the database...");
            let users: Vec<User> = match db.select("user").await {
                Ok(users) => users,
                Err(e) => {
                    eprintln!("Error fetching data from the database: {:?}", e);
                    return Ok(Response::builder()
                        .status(500)
                        .header("Content-Type", "application/json")
                        .body(Full::new(Bytes::from(r#"{"error": "Failed to fetch data from database"}"#)))
                        .unwrap());
                }
            };

            println!("Data fetched successfully");
            Ok(Response::builder()
                .header("Content-Type", "application/json")
                .body(Full::new(Bytes::from(serde_json::to_string(&users).unwrap())))
                .unwrap()
            )
        }
        _ => {
            Ok(Response::builder()
                .status(404)
                .body(Full::new(Bytes::from("404 - Not Found")))
                .unwrap())
        }
    }
}


// TODO: Function to return 404 response
//Response::builder().status(404).body(Body::from("404 - Not Found")).unwrap()
// fn not_found() -> Response<hyper::body::Body> {}
