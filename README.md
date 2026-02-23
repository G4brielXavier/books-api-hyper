# BooksAPI - API Simples em Rust com Hyper

*IT JUST FOR MY STUDIES AND LEARNINGS WITH RUST.*

An RESTfull API designed in Rust, using **Hyper**, **Tokio** and **serde_json**.
Simple, fast and perfect for who want learn how works the basic flow of GET, POST, PUT, PATCH and DELETE using sharing state with `Arc<RwLock<>>`.

### Stacks used
- Rust
- Tokio (async runtime)
- Hyper (HTTP server)
- serde / serde_json (JSON serialization)
- Arc + RwLock (safe share state between threads)

### Project's Structure

```
src/
 └── body.rs
 ├── handler.rs
 ├── main.rs
 ├── model.rs
 └── routes.rs
 ├── state.rs
```

- `body.rs` -> 
- `handler.rs` ->
- `model.rs` -> Structs like `book`
- `routes.rs` -> GET, POST, PATCH and DELETE functions
- `state.rs` -> `AppState` using `Arc<RwLock<Vec<Book>>>`

### Book Model 

```rust
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Book {
    pub id: u64,
    pub name: String,
    pub author: String,
    pub pages: u64
}
```


### Routes

| Método | Rota    | Descrição                   |
| ------ | ------- | --------------------------- |
| GET    | `/`     | Return all books            |
| POST   | `/`     | Create a new book           |
| GET    | `/{id}` | Return a specific book      |
| PUT    | `/{id}` | Edit a book                 |
| DELETE | `/{id}` | Remove a book               |

### How to run

1. Install Rust (nightly or stable)
2. Run: 

```bash
cargo run
```
The API rises in: 

```bash
http://127.0.0.1:3002/
```

Response: 

```json
[
    {
        "id": 1,
        "name": "Silmarillion",
        "author": "JRR Tolkien",
        "pages": 500
    }
]
```


### POST /

```code
POST http://127.0.0.1:3000/
Content-Type: application/json

{
    "id": 2,
    "name": "Hyper API from scratch",
    "author": "Client",
    "pages": 0
}
```

### DELETE /2

```code
DELETE http://127.0.0.1:3000/2

{
    "status": "removed",
    "was": {
        "id": 2,
        "name": "Hyper API from scratch",
        "author": "Client",
        "pages": 0
    }
}
```

### Main learnings
- How uses `Arc<RwLock<T>>` with Hyper.
- How to deal with futures async without block a thread.
- How to parser dynamic endpoints (ex: `/123`).
- How to assembly JSON responses with Hyper.

### Next steps
- Data Validation
- Uses traits for handlers
- Add tests with `tokio::test`
- Add internal documentation



