use {
    crate::logger::drains::Drain,
    log::{Level, Record},
    std::io::{self, Write},
};

pub struct StdoutDrain {
    level: Level,
}

impl StdoutDrain {
    pub fn new(level: Level) -> io::Result<Self> {
        Ok(Self { level })
    }
}

impl Drain for StdoutDrain {
    fn level(&self) -> Level {
        self.level
    }

    fn ignore(&self, _record: &Record) -> bool {
        false
    }

    fn write(&self, string: &str) -> io::Result<()> {
        writeln!(&mut io::stdout(), "{}", string)
    }

    fn flush(&self) -> io::Result<()> {
        io::stdout().flush()
    }
}
