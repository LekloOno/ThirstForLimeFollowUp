use crate::frontmatter::DocType;
use crate::generator::Generator;
use crate::context::Context;
use crate::error::{Result, Error};
use crate::label;
use crate::links::family_link;
use crate::sibling_scan::scan_siblings;
 
/// Builds the `## Majors` block: a table of every `roadmap` doc found
/// directly under `roadmaps/`.
/// Unlike `minors-rmp-table`, this doesn't depend on the hosting doc's own
/// version, since it isn't scoped to one major - 
/// it can be embedded in any document.
pub struct MajorsRmpTable;
 
impl Generator for MajorsRmpTable {
    fn key() -> &'static str {
        "majors-rmp-table"
    }

    fn depends_on_structure() -> bool
    where
        Self: Sized {
        false
    }

    fn new() -> Box<dyn Generator>
    where
        Self: Sized {
        Box::new(MajorsRmpTable {})
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
    let dir = ctx.repo_root.join("roadmaps");
    // scan_siblings only picks up direct .md files, so the vR.M/ minor
    // subdirectories living alongside these docs are naturally skipped —
    // no separate "top-level only" logic needed.
    let mut majors = scan_siblings(&dir, DocType::Roadmap)?;
    majors.sort_by_key(|fm| fm.version.map(|v| (v.release, v.major)).unwrap_or((u32::MAX, u32::MAX)));
 
    if majors.is_empty() {
        return Ok(None);
    }
 
    let mut out = String::new();
    out.push_str("| version       | status        | roadmap   |\n");
    out.push_str("|---------------|---------------|-----------|\n");
    for fm in &majors {
        let v = fm.version.as_ref().ok_or_else(|| {
            Error::Generator(format!("{}: roadmap missing version block", fm.id))
        })?;
        let version_str = format!("{}.{}", v.release, v.major);
        let status = label::roadmap_status_label(fm.roadmap_status.as_deref().unwrap_or("planned"))?;
        let link = family_link(ctx, "roadmaps", &format!("v{version_str}-roadmap.md"))?;
        out.push_str(&format!(
            "| `{version_str}` | {status} | [{}]({link}) |\n",
            fm.title
        ));
    }
    out.pop();
    Ok(Some(out))
}
