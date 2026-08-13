use thiserror::Error;

pub type Result<T> = std::result::Result<T, MarkerError>;

#[derive(Debug, Error)]
pub enum MarkerError {
    #[error("{ctx_path}: line {line}: nested generated block '{key}' found inside '{nested_key}' - generated blocks cannot nest")]
    NestedBlock     { ctx_path: String, line: usize, key: String, nested_key: String },

    #[error("{ctx_path}: line {line}: expected end marker for '{key}' but found end marker for '{mismatch_key}'")]
    MismatchEnd     { ctx_path: String, line: usize, key: String, mismatch_key: String },

    #[error("{ctx_path}: line {line}: no matching end marker for generated block '{key}'")]
    MissingEnd      { ctx_path: String, line: usize, key: String },

    #[error("{ctx_path}: line {line}: end marker for '{key}' found without a matching start marker")]
    MissingStart    { ctx_path: String, line: usize, key: String },
}