mod json;
pub use json::print_codec;

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    #[test]
    fn test_print_codec() {
        let data = vec![1.0];
        let d = data
            .into_iter()
            .coalesce(|pre, cur| if pre < cur { Err((pre, cur)) } else { Ok(pre) });
        itertools::assert_equal(d, vec![1.0]);
    }
}
