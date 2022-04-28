#[cfg(test)]
mod numbers {
    #[test]
    fn int() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn float() {
        assert_eq!(2.0 + 2.0, 4.0)
    }

    #[test]
    fn float_unequal() {
        assert_eq!(1.0 / 3.0, 0.3333333333333333)
    }

    #[test]
    fn float_almost_equal() {
        assert!( 0.001 > (1.0 / 3.0 - 0.3333333333333333) )
    }

}
