use hyper::{Body, Request};

use crate::common::errors::hyper_error::HyperError;

pub fn get_content_length(req: &Request<Body>) -> Result<u64, HyperError> {
    let header_value = match req.headers().get(hyper::header::CONTENT_LENGTH) {
        Some(v) => v,
        None => return Ok(0),
    };

    let content_length_str = header_value.to_str()
        .map_err(|_| HyperError::from("Invalid Content-Length header"))?;

    let content_length = content_length_str.parse::<u64>()
        .map_err(|_| HyperError::from("Failed to parse Content-Length"))?;

    Ok(content_length)
}
