use tfl_docshared::frontmatter::DocType;
use tfl_docshared::context::Context;
use crate::generator::Generator;
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
        let heading = match fm.doc_type {
            DocType::Guide | DocType::Readme | DocType::Log => format!("# {}", fm.title),
            DocType::MajorBrief => {
                let v = require_version(fm)?;
                format!("# {}.{} - {}", v.release, v.major, fm.title)
            }
            DocType::Roadmap => {
                let v = require_version(fm)?;
                format!("# {}.{} Roadmap - {}", v.release, v.major, fm.title)
            }
            DocType::PatchNote | DocType::RoadmapMinor => {
                let v = require_version(fm)?;
                let minor = v.minor.ok_or_else(|| {
                    Error::Generator(format!(
                        "{} requires version.minor to build its title",
                        fm.doc_type
                    ))
                })?;
                format!("# {}.{}.{} - {}", v.release, v.major, minor, fm.title)
            }
        };
        Ok(heading)
    }
}

fn require_version(
    fm: &tfl_docshared::frontmatter::Frontmatter,
) -> Result<&tfl_docshared::frontmatter::Version> {
    fm.version.as_ref().ok_or_else(|| {
        Error::Generator(format!(
            "{} requires a version block to build its title",
            fm.doc_type
        ))
    })
}
