use http_body_util::Full;
use hyper::server::conn::http1;
use hyper::body::Bytes;
use hyper::{Body, Method, Request, Response};
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use serde::{Deserialize, Serialize};
use surrealdb::{Surreal, engine::any::Any};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::time::{timeout, Duration};

use surrealdb::engine::any::connect;
use surrealdb::opt::auth::Root;

const SURREALDB_DOCKER_HOST: &str = "wasmfrp-surrealdb";
const SURREALDB_INTERNAL_DOCKER_PORT: u16 = 8000;

const SURREALDB_NS: &str = "test";
const SURREALDB_DB: &str = "test";
const SURREALDB_USER: &str = "root";
const SURREALDB_PASS: &str = "root";

const PORT: u16 = 8080;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], PORT));
    println!("Server running on http://{}", addr);

    let listener = TcpListener::bind(addr).await?;

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
        return api_handler(req).await;
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

async fn db_connect() -> Result<Surreal<Any>, Response<Full<Bytes>>> {
    println!("Connecting to the database...");
    
    let db: Surreal<Any> = match connect(format!(
        "http://{}:{}/rpc", 
        SURREALDB_DOCKER_HOST, 
        SURREALDB_INTERNAL_DOCKER_PORT
    ))
    .await
    {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Error connecting to the database: {:?}", e);
            return Err(
                Response::builder()
                    .status(500)
                    .header("Content-Type", "application/json")
                    .body(Full::from(r#"{"error": "Failed to connect to database"}"#))
                    .unwrap()
            );
        }
    };
    
    println!("Logging in to the database...");
    let result = timeout(
        Duration::from_secs(5),
        db.signin(Root {
            username: SURREALDB_USER,
            password: SURREALDB_PASS
        })
    )
    .await;

    match result {
        Ok(Ok(_)) => {
            println!("Login successful");
        }
        Ok(Err(e)) => {
            eprintln!("Error signing into the database: {:?}", e);
            return Err(
                Response::builder()
                    .status(500)
                    .header("Content-Type", "application/json")
                    .body(Full::from(r#"{"error": "Failed to sign in to database"}"#))
                    .unwrap()
            );
        }
        Err(_) => {
            eprintln!("Database login timed out");
            return Err(
                Response::builder()
                    .status(500)
                    .header("Content-Type", "application/json")
                    .body(Full::from(r#"{"error": "Database login timed out"}"#))
                    .unwrap()
            );
        }
    }
    
    println!("Selecting namespace and database...");
    if let Err(e) = db.use_ns(SURREALDB_NS).use_db(SURREALDB_DB).await {
        eprintln!("Error selecting namespace and database: {:?}", e);
        return Err(
            Response::builder()
                .status(500)
                .header("Content-Type", "application/json")
                .body(Full::from(r#"{"error": "Failed to select namespace and database"}"#))
                .unwrap()
        );
    }
    
    Ok(db)
}

pub async fn api_handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let path = req.uri().path().to_string();
    let method = req.method().clone();

    let db = match db_connect().await {
        Ok(db) => db,
        Err(error_response) => {
            // If db_connect returned Err(...), 
            // that Err(...) is an HTTP response we want to send back
            return Ok(error_response);
        }
    };
    
    match (method, path.as_str()) {
        (Method::GET, "/api/users") => {
            println!("Fetching data from the database...");
            let users: Vec<User> = match db.select("users").await {
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
        (Method::POST, "/api/users") => {
            /*
            let user: User = db.insert("users").content(Data {name: "Joe", email: "joe@gmail.com"}).await?;
            dbg!(people);
            */

            let body = hyper::body::to_bytes(req.into_body()).await.unwrap();
            let user: User = match serde_json::from_slice(&body) {
                Ok(user) => user,
                Err(e) => {
                    eprintln!("Error deserializing user data: {:?}", e);
                    return Ok(Response::builder()
                        .status(400)
                        .header("Content-Type", "application/json")
                        .body(Full::new(Bytes::from(r#"{"error": "Failed to deserialize user data"}"#)))
                        .unwrap());
                }
            };

            println!("Inserting data into the database...");
            match db.insert("users").content(user).await {
                Ok(_) => {
                    println!("Data inserted successfully");
                    Ok(Response::builder()
                        .status(200)
                        .header("Content-Type", "application/json")
                        .body(Full::new(Bytes::from(r#"{"message": "User inserted successfully"}"#)))
                        .unwrap()
                    )
                }
                Err(e) => {
                    eprintln!("Error inserting data into the database: {:?}", e);
                    Ok(Response::builder()
                        .status(500)
                        .header("Content-Type", "application/json")
                        .body(Full::new(Bytes::from(r#"{"error": "Failed to insert data into database"}"#)))
                        .unwrap()
                    )
                }
            }
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
