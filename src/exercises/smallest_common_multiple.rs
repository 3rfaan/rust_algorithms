// Find the smallest common multiple of the provided parameters that can be evenly divided by both,
// as well as by all sequential numbers in the range between these parameters.

// The range will be an array of two numbers that will not necessarily be in numerical order.

// For example, if given 1 and 3, find the smallest common multiple of both 1 and 3 that is also
// evenly divisible by all numbers between 1 and 3. The answer here would be 6.

#[allow(dead_code)]
#[allow(unused_variables)]
fn smallest_commons(arr: [i32; 2]) -> i32 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(smallest_commons([1, 5]), 60);
    }

    #[test]
    fn test2() {
        assert_eq!(smallest_commons([5, 1]), 60);
    }

    #[test]
    fn test3() {
        assert_eq!(smallest_commons([2, 10]), 2520);
    }

    #[test]
    fn test4() {
        assert_eq!(smallest_commons([1, 13]), 360360);
    }

    #[test]
    fn test5() {
        assert_eq!(smallest_commons([23, 18]), 6056820);
    }
}
