use crate::error::{Result, Error};
use crate::frontmatter::{DocType, Frontmatter, split_frontmatter};
use std::fs;
use std::path::Path;

/// Reads every `.md` file directly inside `dir` and parses its
/// frontmatter. Every file found must be of `expected_type` - a
/// directory reserved for one doc type by convention (e.g.
/// `roadmaps/v0.3/` for `roadmap_minor`) should never silently contain
/// something else, since that's exactly the kind of drift that leaves a
/// generated table wrong without anyone noticing.
///
/// A missing directory is not an error: it just means no children exist
/// yet (e.g. a freshly created roadmap with no minors published).
pub fn scan_siblings(dir: &Path, expected_type: DocType) -> Result<Vec<Frontmatter>> {
    if !dir.exists() {
        return Ok(Vec::new());
    }

    let mut entries: Vec<_> = fs::read_dir(dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|ext| ext == "md").unwrap_or(false))
        .collect();
    entries.sort_by_key(|e| e.file_name());

    let mut out = Vec::with_capacity(entries.len());
    for entry in entries {
        let path = entry.path();
        let raw = fs::read_to_string(&path)?;
        let (fm, _, _) = split_frontmatter(&raw)
            .map_err(|e| Error::Frontmatter(format!("{}: {e}", path.display())))?;

        if fm.doc_type != expected_type {
            return Err(Error::Generator(format!(
                "{} has type '{}', but {} is a directory reserved for '{expected_type}' docs by convention",
                path.display(),
                fm.doc_type,
                dir.display()
            )));
        }
        out.push(fm);
    }

    Ok(out)
}
