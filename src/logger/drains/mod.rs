use log::{Level, Record};

pub mod file;
pub mod stdout;

pub use self::{file::FileDrain, stdout::StdoutDrain};

pub trait Drain {
    fn level(&self) -> Level;
    fn ignore(&self, record: &Record) -> bool;
    fn write(&self, string: &str) -> std::io::Result<()>;
    fn flush(&self) -> std::io::Result<()>;
}
