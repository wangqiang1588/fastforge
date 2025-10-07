use clap::{Parser, Subcommand};

mod cli;
use cli::{AnalyzeArgs, BuildArgs, PackageArgs, PublishArgs, UpgradeArgs};

#[derive(Parser)]
#[command(name = "fastforge")]
#[command(about = "Package and publish your apps with ease.")]
#[command(version = "0.7.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Analyze your project")]
    Analyze(AnalyzeArgs),
    #[command(about = "Build your project")]
    Build(BuildArgs),
    #[command(about = "Package your project")]
    Package(PackageArgs),
    #[command(about = "Publish your project")]
    Publish(PublishArgs),
    #[command(about = "Update Fastforge to the latest version")]
    Upgrade(UpgradeArgs),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Analyze(args) => {
            cli::analyze::execute(args).await?;
        }
        Commands::Build(args) => {
            cli::build::execute(args).await?;
        }
        Commands::Package(args) => {
            cli::package::execute(args).await?;
        }
        Commands::Publish(args) => {
            cli::publish::execute(args).await?;
        }
        Commands::Upgrade(args) => {
            cli::upgrade::execute(args).await?;
        }
    }

    Ok(())
}
