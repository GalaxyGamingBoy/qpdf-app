use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PDFDetails {
    pub title: String,
    pub author: String,
}
