use crate::frontmatter::Frontmatter;
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Heading {
    pub level: u8,
    pub text: String,
    pub slug: String,
}

/// A minimal read-only view over the document's markdown structure.
/// Currently exposes the heading outline, which is all the bundled
/// generators need; extend this if a future generator needs more
/// (e.g. list items, links).
#[derive(Debug, Clone)]
pub struct Ast {
    pub headings: Vec<Heading>,
}

impl Ast {
    pub fn parse(body: &str) -> Ast {
        let mut headings = Vec::new();
        let mut current: Option<(u8, String)> = None;
        let mut used_slugs: HashMap<String, u32> = HashMap::new();

        for event in Parser::new(body) {
            match event {
                Event::Start(Tag::Heading{ level, id: _, classes: _, attrs: _}) => {
                    current = Some((heading_level_to_u8(level), String::new()));
                }
                Event::End(TagEnd::Heading(_)) => {
                    if let Some((level, text)) = current.take() {
                        let text = text.trim().to_string();
                        let slug = unique_slug(&text, &mut used_slugs);
                        headings.push(Heading { level, text, slug });
                    }
                }
                Event::Text(t) | Event::Code(t) => {
                    if let Some((_, text)) = current.as_mut() {
                        text.push_str(&t);
                    }
                }
                _ => {}
            }
        }

        Ast { headings }
    }
}

fn heading_level_to_u8(level: HeadingLevel) -> u8 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

fn slugify(text: &str) -> String {
    let mut s = String::new();
    let mut last_dash = true; // suppresses a leading dash
    for c in text.chars() {
        if c.is_alphanumeric() {
            s.push(c.to_ascii_lowercase());
            last_dash = false;
        } else if !last_dash {
            s.push('-');
            last_dash = true;
        }
    }
    while s.ends_with('-') {
        s.pop();
    }
    s
}

fn unique_slug(text: &str, used: &mut HashMap<String, u32>) -> String {
    let base = slugify(text);
    let count = used.entry(base.clone()).or_insert(0);
    let slug = if *count == 0 {
        base
    } else {
        format!("{base}-{count}")
    };
    *count += 1;
    slug
}

/// Everything a Generator needs to do its job: the file's own frontmatter
/// and heading outline, plus enough repo context to look up sibling
/// documents (used by generators like minors-table / patch-notes that
/// summarize a family of child documents by directory convention).
pub struct Context<'a> {
    pub file_path: &'a Path,
    pub repo_root: &'a Path,
    pub frontmatter: &'a Frontmatter,
    pub ast: &'a Ast,
}
