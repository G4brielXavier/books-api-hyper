
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::model::Book;

pub type AppState = Arc<RwLock<Vec<Book>>>;