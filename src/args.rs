use clap::{Arg, Command, Parser};

#[derive(Parser, Debug)]
#[clap(version, about)]
pub struct ObsArg {
    #[clap(short, long)]
    /// Goto vault directory   
    goto: Option<String>,
    #[clap(short, long)]
    /// Open vault in Obsidian
    open: Option<String>,
}
