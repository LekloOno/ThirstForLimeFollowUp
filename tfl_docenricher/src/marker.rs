use tfl_docshared::marker::error::MarkerError;
use tfl_docshared::context::Context;
use tfl_docshared::marker;
use crate::generator::Registry;

/// Produces the replacement lines for the block between `start_idx` and
/// `end_idx` (exclusive of both marker lines): the registered generator's
/// output if `key` is known, or the block's existing lines untouched (with
/// a warning) if it isn't.
fn resolve_block_content(
    lines: &[&str],
    registry: &Registry,
    ctx: &Context,
    key: &str,
    start_idx: usize,
    end_idx: usize,
) -> crate::error::Result<Vec<String>> {
    match registry.get(key) {
        None => {
            eprintln!(
                "warning: {}: line {}: unknown generator key '{key}' \u{2014} no generator is registered under that name, leaving block as-is",
                ctx.file_path.display(),
                start_idx + 1
            );
            Ok(lines[(start_idx + 1)..end_idx]
                .iter()
                .map(|l| l.to_string())
                .collect())
        }
        Some(generator) => {
            let generated = generator.generate(ctx).map_err(|e| {
                crate::error::Error::Generator(format!(
                    "{}: block '{key}' (line {}): {e}",
                    ctx.file_path.display(),
                    start_idx + 1
                ))
            })?;
            Ok(generated.lines().map(|l| l.to_string()).collect())
        }
    }
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
) -> crate::error::Result<String> {
    let lines: Vec<&str> = body.lines().collect();
    let mut out_lines: Vec<String> = Vec::new();
    let mut i = 0;
 
    while i < lines.len() {
        match marker::parse_marker(lines[i]) {
            Some((key, true)) => {
                let key = key.to_string();
                let end_idx = marker::find_matching_end(&lines, &key, i, ctx)?;
                let block_lines = resolve_block_content(&lines, registry, ctx, &key, i, end_idx)?;
 
                out_lines.push(lines[i].to_string());
                out_lines.extend(block_lines);
                out_lines.push(lines[end_idx].to_string());
 
                i = end_idx + 1;
            }
            Some((key, false)) => {
                return Err(MarkerError::MissingStart {
                    ctx_path: ctx.file_path.display().to_string(),
                    line: i + 1,
                    key: key.to_string(),
                }.into());
            }
            None => {
                out_lines.push(lines[i].to_string());
                i += 1;
            }
        }
    }
 
    let mut result = out_lines.join("\n");
    result.push('\n');
    Ok(result)
}