use super::RsiSelectors;
use super::super::helpers::string::concat_vec;

/// Turns a name and slice of selectors into a full state name like `wrench+m`
pub fn full_state_name(name: &str, select: &[RsiSelectors]) -> String {
    let mut name = name.to_string();
    if select.len() > 0 {
        name += &("+".to_string() + &concat_vec(&select, "+"));
    }
    name
}
