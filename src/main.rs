use clap::Parser;
use jeopardytool::ToolResult;

#[derive(Parser, Debug)]
#[clap(version, author, about)]
#[clap(propagate_version = true)]
enum CLI {
    /// Show the available categories
    Show(Show),
    /// Create a new game
    Create,
    /// Convert a old category to the new format
    Convert,
}

#[derive(Parser, Debug)]
struct Show {
    /// A prefix to select only a subset of games to use in analyzing
    #[clap(long, short)]
    prefix: Option<String>,
}

fn main() -> ToolResult<()> {
    let cli = CLI::parse();
    match cli {
        CLI::Show(show) => jeopardytool::show(show.prefix),
        CLI::Create => todo!("subcommand create"),
        CLI::Convert => todo!("subcommand convert"),
    }
}
