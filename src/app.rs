use crate::cli::Arguments;

pub struct App {
    args: Arguments,
}

impl App {
    pub fn new(args: Arguments) -> Self {
        Self { args }
    }

    pub fn run(&self) {
        println!("Running with arguments: {:?}", self.args);
    }
}
