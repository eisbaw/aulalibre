//! Gallery/media subcommands.

use clap::Subcommand;

/// Browse and download gallery media (photos, videos).
#[derive(Debug, Subcommand)]
pub enum GalleryCommand {
    /// List gallery albums.
    List {
        /// Filter by institution profile ID.
        #[arg(long)]
        institution: Option<u64>,
    },
    /// Show album contents.
    Show {
        /// Album ID.
        album_id: u64,
    },
    /// Download media from an album.
    Download {
        /// Album ID.
        album_id: u64,
        /// Output directory (defaults to current directory).
        #[arg(short, long)]
        output: Option<String>,
    },
}

pub fn handle(cmd: &GalleryCommand) {
    match cmd {
        GalleryCommand::List { .. } => println!("gallery list: not yet implemented"),
        GalleryCommand::Show { .. } => println!("gallery show: not yet implemented"),
        GalleryCommand::Download { .. } => println!("gallery download: not yet implemented"),
    }
}
