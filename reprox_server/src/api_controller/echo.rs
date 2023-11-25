use hyper::{Body, Request, Response};

pub fn echo_controller(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let response = Response::new(Body::from("Hello, mahmut"));

    Ok(response)
}
