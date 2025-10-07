use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct BuildArgs {
    #[arg(short, long = "target")]
    pub target: Option<String>,
}

pub async fn execute(args: &BuildArgs) -> Result<()> {
    log::info!("Executing build command");
    Ok(())
}
