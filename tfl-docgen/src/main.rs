use chrono::{Local, NaiveDate};
use clap::{Parser, Subcommand, ValueEnum};
use std::fmt;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

const TPL_GUIDE: &str = include_str!("../templates/guide.md");
const TPL_LOG: &str = include_str!("../templates/log.md");
const TPL_MAJOR_BRIEF: &str = include_str!("../templates/major_brief.md");
const TPL_PATCH_NOTE: &str = include_str!("../templates/patch_note.md");
const TPL_ROADMAP: &str = include_str!("../templates/roadmap.md");
const TPL_ROADMAP_MINOR: &str = include_str!("../templates/roadmap_minor.md");

#[derive(Parser)]
#[command(name = "tfl-docgen", about = "Generate Thirst for Lime follow-up documents from templates")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Create a new document from a template
    Create(CreateArgs),
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
#[value(rename_all = "snake_case")]
enum DocType {
    Guide,
    Log,
    MajorBrief,
    PatchNote,
    Roadmap,
    RoadmapMinor,
}

impl fmt::Display for DocType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            DocType::Guide => "guide",
            DocType::Log => "log",
            DocType::MajorBrief => "major_brief",
            DocType::PatchNote => "patch_note",
            DocType::Roadmap => "roadmap",
            DocType::RoadmapMinor => "roadmap_minor",
        };
        write!(f, "{s}")
    }
}

#[derive(Parser)]
struct CreateArgs {
    /// Document type
    #[arg(long, short, value_enum)]
    r#type: DocType,

    /// Release number (e.g. 0)
    #[arg(long, conflicts_with = "version")]
    release: Option<u32>,

    /// Major version number (e.g. 3)
    #[arg(long, conflicts_with = "version")]
    major: Option<u32>,

    /// Minor version number (e.g. 1)
    #[arg(long, conflicts_with = "version")]
    minor: Option<u32>,

    /// Full version shorthand, e.g. "0.3.1" or "0.3"
    #[arg(long, short)]
    version: Option<String>,

    /// Document title
    #[arg(long, short('T'))]
    title: String,

    /// Date in YYYY-MM-DD format. Defaults to today.
    #[arg(long, short)]
    date: Option<String>,

    /// Output path. Overrides the default type-based path convention.
    #[arg(long, short)]
    path: Option<PathBuf>,

    /// Base directory the default path convention is rooted at.
    #[arg(long, default_value = "../")]
    output_dir: PathBuf,

    /// Overwrite the destination file if it already exists.
    #[arg(long)]
    force: bool,
}

struct Version {
    release: Option<u32>,
    major: Option<u32>,
    minor: Option<u32>,
}

fn parse_version(args: &CreateArgs) -> Result<Version, String> {
    if let Some(v) = &args.version {
        let parts: Vec<&str> = v.split('.').collect();
        if parts.len() < 2 || parts.len() > 3 {
            return Err(format!(
                "--version expects \"release.major\" or \"release.major.minor\", got \"{v}\""
            ));
        }
        let release = parts[0]
            .parse::<u32>()
            .map_err(|_| format!("invalid release number in --version: \"{}\"", parts[0]))?;
        let major = parts[1]
            .parse::<u32>()
            .map_err(|_| format!("invalid major number in --version: \"{}\"", parts[1]))?;
        let minor = match parts.get(2) {
            Some(m) => Some(
                m.parse::<u32>()
                    .map_err(|_| format!("invalid minor number in --version: \"{m}\""))?,
            ),
            None => None,
        };
        Ok(Version {
            release: Some(release),
            major: Some(major),
            minor,
        })
    } else {
        Ok(Version {
            release: args.release,
            major: args.major,
            minor: args.minor,
        })
    }
}

/// Which version components a given doc type requires, in order.
fn required_components(t: DocType) -> &'static [&'static str] {
    match t {
        DocType::Guide | DocType::Log => &[],
        DocType::MajorBrief | DocType::Roadmap => &["release", "major"],
        DocType::PatchNote | DocType::RoadmapMinor => &["release", "major", "minor"],
    }
}

