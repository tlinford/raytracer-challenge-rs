use thiserror::Error;

#[derive(Error, Debug)]
pub enum SceneParserError {
    #[error("missing required key `{0}`")]
    MissingRequiredKey(String),
    #[error("failed to parse `{0}` as i64")]
    ParseIntError(String),
    #[error("failed to parse `{0}` as f64")]
    ParseFloatError(String),
    #[error("failed to parse `{0}` as vec")]
    ParseVecError(String),
}
