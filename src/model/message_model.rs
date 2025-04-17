use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ResponseJson<T> {
    pub msg: T,
}

#[derive(Debug, Serialize)]
pub struct ResponseJsonWithResult<T, S> {
    pub msg: T,
    pub result: S,
}
