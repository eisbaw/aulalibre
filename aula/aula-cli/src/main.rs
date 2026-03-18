use clap::Parser;

/// CLI tool for interacting with the Aula school platform.
#[derive(Parser)]
#[command(name = "aula", version, about)]
struct Cli {
    /// Print version information and exit.
    #[arg(long)]
    info: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.info {
        println!("aula-cli v{}", env!("CARGO_PKG_VERSION"));
        println!("aula-api v{}", aula_api::version());
    } else {
        println!(
            "aula-cli v{} — no command given, try --help",
            env!("CARGO_PKG_VERSION")
        );
    }
}
