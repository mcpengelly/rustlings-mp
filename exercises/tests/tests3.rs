// tests3.rs
// This test isn't testing our function -- make it do that in such a way that
// the test passes. Then write a second test that tests whether we get the result
// we expect to get when we call `is_even(5)`.
// Execute `rustlings hint tests3` or use the `hint` watch subcommand for a hint.

pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        let some_num = 4;
        assert!(is_even(some_num));
    }

    #[test]
    fn is_false_when_odd() {
        let some_num = 5;
        assert!(!is_even(some_num));
    }
}
