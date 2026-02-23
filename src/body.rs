use std::convert::Infallible;

use hyper::body::Bytes;

use hyper::{Response};
use http_body_util::{Full};


pub async fn get_id_from_path(path: String) -> Option<u64> {

    if let Some(id_string) = path.strip_prefix("/") {
        if let Ok(id) = id_string.parse::<u64>() {
            Some(id)
        } else { None }
    } else { None }

}


pub async fn build_response(bytes: Bytes) -> Result<Response<Full<Bytes>>, Infallible> {

    let res = Response::new(Full::new(bytes));
    return Ok(res);

}

pub async fn build_response_from_string(string: String) -> Result<Response<Full<Bytes>>, Infallible> {

    let res = Response::new(Full::new(Bytes::from(string)));
    return Ok(res);

}