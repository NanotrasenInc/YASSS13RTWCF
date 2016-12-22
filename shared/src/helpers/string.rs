/// Joins a slice of `ToString` into a single String with separator.
pub fn concat_vec<T:ToString>(slice: &[T], separator: &str) -> String {
    slice.iter().fold("".to_string(),
                      |acc, el| if acc.len() > 0 { acc + separator + &el.to_string() } else { el.to_string() }
                     )
}
