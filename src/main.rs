mod ai;
mod cache;
mod commands;
mod error;
mod git;
mod ui;

use ai::{Backend, BackendConfig};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "git-ai", about = "AI-powered git commands for the lazy")]
struct Cli {
    /// AI backend to use
    #[arg(long, global = true, env = "GIT_AI_BACKEND")]
    backend: Option<Backend>,

    /// AI model to use
    #[arg(long, global = true, env = "GIT_AI_MODEL")]
    model: Option<String>,

    /// Max budget in USD (claude only)
    #[arg(long, global = true, env = "GIT_AI_BUDGET", default_value = "0.50")]
    budget: f64,

    /// Enable debug output
    #[arg(long, global = true, env = "GIT_AI_DEBUG")]
    debug: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Generate atomic commits from changes
    Commit(commands::commit::CommitArgs),

    /// AI code review of staged/branch changes
    Review(commands::review::ReviewArgs),

    /// Explain recent commits
    Explain(commands::explain::ExplainArgs),

    /// Suggest conventional branch name
    Branch(commands::branch::BranchArgs),

    /// Generate PR title + body from branch commits
    Pr(commands::pr::PrArgs),

    /// Freeform Q&A about the repo
    Ask(commands::ask::AskArgs),

    /// Manage the commit plan cache
    Cache(commands::cache::CacheArgs),
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let backend_config = BackendConfig {
        backend: cli.backend,
        model: cli.model,
        budget: cli.budget,
        debug: cli.debug,
    };

    let result = match &cli.command {
        Command::Commit(args) => commands::commit::run(args, &backend_config).await,
        Command::Review(args) => commands::review::run(args, &backend_config).await,
        Command::Explain(args) => commands::explain::run(args, &backend_config).await,
        Command::Branch(args) => commands::branch::run(args, &backend_config).await,
        Command::Pr(args) => commands::pr::run(args, &backend_config).await,
        Command::Ask(args) => commands::ask::run(args, &backend_config).await,
        Command::Cache(args) => commands::cache::run(args),
    };

    if let Err(e) = result {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
