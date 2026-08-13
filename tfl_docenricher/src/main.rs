use clap::Parser;
use tfl_docenricher::context::{Ast, Context};
use tfl_docenricher::error::{Result, Error};
use tfl_docenricher::frontmatter::split_frontmatter;
use tfl_docenricher::generators;
use tfl_docenricher::marker;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

const MAX_PASSES: usize = 3;

#[derive(Parser)]
#[command(
    name = "tfl-enrich",
    about = "Regenerate <!-- generated:KEY:start/end --> blocks in a follow-up markdown doc"
)]
struct Cli {
    /// The markdown file to enrich.
    path: PathBuf,

    /// Repo root generators resolve sibling-document conventions against
    /// (e.g. roadmaps/vR.M/, patch_notes/vR.M/). Defaults to the parent
    /// directory.
    #[arg(long, default_value = "../")]
    repo_root: PathBuf,

    /// Print the result to stdout instead of writing it back to `path`.
    #[arg(long, conflicts_with = "check")]
    stdout: bool,

    /// Don't write anything; exit with a non-zero status if enrichment
    /// would change the file. Useful as a CI gate.
    #[arg(long, conflicts_with = "stdout")]
    check: bool,
}

/// Enriches one document: reads it, regenerates every marked block,
/// re-deriving the heading outline between passes so structure-sensitive
/// generators (currently just `toc`) see the up-to-date document. Runs
/// to a fixpoint (capped) rather than a hardcoded two passes, so it stays
/// correct if a future generator's output depends on another's.
fn enrich(raw: &str, path: &PathBuf, repo_root: &PathBuf) -> Result<String> {
    let registry = generators::build_registry();
    let (fm, yaml, mut body) = split_frontmatter(raw)
        .map_err(|e| Error::Frontmatter(format!("{}: {e}", path.display())))?;

    for pass in 0..MAX_PASSES {
        let ast = Ast::parse(&body);
        let ctx = Context {
            file_path: path,
            repo_root,
            frontmatter: &fm,
            ast: &ast,
        };
        let new_body = marker::process_markers(&body, &registry, &ctx)?;

        if new_body == body {
            return Ok(format!("---\n{yaml}\n---\n{new_body}"));
        }
        body = new_body;

        if pass == MAX_PASSES - 1 {
            return Err(Error::Unstable(format!(
                "{}: generated content did not stabilize after {MAX_PASSES} passes \u{2014} \
                 a generator's output is likely depending on something that keeps changing",
                path.display()
            )));
        }
    }
    unreachable!()
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let raw = match fs::read_to_string(&cli.path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("error: reading {}: {e}", cli.path.display());
            return ExitCode::FAILURE;
        }
    };

    let enriched = match enrich(&raw, &cli.path, &cli.repo_root) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::FAILURE;
        }
    };

    if cli.check {
        if enriched == raw {
            println!("{}: up to date", cli.path.display());
            ExitCode::SUCCESS
        } else {
            eprintln!("{}: would be changed by enrichment", cli.path.display());
            ExitCode::FAILURE
        }
    } else if cli.stdout {
        print!("{enriched}");
        ExitCode::SUCCESS
    } else {
        match fs::write(&cli.path, &enriched) {
            Ok(()) => {
                if enriched == raw {
                    println!("{}: already up to date", cli.path.display());
                } else {
                    println!("{}: enriched", cli.path.display());
                }
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("error: writing {}: {e}", cli.path.display());
                ExitCode::FAILURE
            }
        }
    }
}
