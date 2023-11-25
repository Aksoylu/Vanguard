use hyper::{Body, Request, Response};

pub fn echo_controller(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let response = Response::new(Body::from("Hello, Rust HTTP Server!"));

    Ok(response)
}
