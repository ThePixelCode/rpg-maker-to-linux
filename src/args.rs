#[derive(clap::Parser)]
#[command(name = "rpg2linux",author,version,about,long_about=None)]
pub struct Args {
    #[arg(short, long, help = "verbosity when logging to .cache/rpg2linux/logs/log-[date] or stderr if that fails", action = clap::ArgAction::Count)]
    pub verbose: u8,
    #[command(subcommand)]
    pub command: Commands,
    #[arg(short, long, help = "use nwjs with sdk")]
    pub sdk: bool,
    #[arg(long, help = "use stderr instead of log file")]
    pub stderr: bool,
}

#[derive(clap::Subcommand)]
pub enum Commands {
    #[command(about = "port and run a game")]
    Run { path: String },
    #[command(about = "port and run a game for steam")]
    SteamRun { args: Vec<String> },
    #[command(about = "port a game")]
    Port { path: String },
}
