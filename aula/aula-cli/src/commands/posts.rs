//! Posts/feed subcommands.

use clap::Subcommand;

/// View and manage posts in the institution feed.
#[derive(Debug, Subcommand)]
pub enum PostsCommand {
    /// List recent posts.
    List {
        /// Maximum number of posts to show.
        #[arg(short = 'n', long, default_value = "20")]
        limit: u32,
        /// Filter by institution profile ID.
        #[arg(long)]
        institution: Option<u64>,
    },
    /// Show a single post by ID.
    Show {
        /// Post ID.
        post_id: u64,
    },
}

pub fn handle(cmd: &PostsCommand) {
    match cmd {
        PostsCommand::List { .. } => println!("posts list: not yet implemented"),
        PostsCommand::Show { .. } => println!("posts show: not yet implemented"),
    }
}
