

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Book {
    pub id: u64,
    pub name: String,
    pub author: String,
    pub pages: u64
}