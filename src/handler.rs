
use std::convert::Infallible;

use hyper::{Method, Request, Response, StatusCode};
use hyper::body::{Bytes, Incoming};
use http_body_util::{Full};

use crate::state::AppState;

use crate::routes::{
    get_all_books,
    add_book,
    remove_book,
    get_a_book
};

use crate::body::get_id_from_path;


pub async fn handler(req: Request<Incoming>, state: AppState) -> Result<Response<Full<Bytes>>, Infallible>  {

    let method = req.method();
    let path = req.uri().path().to_string();

    println!("BooksAPI [type:log] [from:handler] -- Client has call {} {}", method, path);

    match (method, path.as_str()) {
        
        // GET / -> Get all books
        (&Method::GET, "/") => get_all_books(state).await,
        (&Method::POST, "/") => add_book(state, req).await,

        _ => {


            if method == Method::DELETE {
                if let Some(id) = get_id_from_path(path.clone()).await {
                    return remove_book(state, id).await;
                }
            }

            if method == Method::GET {
                if let Some(id) = get_id_from_path(path.clone()).await {
                    return get_a_book(state, id).await;
                }
            }


            // IF THE METHOD AND PATH NOT FOUND.
            let mut nf = Response::new(Full::new(Bytes::from("BooksAPI [type:log] [from:match] - Not found")));
            *nf.status_mut() = StatusCode::NOT_FOUND;
            return Ok(nf);
        }

    }

}