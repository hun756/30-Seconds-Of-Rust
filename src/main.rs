mod algorithm;

#[cfg(test)]
mod tests {
    use crate::algorithm::{
        factorial::Factorial, 
        all::all
    };

    #[test]
    fn test_factorial_u128() {
        // Test some cases and compare the results with the expected values
        assert_eq!(1, 0u128.factorial());
        assert_eq!(120, 5u128.factorial());
        assert_eq!(3628800, 10u128.factorial());
    }

    #[test]
    fn test_all() {
        assert_eq!(all(&[1, 2, 3, 4], |x| x > 0), true);
        assert_eq!(all(&[true, false, true], |x| x), false);
        assert_eq!(all(&["a", "b", "c"], |x| x.len() == 1), true);
    }
}

fn main() { }