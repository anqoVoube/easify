use easify_macros::let_unpack;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let unpacking = vec![5, 3, 7];
        let_unpack!(a, *b, c = unpacking);
        assert_eq!(a, 5);
        assert_eq!(b, &vec![3]);
        assert_eq!(c, 7);
    }
    #[test]
    fn it_works_2() {
        let unpacking = vec![5, 6, 3, 7];
        let_unpack!(*a, b, c = unpacking);
        assert_eq!(a, &vec![5, 6]);
        assert_eq!(b, 3);
        assert_eq!(c, 7);
    }
}

