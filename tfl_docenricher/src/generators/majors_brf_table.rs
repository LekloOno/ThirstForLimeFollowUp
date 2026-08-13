use crate::frontmatter::DocType;
use crate::generator::Generator;
use crate::context::Context;
use crate::error::{Result, Error};
use crate::links::family_link;
use crate::sibling_scan::scan_siblings;
 
/// Builds the `## Major Briefs` block: a table of every `major_brief` doc found
/// directly under `major_briefs/`.
/// It can be embedded in any document.
pub struct MajorsBrfTable;
 
impl Generator for MajorsBrfTable {
    fn key() -> &'static str {
        "majors-brf-table"
    }

    fn depends_on_structure() -> bool
    where
        Self: Sized {
        false
    }

    fn new() -> Box<dyn Generator>
    where
        Self: Sized {
        Box::new(MajorsBrfTable {})
    }
 
    fn generate(&self, ctx: &Context) -> Result<String> {
        let mut out = String::from("## Majors\n");
        if let Some(table) = render_majors_table(ctx)? {
            out.push('\n');
            out.push_str(&table);
        }
        Ok(out)
    }
}
 
fn render_majors_table(ctx: &Context) -> Result<Option<String>> {
    let dir = ctx.repo_root.join("major_briefs");
    // scan_siblings only picks up direct .md files, so the vR.M/ minor
    // subdirectories living alongside these docs are naturally skipped —
    // no separate "top-level only" logic needed.
    let mut majors = scan_siblings(&dir, DocType::MajorBrief)?;
    majors.sort_by_key(|fm| fm.version.map(|v| (v.release, v.major)).unwrap_or((u32::MAX, u32::MAX)));
 
    if majors.is_empty() {
        return Ok(None);
    }
 
    let mut out = String::new();
    out.push_str("| version       | brief   |\n");
    out.push_str("|---------------|-----------|\n");
    for fm in &majors {
        let v = fm.version.as_ref().ok_or_else(|| {
            Error::Generator(format!("{}: brief missing version block", fm.id))
        })?;
        let version_str = format!("{}.{}", v.release, v.major);
        let link = family_link(ctx, "major_briefs", &format!("v{version_str}-brief.md"))?;
        out.push_str(&format!(
            "| `{version_str}` | [{}]({link}) |\n",
            fm.title
        ));
    }
    out.pop();
    Ok(Some(out))
}
