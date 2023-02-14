use std::convert::TryFrom;
use std::fmt;

pub enum CmdOption {
    Goto,
    Open,
}

// Reference: https://kerkour.com/rust-enum-to-string
impl fmt::Display for CmdOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CmdOption::Goto => write!(f, "goto"),
            CmdOption::Open => write!(f, "open"),
        }
    }
}

// Reference: https://stackoverflow.com/questions/28028854/how-do-i-match-enum-values-with-an-integer/57578431#57578431
impl TryFrom<usize> for CmdOption {
    type Error = ();
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            x if x == CmdOption::Goto as usize => Ok(CmdOption::Goto),
            x if x == CmdOption::Open as usize => Ok(CmdOption::Open),
            _ => Err(()),
        }
    }
}

impl CmdOption {
    pub fn to_string_vec() -> Vec<String> {
        vec![CmdOption::Goto.to_string(), CmdOption::Open.to_string()]
    }
}
