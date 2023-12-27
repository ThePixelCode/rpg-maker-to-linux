fn main() {
    let cli = <rpg2linux::args::Args as clap::Parser>::parse();
    let mut logger = match cli.stderr {
        true => rpg2linux::logger::Logger::new_std(cli.verbose),
        false => {
            let time_secs = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            rpg2linux::logger::Logger::new(format!("logs/log-{}", time_secs), cli.verbose)
        }
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
            let mut args = rpg2linux::parse_steam_args(args);
            let mut game_data = match rpg2linux::GameData::new(args.last().unwrap(), cli.sdk) {
                Ok(game_data) => game_data,
                Err(e) => logger.error(&format!(
                    "Error found when indexing game data, the error was {}",
                    e
                )),
            };
            match rpg2linux::port::prepare(&mut game_data, &mut logger) {
                Ok(_) => (),
                Err(e) => logger.error(&format!("{}", e)),
            }
            let game = args.pop().unwrap();
            args.push(format!(
                "{}",
                rpg2linux::get_cache_folder()
                    .map(|folder| folder.join(match cli.sdk {
                        true => format!(
                            "nwjs-sdk-{0}-linux-x64/nwjs-sdk-{0}-linux-x64/nw",
                            rpg2linux::NWJS_VERSION
                        ),
                        false => format!(
                            "nwjs-{0}-linux-x64/nwjs-{0}-linux-x64/nw",
                            rpg2linux::NWJS_VERSION
                        ),
                    }))
                    .unwrap_or_else(|e| logger.error(&format!("{}", e)))
                    .display()
            ));
            args.push(game);
            match rpg2linux::run::run_steam_game(args, &mut logger) {
                Ok(_) => (),
                Err(e) => logger.error(&format!("{}", e)),
            }
        }
        #[allow(unreachable_patterns)]
        _ => unimplemented!(),
    }
}
