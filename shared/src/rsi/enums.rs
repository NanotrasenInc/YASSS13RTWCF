use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RsiFlags {

}

#[derive(Clone, Debug)]
pub enum RsiSelectors {

}

impl fmt::Display for RsiSelectors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}
