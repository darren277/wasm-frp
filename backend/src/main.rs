use http_body_util::Full;
use hyper::server::conn::http1;
use hyper::{Request, Response};
use hyper::body::Bytes;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use std::convert::Infallible;
use std::net::SocketAddr;

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

    let file_path = match path {
        "/" => "static/index.html",
        _ => &format!("static{}", path),
    };

    println!("Requesting file: {}", file_path);

    let mut file = match File::open(file_path).await {
        Ok(file) => file,
        //Err(_) => return Ok(not_found()),
        Err(_) => return Ok(Response::new(Full::new(Bytes::from("Error one!")))),
    };

    let mut contents = vec![];
    if let Err(_) = file.read_to_end(&mut contents).await {
        //return Ok(not_found());
        return Ok(Response::new(Full::new(Bytes::from("Error two!"))));
    }

    //Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))

    let mime_type = get_mime_type(file_path);
    Ok(Response::builder()
        .header("Content-Type", mime_type)
        .body(Full::from(contents))
        .unwrap())

    //Ok(Response::new(Full::new(Bytes::from(contents))))
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

// TODO: Function to return 404 response
//Response::builder().status(404).body(Body::from("404 - Not Found")).unwrap()
// fn not_found() -> Response<hyper::body::Body> {}
