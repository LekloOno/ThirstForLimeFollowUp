use crate::generator::Generator;
use crate::context::Context;
use crate::error::{Result, Error};

/// Regenerates the `# ...` H1 heading from frontmatter
///   guide, log                -> "# {title}"
///   major_brief               -> "# {release}.{major} - {title}"
///   roadmap                   -> "# {release}.{major} Roadmap - {title}"
///   patch_note, roadmap_minor -> "# {release}.{major}.{minor} - {title}"
pub struct Title;

impl Generator for Title {
    fn key() -> &'static str {
        "title"
    }

    fn depends_on_structure() -> bool
    where
        Self: Sized {
        false
    }
    
    fn new() -> Box<dyn Generator>
    where
        Self: Sized {
        Box::new(Title)
    }

    fn generate(&self, ctx: &Context) -> Result<String> {
        let fm = ctx.frontmatter;
        let heading = match fm.doc_type.as_str() {
            "guide" | "log" => format!("# {}", fm.title),
            "major_brief" => {
                let v = require_version(fm, "major_brief")?;
                format!("# {}.{} - {}", v.release, v.major, fm.title)
            }
            "roadmap" => {
                let v = require_version(fm, "roadmap")?;
                format!("# {}.{} Roadmap - {}", v.release, v.major, fm.title)
            }
            "patch_note" | "roadmap_minor" => {
                let v = require_version(fm, &fm.doc_type)?;
                let minor = v.minor.ok_or_else(|| {
                    Error::Generator(format!(
                        "{} requires version.minor to build its title",
                        fm.doc_type
                    ))
                })?;
                format!("# {}.{}.{} - {}", v.release, v.major, minor, fm.title)
            }
            other => {
                return Err(Error::Generator(format!(
                    "no title convention known for doc type '{other}'"
                )))
            }
        };
        Ok(heading)
    }
}

fn require_version<'a>(
    fm: &'a crate::frontmatter::Frontmatter,
    doc_type: &str,
) -> Result<&'a crate::frontmatter::Version> {
    fm.version.as_ref().ok_or_else(|| {
        Error::Generator(format!("{doc_type} requires a version block to build its title"))
    })
}
