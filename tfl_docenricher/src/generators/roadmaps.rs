use crate::generators::minors_table::render_minors_table;
use crate::generator::Generator;
use crate::context::Context;
use crate::error::{Result, Error};
use crate::links::family_link;

/// Builds the `## Roadmaps` block on a `major_brief` doc: a link to this
/// major's own roadmap doc, followed by the same minors table shown on
/// the roadmap doc itself (reused, not recomputed differently — both are
/// reflections of the same `roadmap_minor` siblings).
pub struct Roadmaps;

impl Generator for Roadmaps {
    fn key() -> &'static str {
        "roadmaps"
    }

    fn depends_on_structure() -> bool
    where
        Self: Sized {
        false
    }
    
    fn new() -> Box<dyn Generator>
    where
        Self: Sized {
        Box::new(Roadmaps)
    }

    fn generate(&self, ctx: &Context) -> Result<String> {
        let v = ctx.frontmatter.version.as_ref().ok_or_else(|| {
            Error::Generator("major_brief requires a version block".to_string())
        })?;

        let roadmap_file = format!("v{}.{}-roadmap.md", v.release, v.major);
        let roadmap_link = family_link(ctx, "roadmaps", &roadmap_file)?;

        let mut out = format!("## Roadmaps\n\n[{}.{} Roadmap]({roadmap_link})\n", v.release, v.major);
        if let Some(table) = render_minors_table(ctx, v.release, v.major)? {
            out.push('\n');
            out.push_str(&table);
        }
        Ok(out.trim_end().to_string())
    }
}
