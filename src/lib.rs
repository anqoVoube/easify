pub use easify_macros::{dynamic_tuple, let_unpack, unpack_split};


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(dynamic_tuple!(5, 3), (5, 5, 5));
        let unpacking = vec![5, 5, 3];
        let_unpack!(a, *b = unpacking);
        assert_eq!(a, 5);
        assert_eq!(b, &vec![5, 3]);
        let hello_text = "hello,bye";
        assert_eq!(("hello", "bye"), unpack_split!(hello_text, ",", 2));
    }
}
