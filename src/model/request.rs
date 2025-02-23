// use serde::{Deserialize, Serialize};
//
//
//
//
//
// #[derive(Debug, Clone, Serialize, Deserialize, Copy)]
// #[serde(default)]
// pub struct Pagination {
//     pub offset: u64,
//     pub limit: u32,
//     pub count: Option<bool>
// }
//
// impl From<&Pagination> for (i64, i32, bool) {
//     fn from(value: &Pagination) -> Self {
//         (
//             value.offset as i64,
//             value.limit as i32,
//             value.count.unwrap_or_default(),
//         )
//     }
// }
//
// impl From<Pagination> for (i64, i32, bool) {
//     fn from(value: Pagination) -> Self {
//         (
//             value.offset as i64,
//             value.limit as i32,
//             value.count.unwrap_or_default(),
//         )
//     }
// }
//
// impl Default for Pagination {
//     fn default() -> Self {
//         Self {
//             offset: 0,
//             limit: 10,
//             count: None,
//         }
//     }
// }
//
//
//
