
use std::convert::Infallible;

use crate::AppState;
use crate::body::{
    build_response,
    build_response_from_string
};
use crate::model::Book;

use hyper::body::{Bytes, Incoming};
use hyper::{Request, Response};
use http_body_util::{BodyExt, Full};


pub async fn get_all_books(state: AppState) -> Result<Response<Full<Bytes>>, Infallible> {

    let books = state.read().await;
    let json_string = serde_json::to_string(&*books).unwrap();
    let bytes_json = Bytes::from(json_string);

    let res = build_response(bytes_json).await.unwrap();

    Ok(res)

}

pub async fn get_a_book(state: AppState, id: u64) -> Result<Response<Full<Bytes>>, Infallible> {

    let books = state.read().await;

    for book in books.iter() {
        if book.id == id {
            let found = book.clone();

            let json_string = match serde_json::to_string(&found) {
                Ok(json) => json,
                Err(_) => return Ok(Response::builder()
                    .status(500)
                    .body(Full::new(Bytes::from("Error to generate JSON")))
                    .unwrap()),
            }; 

            let byte_slice = Bytes::from(json_string);

            let mut res = build_response(byte_slice).await.unwrap();
            res.headers_mut().insert(hyper::header::CONTENT_TYPE, "application/json".parse().unwrap());
            
            return Ok(res);
        }
    }

    let res_json = serde_json::json!({
        "status": "book_not_found",
        "message": "The book not found"
    });

    let mut res = build_response_from_string(res_json.to_string()).await.unwrap();
    res.headers_mut().insert(hyper::header::CONTENT_TYPE, "application/json".parse().unwrap());
    return Ok(res);

}

pub async fn add_book(state: AppState, req: Request<Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {

    let collected = req.into_body().collect().await.unwrap();
    let body_bytes = collected.to_bytes();
    let new_book: Book = serde_json::from_slice(&body_bytes).unwrap();

    let mut books = state.write().await;
    books.push(new_book.clone());

    let res_json = serde_json::json!({
        "status": "created",
        "received": new_book
    });

    let mut res = build_response_from_string(res_json.to_string()).await.unwrap();
    res.headers_mut().insert(hyper::header::CONTENT_TYPE, "application/json".parse().unwrap());
    
    Ok(res)

}

pub async fn remove_book(state: AppState, id: u64) -> Result<Response<Full<Bytes>>, Infallible> {

    let mut books = state.write().await;

    for (idx, book) in books.iter().enumerate() {

        if book.id == id {
            let deleted = books[idx].clone();

            books.remove(idx);

            let res_json = serde_json::json!({
                "status": "removed",
                "was": deleted
            });

            let mut res = build_response_from_string(res_json.to_string()).await.unwrap();
            res.headers_mut().insert(hyper::header::CONTENT_TYPE, "application/json".parse().unwrap());
            return Ok(res);
        }

    }

    let res_json = serde_json::json!({
        "status": "book_not_found",
        "message": "The book not found"
    });

    let mut res = build_response_from_string(res_json.to_string()).await.unwrap();
    res.headers_mut().insert(hyper::header::CONTENT_TYPE, "application/json".parse().unwrap());
    return Ok(res);

}