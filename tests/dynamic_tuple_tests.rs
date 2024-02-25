pub use easify_macros::dynamic_tuple;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = dynamic_tuple!(5, 3);
        assert_eq!(result, (5, 5, 5));
    }
}


