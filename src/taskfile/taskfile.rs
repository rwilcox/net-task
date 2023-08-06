use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Taskfile {
    pub version: String
}
