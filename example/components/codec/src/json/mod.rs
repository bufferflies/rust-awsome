mod json;
pub use json::print_codec;

#[cfg(test)]
mod tests{
    #[test]
    fn test_print_codec(){
        super::print_codec();
    }
}
