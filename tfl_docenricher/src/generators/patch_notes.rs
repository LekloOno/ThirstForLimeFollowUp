use tfl_docshared::frontmatter::DocType;
use tfl_docshared::context::Context;
use crate::generator::Generator;
use crate::error::{Result, Error};
use crate::links::family_link;
use crate::sibling_scan::scan_siblings;

/// Builds the `## Patch notes` block on a `major_brief` doc by scanning
/// `patch_notes/v{release}.{major}/*.md` for `patch_note` siblings. Like
/// minors-rmp-table, this is a pure reflection of the children's own
/// frontmatter — the relationship lives in directory convention, not in
/// a stored `children` link, since it's presentational, not semantic.
pub struct PatchNotes;

impl Generator for PatchNotes {
    fn key() -> &'static str {
        "patch_notes"
    }

    fn depends_on_structure() -> bool
    where
        Self: Sized {
        false
    }
    
    fn new() -> Box<dyn Generator>
    where
        Self: Sized {
        Box::new(PatchNotes)
    }

    fn generate(&self, ctx: &Context) -> Result<String> {
        let v = ctx.frontmatter.version.as_ref().ok_or_else(|| {
            Error::Generator("major_brief requires a version block".to_string())
        })?;

        let mut out = String::from("## Patch notes\n");
        if let Some(table) = render_patch_notes_table(ctx, v.release, v.major)? {
            out.push('\n');
            out.push_str(&table);
        }
        Ok(out)
    }
}

pub(crate) fn render_patch_notes_table(
    ctx: &Context,
    release: u32,
    major: u32,
) -> Result<Option<String>> {
    let dir_name = format!("v{release}.{major}");
    let dir = ctx.repo_root.join("patch_notes").join(&dir_name);
    let mut siblings = scan_siblings(&dir, DocType::PatchNote)?;
    siblings.sort_by_key(|fm| fm.version.and_then(|v| v.minor).unwrap_or(u32::MAX));

    if siblings.is_empty() {
        return Ok(None);
    }

    let mut out = String::new();
    out.push_str("| version       | patch note |\n");
    out.push_str("|---------------|------------|\n");
    for fm in &siblings {
        let sv = fm.version.as_ref().ok_or_else(|| {
            Error::Generator(format!("{}: patch_note missing version block", fm.id))
        })?;
        let minor = sv.minor.ok_or_else(|| {
            Error::Generator(format!("{}: patch_note missing version.minor", fm.id))
        })?;
        let version_str = format!("{}.{}.{}", sv.release, sv.major, minor);
        let link = family_link(ctx, "patch_notes", &format!("{dir_name}/v{version_str}.md"))?;
        out.push_str(&format!(
            "| `{version_str}` | [{}]({link}) |\n",
            fm.title
        ));
    }
    out.pop();
    Ok(Some(out))
}
