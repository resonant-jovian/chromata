use clap::{Parser, Subcommand};

mod check;
mod clean;
pub mod codegen;
pub mod fetch;
pub mod generate;

#[derive(Parser)]
#[command(name = "xtask", about = "Chromata code generation tasks")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Fetch upstream theme data for a collection
    Fetch {
        /// Collection: base16, base24, vim, emacs, or all
        collection: String,
        /// Re-download even if data already exists
        #[arg(long)]
        force: bool,
    },
    /// Generate Rust source files from fetched data
    Generate {
        /// Collection: base16, base24, vim, emacs, or all
        collection: String,
    },
    /// Remove fetched data and/or generated files
    Clean {
        /// Remove only the download cache (data/)
        #[arg(long)]
        cache: bool,
        /// Remove only generated .rs files
        #[arg(long)]
        generated: bool,
    },
    /// Verify generated files match committed files
    Check,
    /// Run the full CI pipeline: fetch -> generate -> clippy -> test
    Ci,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Fetch { collection, force } => {
            fetch::fetch(&collection, force)?;
            // Clean up tinted-theming temp dir after all fetches
            fetch::cleanup_tinted_theming();
            Ok(())
        }
        Command::Generate { collection } => generate::generate(&collection),
        Command::Clean { cache, generated } => clean::clean(cache, generated),
        Command::Check => check::check(),
        Command::Ci => check::ci(),
    }
}
