use {
    crate::logger::drains::Drain,
    log::{Level, Record},
    std::{
        fs::{File, OpenOptions},
        io::{self, Write},
        path::Path,
        sync::Mutex,
    },
};

pub struct FileDrain {
    level: Level,
    file: Mutex<File>,
}

impl FileDrain {
    pub fn new(level: Level, path: impl AsRef<Path>) -> io::Result<Self> {
        let file = Mutex::new(
            OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(path)?,
        );

        Ok(Self { level, file })
    }
}

impl Drain for FileDrain {
    fn level(&self) -> Level {
        self.level
    }

    fn ignore(&self, _record: &Record) -> bool {
        false
    }

    fn write(&self, string: &str) -> io::Result<()> {
        match self.file.lock() {
            Err(_) => Err(io::Error::new(io::ErrorKind::Other, "mutex poison error")),
            Ok(mut lock) => writeln!(&mut lock, "{}", string),
        }
    }

    fn flush(&self) -> io::Result<()> {
        match self.file.lock() {
            Err(_) => Err(io::Error::new(io::ErrorKind::Other, "mutex poison error")),
            Ok(mut lock) => lock.flush(),
        }
    }
}
