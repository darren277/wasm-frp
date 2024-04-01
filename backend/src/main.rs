use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // Purely process request and generate response
    let response = match (req.method(), req.uri().path()) {
        (&hyper::Method::GET, "/") => Response::new(Body::from("Hello, World!")),
        _ => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Not Found"))
            .unwrap(),
    };

    Ok(response)
}

#[tokio::main]
async fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle_request))
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
