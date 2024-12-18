use anyhow::anyhow;
use clap::{Parser, Subcommand};

use crate::gh;

pub async fn figure() -> anyhow::Result<String> {
    let cli = Cli::parse();

    let result: anyhow::Result<String> = match cli.command {
        Some(Commands::Get { token, anonymous }) => {
            gh::get_all_contributions(token, anonymous.unwrap_or_default()).await
        }
        Some(Commands::Init {}) => Err(anyhow!("Not Implemented".to_string())),
        Some(Commands::Markdown) => Ok(clap_markdown::help_markdown::<Cli>()),
        None => Ok("try me --help for information on how to use me".to_string()),
    };

    match result {
        Ok(o) => Ok(o),
        Err(err) => Err(err),
    }
}

/// me cli
#[derive(Parser, Debug)]
#[command(author, version, about, long_about, name = "me")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
#[command(rename_all = "snake_case")]
enum Commands {
    /// [STABLE] print markdown doc of me to std out
    Markdown,

    /// [STABLE] creates an example config
    Init {},

    /// [STABLE] get all stats for a user
    Get {
        #[arg(short, long, env = "GITHUB_TOKEN")]
        token: String,

        #[arg(short, long, env = "ME_ANONYMOUS")]
        anonymous: Option<Vec<String>>,
    },
}
