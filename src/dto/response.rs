use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageResponse {
    pub message: String,
}

impl MessageResponse {
    pub fn new<S: Into<String>>(message: S) -> Self {
        Self {
            message: message.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageResponse<T> {
    pub data: Vec<T>,
    pub page_num: u64,
    pub page_size: u64,
    pub total: u64,
}

impl<T> PageResponse<T> {
    pub fn new(data: Vec<T>, page_num: u64, page_size: u64, total: u64) -> PageResponse<T> {
        PageResponse {
            data,
            page_num,
            page_size,
            total,
        }
    }

    pub fn map<F, B>(&self, f: F) -> PageResponse<B>
    where
        F: FnMut(&T) -> B,
    {
        let data: Vec<B> = self.data.iter().map(f).collect();
        PageResponse {
            data,
            page_num: self.page_num,
            page_size: self.page_size,
            total: self.total,
        }
    }
}
