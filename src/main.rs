use anyhow::{Context, Result};
use structopt::StructOpt;

/// Searches for a pattern on a given file path and displays the lines containing
#[derive(StructOpt)]
struct Cli {
    /// String pattern to search
    pattern: String,
    /// File path where pattern will be searched
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read file `{:?}`", &args.path))?;

    grrs::find_matches(&content, &args.pattern, &mut std::io::stdout())?;

    Ok(())
}
