use thiserror::Error;

pub type Result<T> = std::result::Result<T, FrontmatterError>;

#[derive(Debug, Error)]
pub enum FrontmatterError {
    #[error("document must start with a '---' YAML frontmatter block")]
    MissingStart,
    #[error("could not find closing '---' for frontmatter block")]
    MissingEnd,
    #[error("invalid frontmatter YAML: {0}")]
    YamlError(#[from] serde_yaml::Error),
}