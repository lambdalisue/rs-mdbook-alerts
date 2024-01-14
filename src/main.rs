use clap::{Parser, Subcommand};
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor};
use semver::{Version, VersionReq};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Supports { renderer: String },
}

fn main() {
    let args = Args::parse();

    let preprocessor = mdbook_alerts::Preprocessor;
    if let Some(Commands::Supports { renderer }) = args.command {
        handle_supports(&preprocessor, &renderer);
    } else if let Err(e) = handle_preprocessing(&preprocessor) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn handle_supports(pre: &dyn Preprocessor, renderer: &str) {
    if pre.supports_renderer(renderer) {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(std::io::stdin())?;

    let book_version = Version::parse(&ctx.mdbook_version)?;
    let version_req = VersionReq::parse(mdbook::MDBOOK_VERSION)?;
    if !version_req.matches(&book_version) {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, but we're being called from version {}",
            pre.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version,
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(std::io::stdout(), &processed_book)?;

    Ok(())
}
