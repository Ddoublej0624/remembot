use simplelog::*;
use anyhow;

pub fn init_logging() -> anyhow::Result<()> {
    //TODO: Make this configurable, maybe use clap or something to pass cli arguments
    let _ = SimpleLogger::init(LevelFilter::Trace, Config::default());
    Ok(())
}
