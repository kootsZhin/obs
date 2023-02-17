use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about)]
pub struct ObsArg {
    #[clap(short, long, group = "action")]
    /// Goto vault directory   
    pub goto: Option<String>,
    #[clap(short, long, group = "action")]
    /// Open vault in Obsidian
    pub open: Option<String>,
}
