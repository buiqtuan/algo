use std::fmt::Display;

pub struct StderrLogger;

pub trait Logger {
    fn log(&self, verbosity: u8, message: impl Display);
}

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: impl Display) {
        if verbosity > 0 {
            eprintln!("{}", message);
        }
    }
}

pub fn do_things(logger: &impl Logger) {
    logger.log(1, "FYI");
    logger.log(2, "Proceeding...");
    logger.log(3, "Success...");
}

pub struct Verbosity {
    pub max_verbosity: u8,
    pub inner: StderrLogger
}

impl Logger for Verbosity {
    fn log(&self, verbosity: u8, message: impl Display) {
        if verbosity <= self.max_verbosity {
            self.inner.log(verbosity, message);
        }
    }
}

trait Animal {
    fn leg_count(&self) -> u32;
}

trait Pet : Animal {
    fn owner(&self) -> String;
}

struct Dog {
    name: String,
    owner: String
}

impl Animal for Dog {
    fn leg_count(&self) -> u32 {
        4
    }
}

impl Pet for Dog {
    fn owner(&self) -> String {
        self.owner.clone()
    }
}