use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Frontmatter {
    pub id: String,
    #[serde(rename = "type")]
    pub doc_type: String,
    pub status: String,
    #[serde(default)]
    pub roadmap_status: Option<String>,
    pub title: String,
    pub date: String,
    #[serde(default)]
    pub version: Option<Version>,
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct Version {
    pub release: u32,
    pub major: u32,
    #[serde(default)]
    pub minor: Option<u32>,
}

/// Splits a raw document into (frontmatter, raw_yaml_text, body).
/// Only the body is ever touched by enrichment - the raw YAML text is
/// preserved byte-for-byte so frontmatter formatting is never altered.
pub fn split_frontmatter(raw: &str) -> Result<(Frontmatter, String, String), String> {
    let raw = raw.strip_prefix('\u{feff}').unwrap_or(raw); // tolerate a BOM
    let mut rest = raw
        .strip_prefix("---\n")
        .or_else(|| raw.strip_prefix("---\r\n"))
        .ok_or_else(|| "document must start with a '---' YAML frontmatter block".to_string())?;

    let mut yaml_end = None;
    let mut offset = 0usize;
    for line in rest.split_inclusive('\n') {
        let trimmed = line.trim_end_matches(['\n', '\r']);
        if trimmed == "---" {
            yaml_end = Some(offset);
            offset += line.len();
            break;
        }
        offset += line.len();
    }
    
    if yaml_end.is_none() && rest.trim_end_matches(['\n', '\r']) == "---" {
        yaml_end = Some(0);
        offset = rest.len();
    }

    let yaml_end = yaml_end
        .ok_or_else(|| "could not find closing '---' for frontmatter block".to_string())?;
    let yaml_str = rest[..yaml_end].trim_end_matches(['\n', '\r']).to_string();
    rest = &rest[offset..];
    let body = rest.to_string();

    let fm: Frontmatter = serde_yaml::from_str(&yaml_str)
        .map_err(|e| format!("invalid frontmatter YAML: {e}"))?;

    Ok((fm, yaml_str, body))
}
