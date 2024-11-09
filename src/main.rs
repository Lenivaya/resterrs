use anyhow::Result;
use clap::Parser;
use cli::Arguments;
use resterrs::app::App;
use resterrs::cli;
use resterrs::config::Config;

fn main() -> Result<()> {
    let args = Arguments::parse();
    let config = Config::new(&args);

    let app = App::new(args, config);
    app.run()
}
