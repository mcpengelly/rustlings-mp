// iterators4.rs
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a hint.

pub fn factorial_imperative_style(num: u64) -> u64 {
    let mut count: u64 = num;
    let mut sum: u64 = 1;

    while count > 0 {
        sum *= count;
        count -= 1;
    }
    sum
}

pub fn factorial_recursive_style(num: u64) -> u64 {
    if num == 0 || num == 1 {
        1
    } else {
        num * factorial_recursive_style(num - 1)
    }
}

// ex: 5! = 5x4x3x2x1 = 12
// in this case starting from bottom and building up
pub fn factorial(num: u64) -> u64 {
    (0..num + 1).into_iter().fold(
        1,
        |accum, next| {
            if next == 0 {
                accum * 1
            } else {
                accum * next
            }
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn factorial_of_0() {
    //     assert_eq!(1, factorial(0));
    // }

    // #[test]
    // fn factorial_of_1() {
    //     assert_eq!(1, factorial(1));
    // }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
