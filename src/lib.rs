pub use easify_macros::dynamic_tuple;


pub fn add(left: usize, right: usize) -> (i32, i32, i32) {
    dynamic_tuple!(5, 3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(1, 5);
        assert_eq!(result, (5, 5, 5));
    }
}
