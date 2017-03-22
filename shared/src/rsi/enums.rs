use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RsiFlags {

}

// If you add any selectors.
// You're probably gonna need to fix the JSON loader so it can parse full StateId names.
// Sorry about that.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum RsiSelectors {

}

impl fmt::Display for RsiSelectors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}
