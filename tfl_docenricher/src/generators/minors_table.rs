use crate::generator::Generator;
use crate::context::Context;
use crate::error::{Result, Error};
use crate::links::family_link;
use crate::sibling_scan::scan_siblings;

/// Builds the `## Minors` block on a `roadmap` doc by scanning
/// `roadmaps/v{release}.{major}/*.md` for `roadmap_minor` siblings and
/// reading each one's own frontmatter (status is never duplicated into
/// the parent, this table is always a reflection of the children, never
/// a second source of truth for it).
pub struct MinorsTable;

impl Generator for MinorsTable {
    fn key() -> &'static str {
        "minors-table"
    }

    fn depends_on_structure() -> bool
    where
        Self: Sized {
        false
    }

    fn new() -> Box<dyn Generator>
    where
        Self: Sized {
        Box::new(MinorsTable {})
    }

    fn generate(&self, ctx: &Context) -> Result<String> {
        let v = ctx.frontmatter.version.as_ref().ok_or_else(|| {
            Error::Generator("roadmap requires a version block".to_string())
        })?;

        let mut out = String::from("## Minors\n");
        if let Some(table) = render_minors_table(ctx, v.release, v.major)? {
            out.push('\n');
            out.push_str(&table);
        }
        Ok(out)
    }
}

pub(crate) fn render_minors_table(
    ctx: &Context,
    release: u32,
    major: u32,
) -> Result<Option<String>> {
    let dir_name = format!("v{release}.{major}");
    let dir = ctx.repo_root.join("roadmaps").join(&dir_name);
    let mut siblings = scan_siblings(&dir, "roadmap_minor")?;
    siblings.sort_by_key(|fm| fm.version.and_then(|v| v.minor).unwrap_or(u32::MAX));

    if siblings.is_empty() {
        return Ok(None);
    }

    let mut out = String::new();
    out.push_str("| version       | status        | roadmap   |\n");
    out.push_str("|---------------|---------------|-----------|\n");
    for fm in &siblings {
        let sv = fm.version.as_ref().ok_or_else(|| {
            Error::Generator(format!("{}: roadmap_minor missing version block", fm.id))
        })?;
        let minor = sv.minor.ok_or_else(|| {
            Error::Generator(format!("{}: roadmap_minor missing version.minor", fm.id))
        })?;
        let version_str = format!("{}.{}.{}", sv.release, sv.major, minor);
        let status = roadmap_status_label(fm.roadmap_status.as_deref().unwrap_or("planned"))?;
        let link = family_link(ctx, "roadmaps", &format!("{dir_name}/v{version_str}.md"))?;
        out.push_str(&format!(
            "| `{version_str}` | {status} | [{}]({link}) |\n",
            fm.title
        ));
    }
    out.pop();
    Ok(Some(out))
}

fn roadmap_status_label(status: &str) -> Result<&'static str> {
    Ok(match status {
        "planned" => "\u{26AB} PLANNED",
        "cancelled" => "\u{1F534} CANCELLED",
        "delayed" => "\u{1F7E0} DELAYED",
        "wip" => "\u{1F7E1} WIP",
        "completed" => "\u{1F535} COMPLETED",
        "live" => "\u{1F7E2} LIVE",
        other => {
            return Err(Error::Generator(format!(
                "unknown roadmap_status '{other}' (expected one of planned, cancelled, delayed, wip, completed, live)"
            )))
        }
    })
}
