// A prime number is a whole number greater than 1 with exactly two divisors: 1 and itself.
// For example, 2 is a prime number because it is only divisible by 1 and 2.
// In contrast, 4 is not prime since it is divisible by 1, 2 and 4.

// Rewrite sumPrimes so it returns the sum of all prime numbers that are less than or equal to num.

#[allow(dead_code)]
#[allow(unused_variables)]
fn sum_primes(num: i64) -> i64 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(sum_primes(10), 17);
    }

    #[test]
    fn test2() {
        assert_eq!(sum_primes(977), 73156);
    }
}
