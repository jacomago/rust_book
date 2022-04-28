#[cfg(test)]
mod primitives {
    #[test]
    fn signed_integers() {
        assert_eq!(2 + 2i32 + -3, 1);
    }

    #[test]
    fn floating_point() {
        assert_eq!(2.0 + 2.0, 4.0)
    }

    #[test]
    fn float_unequal() {
        assert_eq!(1.0 / 3.0, 0.3333333333333333)
    }

    #[test]
    fn float_almost_equal() {
        assert!(0.001 > (1.0 / 3.0 - 0.3333333333333333))
    }

    #[test]
    fn character() {
        assert_eq!('a'.len_utf8(), 1)
    }

    #[test]
    fn boolean() {
        assert_ne!(true, false)
    }
}

#[cfg(test)]
mod variables {

    #[test]
    fn assign() {
        let health = 5;
        assert_eq!(health, 5);
    }

    #[test]
    #[allow(unused_assignments)]
    fn mutability() {
        let mut health = 5;
        health = 6;
        assert_eq!(health, 6);
    }

    #[test]
    fn variable_type() {
        // Won't work on all platforms as type names change
        fn type_of<T>(_: &T) -> &'static str {
            return std::any::type_name::<T>();
        }
        let health = 5;
        assert_eq!(type_of(&health), "i32");
    }

    #[test]
    fn multi_assign() {
        let health = 5;
        let new_health = health;
        assert_eq!(health, new_health);
        assert_eq!(new_health, 5);
    }
}
