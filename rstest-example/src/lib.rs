pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    use mockall::predicate::*;
    use mockall::*;

    use super::*;
    #[automock]
    trait AnyTrait {
        fn any_method_name(&self, x: u32) -> u32;
    }

    fn takes_any_trait_functions(x: &dyn AnyTrait, v: u32) -> u32 {
        x.any_method_name(v)
    }

    #[rstest::rstest]
    fn test_mockall() {
        let mut mock = MockAnyTrait::new();
        mock.expect_any_method_name().returning(|x| x + 1);
        assert_eq!(10, takes_any_trait_functions(&mock, 9));
    }

    #[rstest::fixture]
    fn any_fixture_name() -> i32 {
        24
    }

    #[rstest::rstest]
    fn any_function_name(any_fixture_name: i32) {
        assert_eq!(any_fixture_name * 2, 48);
    }

    #[rstest::rstest]
    #[case(10, 0, 10)]
    #[case(100, 5, 95)]
    fn test_sub(#[case] a: i32, #[case] b: i32, #[case] expected: i32) {
        assert_eq!(sub(a, b), expected);
    }
}
