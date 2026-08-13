use crate::context::Context;
use crate::error::{Result, Error};
use crate::generator::Registry;

const START_PREFIX: &str = "<!-- generated:";
const START_SUFFIX: &str = ":start -->";
const END_SUFFIX: &str = ":end -->";

/// Returns Some((key, is_start)) if the line is a generated-block marker.
fn parse_marker(line: &str) -> Option<(&str, bool)> {
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

/// Scans `body` for `<!-- generated:KEY:start -->` / `:end -->` marker
/// pairs, regenerates the content strictly between each pair using the
/// generator registered under KEY, and returns the resulting body.
///
/// Content outside marker pairs is passed through untouched. An unknown
/// key, an unmatched start, or an unmatched end is a hard error rather
/// than a silent skip.
pub fn process_markers(
    body: &str,
    registry: &Registry,
    ctx: &Context,
) -> Result<String> {
    let lines: Vec<&str> = body.lines().collect();
    let mut out_lines: Vec<String> = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];
        match parse_marker(line) {
            Some((key, true)) => {
                let key = key.to_string();
                out_lines.push(line.to_string());

                let mut end_idx = None;
                let mut j = i + 1;
                while j < lines.len() {
                    if let Some((end_key, is_start)) = parse_marker(lines[j]) {
                        if !is_start {
                            if end_key != key {
                                return Err(Error::Marker(format!(
                                    "{}: line {}: expected end marker for '{key}' but found end marker for '{end_key}'",
                                    ctx.file_path.display(),
                                    j + 1
                                )));
                            }
                            end_idx = Some(j);
                            break;
                        } else {
                            return Err(Error::Marker(format!(
                                "{}: line {}: nested generated block '{end_key}' found inside '{key}' \u{2014} generated blocks cannot nest",
                                ctx.file_path.display(),
                                j + 1
                            )));
                        }
                    }
                    j += 1;
                }

                let end_idx = end_idx.ok_or_else(|| {
                    Error::Marker(format!(
                        "{}: line {}: no matching end marker for generated block '{key}'",
                        ctx.file_path.display(),
                        i + 1
                    ))
                })?;

                match registry.get(key.as_str()) {
                    None => {
                        eprintln!(
                            "warning: {}: line {}: unknown generator key '{key}' \u{2014} no generator is registered under that name, leaving block as-is",
                            ctx.file_path.display(),
                            i + 1
                        );
                        // Preserve the block's existing content untouched.
                        for l in &lines[(i + 1)..end_idx] {
                            out_lines.push(l.to_string());
                        }
                    }
                    Some(generator) => {
                        let generated = generator.generate(ctx).map_err(|e| {
                            Error::Generator(format!(
                                "{}: block '{key}' (line {}): {e}",
                                ctx.file_path.display(),
                                i + 1
                            ))
                        })?;

                        for l in generated.lines() {
                            out_lines.push(l.to_string());
                        }
                    }
                }

                out_lines.push(lines[end_idx].to_string());
                i = end_idx + 1;
            }
            Some((key, false)) => {
                return Err(Error::Marker(format!(
                    "{}: line {}: end marker for '{key}' found without a matching start marker",
                    ctx.file_path.display(),
                    i + 1
                )));
            }
            None => {
                out_lines.push(line.to_string());
                i += 1;
            }
        }
    }

    let mut result = out_lines.join("\n");
    result.push('\n');
    Ok(result)
}