#[derive(Debug, serde::Deserialize)]
pub struct ChapterForm {
    pub pre: String,
    pub main: String,
    pub post: String,
}
