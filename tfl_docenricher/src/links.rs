use tfl_docshared::context::Context;
use crate::error::{Result, Error};

/// Builds a relative markdown link from the current document to
/// `{target_top_dir}/{sub_path}`, correct regardless of which top-level
/// doc-type directory the current file lives in.
///
/// Every doc-type directory (roadmaps/, major_briefs/, patch_notes/, ...)
/// sits exactly one level below repo_root, so this only needs to check
/// whether the current file is already inside `target_top_dir` (in which
/// case no "../" round trip is needed) or in a sibling directory (in
/// which case one "../" gets back to repo_root before descending into
/// `target_top_dir`).
pub fn family_link(ctx: &Context, target_top_dir: &str, sub_path: &str) -> Result<String> {
    let relative = ctx.file_path.strip_prefix(ctx.repo_root).map_err(|_| {
        Error::Generator(format!(
            "{} is not located under --repo-root {} (needed to compute links to sibling doc families)",
            ctx.file_path.display(),
            ctx.repo_root.display()
        ))
    })?;

    let own_top_dir = relative
        .components()
        .next()
        .and_then(|c| c.as_os_str().to_str())
        .unwrap_or("");

    if own_top_dir == target_top_dir {
        Ok(sub_path.to_string())
    } else {
        Ok(format!("../{target_top_dir}/{sub_path}"))
    }
}
