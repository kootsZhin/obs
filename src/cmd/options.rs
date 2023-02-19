use std::convert::TryFrom;
use std::fmt;

pub enum ObsOption {
    Goto,
    Open,
    Backup,
    // for entering selection mode
    Select,
}

// Reference: https://kerkour.com/rust-enum-to-string
impl fmt::Display for ObsOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ObsOption::Goto => write!(f, "goto"),
            ObsOption::Open => write!(f, "open"),
            ObsOption::Backup => write!(f, "backup"),

            ObsOption::Select => write!(f, "select"),
        }
    }
}

// Reference: https://stackoverflow.com/questions/28028854/how-do-i-match-enum-values-with-an-integer/57578431#57578431
impl TryFrom<usize> for ObsOption {
    type Error = ();
    fn try_from(v: usize) -> Result<Self, ()> {
        match v {
            x if x == ObsOption::Goto as usize => Ok(ObsOption::Goto),
            x if x == ObsOption::Open as usize => Ok(ObsOption::Open),
            x if x == ObsOption::Backup as usize => Ok(ObsOption::Backup),
            _ => Err(()),
        }
    }
}

impl ObsOption {
    pub fn to_string_vec() -> Vec<String> {
        vec![
            ObsOption::Goto.to_string(),
            ObsOption::Open.to_string(),
            ObsOption::Backup.to_string(),
        ]
    }
}
