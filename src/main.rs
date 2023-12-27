fn main() {
    let cli = <rpg2linux::args::Args as clap::Parser>::parse();
    let mut logger = match cli.stderr {
        true => rpg2linux::logger::Logger::new_std(cli.verbose),
        false => rpg2linux::logger::Logger::new("log", cli.verbose),
    };
    match cli.command {
        rpg2linux::args::Commands::Run { path } => {
            let mut game_data = match rpg2linux::GameData::new(path, cli.sdk) {
                Ok(game_data) => game_data,
                Err(e) => logger.error(&format!(
                    "Error found when indexing game data, the error was {}",
                    e
                )),
            };
            match rpg2linux::port::port(&mut game_data, &mut logger) {
                Ok(_) => (),
                Err(e) => logger.error(&format!("{}", e)),
            }
            match rpg2linux::run::run_game(&mut game_data, &mut logger, Vec::new()) {
                Ok(_) => (),
                Err(e) => logger.error(&format!("{}", e)),
            }
        }
        rpg2linux::args::Commands::Port { path } => {
            let mut game_data = match rpg2linux::GameData::new(path, cli.sdk) {
                Ok(game_data) => game_data,
                Err(e) => logger.error(&format!(
                    "Error found when indexing game data, the error was {}",
                    e
                )),
            };
            match rpg2linux::port::port(&mut game_data, &mut logger) {
                Ok(_) => (),
                Err(e) => logger.error(&format!("{}", e)),
            }
        }
        rpg2linux::args::Commands::SteamRun { args } => {
            let args = rpg2linux::parse_steam_args(args);
            let mut game_data = match rpg2linux::GameData::new(args.last().unwrap(), cli.sdk) {
                Ok(game_data) => game_data,
                Err(e) => logger.error(&format!(
                    "Error found when indexing game data, the error was {}",
                    e
                )),
            };
            match rpg2linux::port::port(&mut game_data, &mut logger) {
                Ok(_) => (),
                Err(e) => logger.error(&format!("{}", e)),
            }
            match rpg2linux::run::run_steam_game(args, &mut logger) {
                Ok(_) => (),
                Err(e) => logger.error(&format!("{}", e)),
            }
        }
        #[allow(unreachable_patterns)]
        _ => unimplemented!(),
    }
}
