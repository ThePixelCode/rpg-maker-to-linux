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
    // run <PATH>
    #[command(about = "port and run a game")]
    Run { path: String },
    // steam-run %command%
    // this means:
    // steam-run reaper SteamLaunch AppId=123456 -- steam-launch-wrapper -- SteamLinuxRuntime_sniper/_v2-entry-point --verb=waitforexitandrun -- proton waitforexitandrun gamebin
    // or
    // steam-run reaper SteamLaunch AppId=123456 -- steam-launch-wrapper -- gamebin
    #[command(about = "port and run a game for steam")]
    SteamRun { args: Vec<String> },
    // port <PATH>
    #[command(about = "port a game")]
    Port { path: String },
}
