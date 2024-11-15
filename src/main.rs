use hyper::{Body, Request, Response, Server, Method, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[derive(Serialize, Deserialize)]
struct Message {
    message: String,
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        // Handle GET request to /hello
        (&Method::GET, "/hello") => {
            let response = json!({
                "message": "Hello, welcome to our server!"
            });
            Ok(Response::new(Body::from(response.to_string())))
        }

        // Handle POST request to /echo
        (&Method::POST, "/echo") => {
            let whole_body = hyper::body::to_bytes(req.into_body()).await.unwrap();
            let received: Message = serde_json::from_slice(&whole_body).unwrap_or(Message {
                message: "Invalid JSON".to_string(),
            });
            let response = json!({
                "echoed_message": received.message
            });
            Ok(Response::new(Body::from(response.to_string())))
        }

        // Handle GET request to /static/file.txt
        (&Method::GET, "/static/file.txt") => {
            match serve_static_file("static/file.txt").await {
                Ok(contents) => Ok(Response::new(Body::from(contents))),
                Err(_) => {
                    let mut not_found = Response::new(Body::from("File not found"));
                    *not_found.status_mut() = StatusCode::NOT_FOUND;
                    Ok(not_found)
                }
            }
        }

        // Catch-all for unrecognized routes
        _ => {
            let mut not_found = Response::new(Body::from("Route not found"));
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

// Helper function to serve static files
async fn serve_static_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}

#[tokio::main]
async fn main() {
    // Set the address for the server to listen on
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Create a service handler
    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, Infallible>(service_fn(handle_request)) }
    });

    // Start the server
    let server = Server::bind(&addr).serve(make_svc);

    println!("Server running on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}