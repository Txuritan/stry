use {
    crate::Error,
    chrono::Local,
    log::{Level, Log, Metadata, Record},
    std::path::Path,
};

pub mod drains;

pub use self::drains::*;

struct Logger {
    level: Level,
    file_drain: FileDrain,
    stdout_drain: StdoutDrain,
}

impl Logger {
    fn new(level: Level, path: impl AsRef<Path>) -> Result<Self, Error> {
        Ok(Self {
            level,
            file_drain: FileDrain::new(Level::Trace, path)?,
            stdout_drain: StdoutDrain::new(level)?,
        })
    }

    fn log_format(record: &Record) -> String {
        let target = if !record.target().is_empty() {
            record.target()
        } else {
            record.module_path().unwrap_or_default()
        };

        format!(
            "{} {:<5} [{}] {}",
            Local::now().format("%Y-%m-%d %H:%M:%S,%3f"),
            record.level().to_string(),
            target,
            record.args(),
        )
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.file_drain.level() || metadata.level() <= self.stdout_drain.level()
    }

    fn log(&self, record: &Record) {
        let msg = Self::log_format(&record);

        if let Err(err) = self.file_drain.write(&msg) {
            eprintln!("Could not write to target: {}", err);
        }

        if record.level().le(&self.level) {
            if let Err(err) = self.stdout_drain.write(&msg) {
                eprintln!("Could not write to target: {}", err);
            }
        }
    }

    fn flush(&self) {
        if let Err(err) = self.file_drain.flush() {
            eprintln!("Could not flush drain: {}", err);
        }

        if let Err(err) = self.stdout_drain.flush() {
            eprintln!("Could not flush drain: {}", err);
        }
    }
}

pub fn init_with_level(level: Level, path: impl AsRef<Path>) -> Result<(), Error> {
    let logger = Logger::new(level, path)?;

    log::set_boxed_logger(Box::new(logger)).expect("Unable to set logger");
    log::set_max_level(Level::Trace.to_level_filter());

    Ok(())
}
