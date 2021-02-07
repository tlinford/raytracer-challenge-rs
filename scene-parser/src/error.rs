use thiserror::Error;

#[derive(Error, Debug)]
pub enum SceneParserError {
    #[error("invalid input file `{0}`")]
    BadInputFile(String),
    #[error("missing required key `{0}`")]
    MissingRequiredKey(String),
    #[error("failed to parse `{0}` as i64")]
    ParseIntError(String),
    #[error("failed to parse `{0}` as f64")]
    ParseFloatError(String),
    #[error("failed to parse `{0}` as vec")]
    ParseVecError(String),
    #[error("failed to parse transform")]
    ParseTransformError,
    #[error("failed to parse material")]
    ParseMaterialError,
    #[error("invalid add element found")]
    InvalidAddElementError,
    #[error("invalid define element found")]
    InvalidDefineElementError,
    #[error("failed to parse pattern")]
    ParsePatternError,
}
