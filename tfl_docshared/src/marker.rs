use crate::marker::error::{MarkerError, Result};

pub mod error;

pub const START_PREFIX: &str = "<!-- generated:";
pub const START_SUFFIX: &str = ":start -->";
pub const END_SUFFIX: &str = ":end -->";

/// Returns Some((key, is_start)) if the line is a generated-block marker.
pub fn parse_marker(line: &str) -> Option<(&str, bool)> {
    let trimmed = line.trim();
    let rest = trimmed.strip_prefix(START_PREFIX)?;
    if let Some(key) = rest.strip_suffix(START_SUFFIX) {
        return Some((key, true));
    }
    if let Some(key) = rest.strip_suffix(END_SUFFIX) {
        return Some((key, false));
    }
    None
}

/// Scans forward from `start_idx + 1` for the `:end` marker matching `key`.
/// Any other marker encountered first, a mismatched end, or a nested
/// start, is an error rather than something to skip past.
pub fn find_matching_end(
    lines: &[&str],
    key: &str,
    start_idx: usize,
    path: &str,
) -> Result<usize> {
    let mut j = start_idx + 1;
    while j < lines.len() {
        if let Some((found_key, is_start)) = parse_marker(lines[j]) {
            if is_start {
                return Err(MarkerError::NestedBlock {
                    ctx_path: path.to_string(),
                    line: j + 1,
                    key: key.to_string(),
                    nested_key: found_key.to_string(),
                });
            }
            if found_key != key {
                return Err(MarkerError::MismatchEnd {
                    ctx_path: path.to_string(),
                    line: j + 1,
                    key: key.to_string(),
                    mismatch_key: found_key.to_string(),
                });
            }
            return Ok(j);
        }
        j += 1;
    }
    Err(MarkerError::MissingEnd {
        ctx_path: path.to_string(),
        line: start_idx + 1,
        key: key.to_string(),
    })
}

/// Removes all generated blocks from `lines`, including their marker lines
/// and their content.
///
/// Content outside generated blocks is preserved.
pub fn clean_markers<'a>(
    lines: &[&'a str],
    path: &str,
) -> Result<Vec<&'a str>> {
    let mut out_lines = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        match parse_marker(lines[i]) {
            Some((key, true)) => {
                // Skip the entire block, including both markers.
                let end_idx = find_matching_end(lines, key, i, path)?;
                i = end_idx + 1;
            }
            Some((key, false)) => {
                // An end marker without a start marker.
                return Err(MarkerError::MissingStart {
                    ctx_path: path.to_string(),
                    line: i + 1,
                    key: key.to_string(),
                }
                .into());
            }
            None => {
                out_lines.push(lines[i]);
                i += 1;
            }
        }
    }

    Ok(out_lines)
}