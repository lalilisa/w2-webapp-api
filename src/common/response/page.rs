use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct Page<T: Serialize> {
    pub page: u64,
    pub limit: u32,
    pub total: Option<u64>,
    pub data: Vec<T>,
}



impl<T: Serialize> Page<T> {
    pub fn new(page: u64, limit: u32, total: u64, data: Vec<T>) -> Page<T> {
        Self {
            page,
            limit,
            total: Some(total),
            data,
        }
    }

    pub fn map<B, F>(self, f: F) -> Page<B>
    where
        Self: Sized,
        F: FnMut(T) -> B,
        B: Serialize
    {
        Page {
            page: self.page,
            limit: self.limit,
            total: self.total,
            data: self.data.into_iter().map(f).collect()
        }
    }
}

impl<T: Serialize> Iterator for Page<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.data.is_empty() {
            Some(self.data.remove(0))
        } else {
            None
        }
    }
}

pub trait IntoPage<Item: Serialize> {
    type Item;
    fn into_page(self, page_request: impl Into<(i64, i32, bool)>) -> Page<Item>;
}

impl<T: Serialize> IntoPage<T> for (Vec<T>, Option<i64>) {
    type Item = T;
    fn into_page(self, page_request: impl Into<(i64, i32, bool)>) -> Page<T> {
        let page_request = page_request.into();
        let page = if page_request.1 == 1 {
            0
        } else {
            (page_request.0 / page_request.1 as i64) as u64
        };
        Page {
            page,
            limit: page_request.1 as u32,
            total: self.1.map(|x| x as u64),
            data: self.0,
        }
    }
}

// impl<T: Serialize> IntoPage<T> for (Vec<T>, Option<u64>) {
//     type Item = T;
//     fn into_page(self, page_request: impl Into<(i64, i32, bool)>) -> Page<T> {
//         let page_request = page_request.into();
//         let page = if page_request.1 == 1 {
//             0
//         } else {
//             (page_request.0 / page_request.1 as i64) as u64
//         };
//         Page {
//             page,
//             limit: page_request.1 as u32,
//             total: self.1,
//             data: self.0,
//         }
//     }
// }
