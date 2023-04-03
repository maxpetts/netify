use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Page<T> {
    pub href: String,
    pub items: Vec<T>,
    pub limit: u32,
    pub next: Option<String>,
    pub offset: u32,
    pub previous: Option<String>,
    pub total: u32,
}
