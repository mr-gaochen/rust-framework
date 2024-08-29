use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, Serialize, Dummy, ToSchema, IntoParams, Clone)]
pub struct PageQueryParam {
    pub page_num: u64,
    pub page_size: u64,
    pub sort_by: Option<String>,
    pub sort_direction: Option<Direction>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    DESC,
    ASC,
}

// TODO #![feature(unboxed_closures)] unstable
impl Direction {
    pub fn as_closure<T>(&self) -> impl Fn((T, T)) -> bool
    where
        T: Ord,
    {
        match self {
            Direction::ASC => |(a, b)| a <= b,
            Direction::DESC => |(a, b)| a >= b,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct IdsReq {
    pub ids: String,
}
