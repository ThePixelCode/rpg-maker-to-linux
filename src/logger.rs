pub enum Logger {
    FileLogger {
        file: std::fs::File,
        verbose_level: u8,
    },
    StdOutLogger {
        verbose_level: u8,
    },
}

impl Logger {
    pub fn new<P>(path: P, verbose_level: u8) -> Self
    where
        P: AsRef<std::path::Path>,
    {
        match std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
        {
            Ok(file) => Logger::FileLogger {
                file,
                verbose_level,
            },
            Err(_) => {
                eprintln!("logfile failed, defaulting to stderr");
                Logger::StdOutLogger { verbose_level }
            }
        }
    }

    pub fn new_std(verbose_level: u8) -> Self {
        eprintln!("stdout only enable");
        Logger::StdOutLogger { verbose_level }
    }

    fn get_log_level(&self) -> &u8 {
        match self {
            Logger::FileLogger {
                file: _,
                verbose_level,
            } => verbose_level,
            Logger::StdOutLogger { verbose_level } => verbose_level,
        }
    }

    pub fn write(&mut self, text: &str, verbose_level: u8) {
        if self.get_log_level() < &verbose_level {
            return;
        }
        match self {
            Logger::FileLogger {
                ref mut file,
                verbose_level: _,
            } => match std::io::Write::write_all(file, format!("{}\n", text).as_bytes())
                .map(|_| file.sync_all())
            {
                Ok(e) => match e {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("logfile write failed with error: {}", e);
                        eprintln!("trying to write: {}", text)
                    }
                },
                Err(e) => {
                    eprintln!("logfile write failed with error: {}", e);
                    eprintln!("trying to write: {}", text)
                }
            },
            Logger::StdOutLogger { verbose_level: _ } => eprintln!("{}", text),
        }
    }

    pub fn log(&mut self, text: &str) {
        self.write(text, crate::DEBUG_INFO);
    }

    pub fn warn(&mut self, text: &str) {
        self.write(text, crate::DEBUG_WARN);
    }

    pub fn error(&mut self, text: &str) -> ! {
        self.write(text, crate::DEBUG_ERROR);
        std::process::exit(1)
    }
}
