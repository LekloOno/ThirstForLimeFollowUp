use tfl_docshared::context::Context;
use crate::generator::Generator;
use crate::error::Result;

/// Builds a nested bullet list from the document's own heading outline.
/// The H1 (the doc's title, itself generated) is excluded — a toc linking
/// to the document's own title would be redundant.
pub struct Toc;

impl Generator for Toc {
    fn key() -> &'static str {
        "toc"
    }

    fn depends_on_structure() -> bool
    where
        Self: Sized {
        true
    }

    fn new() -> Box<dyn Generator>
    where
        Self: Sized {
        Box::new(Toc)
    }

    fn generate(&self, ctx: &Context) -> Result<String> {
        let headings: Vec<_> = ctx.ast.headings.iter().filter(|h| h.level >= 2).collect();
        if headings.is_empty() {
            return Ok(String::new());
        }
        let min_level = headings.iter().map(|h| h.level).min().unwrap();

        let mut out = String::new();
        for h in headings {
            let indent = "  ".repeat((h.level - min_level) as usize);
            out.push_str(&format!("{indent}- [{}](#{})\n", h.text, h.slug));
        }
        out.pop(); // trailing newline; process_markers re-adds line structure
        Ok(out)
    }
}
