use crate::UI;
use clap::Parser;

/// A full-terminal interactive shell
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Options {}

pub fn run() -> anyhow::Result<()> {
    let _ = Options::parse();
    UI::new()?.run()
}
