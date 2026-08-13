use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("frontmatter error: {0}")]
    Frontmatter(String),

    #[error("marker error: {0}")]
    Marker(String),

    #[error("generator error: {0}")]
    Generator(String),

    #[error("{0}")]
    Unstable(String),
}