fn slugify(title: &str) -> String {
    let mut slug = String::new();
    let mut last_was_dash = false;
    for c in title.chars() {
        if c.is_alphanumeric() {
            slug.push(c.to_ascii_lowercase());
            last_was_dash = false;
        } else if !last_was_dash && !slug.is_empty() {
            slug.push('-');
            last_was_dash = true;
        }
    }
    while slug.ends_with('-') {
        slug.pop();
    }
    slug
}

fn run(args: CreateArgs) -> Result<PathBuf, String> {
    let version = parse_version(&args)?;

    for component in required_components(args.r#type) {
        let present = match *component {
            "release" => version.release.is_some(),
            "major" => version.major.is_some(),
            "minor" => version.minor.is_some(),
            _ => unreachable!(),
        };
        if !present {
            return Err(format!(
                "--type {} requires --{component} (or a --version with enough components)",
                args.r#type
            ));
        }
    }
    if args.r#type == DocType::MajorBrief && version.minor.is_some() {
        eprintln!(
            "warning: --type major_brief doesn't use a minor version, ignoring the one provided"
        );
    }

    let date = match &args.date {
        Some(d) => NaiveDate::parse_from_str(d, "%Y-%m-%d")
            .map_err(|_| format!("--date must be in YYYY-MM-DD format, got \"{d}\""))?
            .format("%Y-%m-%d")
            .to_string(),
        None => Local::now().format("%Y-%m-%d").to_string(),
    };

    let release = version.release.map(|v| v.to_string()).unwrap_or_default();
    let major = version.major.map(|v| v.to_string()).unwrap_or_default();
    let minor = version.minor.map(|v| v.to_string()).unwrap_or_default();
    let slug = slugify(&args.title);

    let template = match args.r#type {
        DocType::Guide => TPL_GUIDE,
        DocType::Log => TPL_LOG,
        DocType::MajorBrief => TPL_MAJOR_BRIEF,
        DocType::PatchNote => TPL_PATCH_NOTE,
        DocType::Roadmap => TPL_ROADMAP,
        DocType::RoadmapMinor => TPL_ROADMAP_MINOR,
    };

    let content = template
        .replace("TITLE_KEY", &args.title)
        .replace("DATE_KEY", &date)
        .replace("RELEASE_KEY", &release)
        .replace("MAJOR_KEY", &major)
        .replace("MINOR_KEY", &minor)
        .replace("GUIDE_KEY", &slug);

    let dest = match &args.path {
        Some(p) => p.clone(),
        None => default_path(&args.output_dir, args.r#type, &release, &major, &minor, &slug, &date),
    };

    if dest.exists() && !args.force {
        return Err(format!(
            "{} already exists (pass --force to overwrite)",
            dest.display()
        ));
    }

    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("failed to create {}: {e}", parent.display()))?;
    }
    fs::write(&dest, content).map_err(|e| format!("failed to write {}: {e}", dest.display()))?;

    Ok(dest)
}

fn default_path(
    base: &PathBuf,
    t: DocType,
    release: &str,
    major: &str,
    minor: &str,
    slug: &str,
    date: &str,
) -> PathBuf {
    let mut p = base.clone();
    match t {
        DocType::Guide => {
            p.push("guides");
            p.push(format!("{slug}.md"));
        }
        DocType::Log => {
            p.push("logs");
            p.push(format!("{date}.md"));
        }
        DocType::MajorBrief => {
            p.push("major_briefs");
            p.push(format!("v{release}.{major}-brief.md"));
        }
        DocType::PatchNote => {
            p.push("patch_notes");
            p.push(format!("v{release}.{major}"));
            p.push(format!("v{release}.{major}.{minor}.md"));
        }
        DocType::Roadmap => {
            p.push("roadmaps");
            p.push(format!("v{release}.{major}-roadmap.md"));
        }
        DocType::RoadmapMinor => {
            p.push("roadmaps");
            p.push(format!("v{release}.{major}"));
            p.push(format!("v{release}.{major}.{minor}.md"));
        }
    }
    p
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.command {
        Command::Create(args) => match run(args) {
            Ok(path) => {
                println!("Created {}", path.display());
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("error: {e}");
                ExitCode::FAILURE
            }
        },
    }
}
