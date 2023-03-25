// Given a positive integer num, return the sum of all odd Fibonacci numbers that are less than or equal to num.

// The first two numbers in the Fibonacci sequence are 0 and 1. Every additional number in the sequence is the sum of the two previous numbers.
// The first seven numbers of the Fibonacci sequence are 0, 1, 1, 2, 3, 5 and 8.

// For example, sumFibs(10) should return 10 because all odd Fibonacci numbers less than or equal to 10 are 1, 1, 3, and 5.

#[allow(dead_code)]
#[allow(unused_variables)]
fn sum_fibs(num: i64) -> i64 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(sum_fibs(1000), 1785);
    }

    #[test]
    fn test2() {
        assert_eq!(sum_fibs(4000000), 4613732);
    }

    #[test]
    fn test3() {
        assert_eq!(sum_fibs(4), 5);
    }

    #[test]
    fn test4() {
        assert_eq!(sum_fibs(75024), 60696);
    }

    #[test]
    fn test5() {
        assert_eq!(sum_fibs(75025), 135721);
    }
}
