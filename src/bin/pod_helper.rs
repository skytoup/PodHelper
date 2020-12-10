use env_logger::Target;
use pod_helper::{cmds::check_file, error::Result, opts::pod_helper::Command};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "pod_helper")]
pub struct Opt {
    /// sub cmd
    #[structopt(subcommand)]
    command: Command,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();

    let mut log_builder = env_logger::Builder::from_default_env();
    log_builder.target(Target::Stdout).init();

    match opt.command {
        Command::Check { opt } => {
            check_file(opt).await?;
        }
    }

    Ok(())
}
