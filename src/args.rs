use clap::Parser;

use crate::ObsOption;

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

impl ObsArg {
    pub fn into_opt(&self) -> ObsOption {
        if self.goto.is_some() {
            ObsOption::Goto
        } else if self.open.is_some() {
            ObsOption::Open
        } else {
            ObsOption::Select
        }
    }
}
