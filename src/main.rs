use anyhow::Result;
use wstd::http::body::Body;
use wstd::http::{Request, Response, StatusCode};

#[wstd::http_server]
async fn main(request: Request<Body>) -> Result<Response<Body>> {
    let path = request.uri().path();

    match path {
        "/" => Ok(Response::new(
            "Hello from Cargo HTTP Server! 🦀\n".to_owned().into(),
        )),
        "/health" => Ok(Response::new("OK\n".to_owned().into())),
        "/echo" => {
            // Echo back the request method and path
            let msg = format!("Method: {}\nPath: {}\n", request.method(), path);
            Ok(Response::new(msg.into()))
        }
        _ => {
            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap();
            Ok(response)
        }
    }
}
