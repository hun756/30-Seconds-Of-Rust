mod algorithm;

#[cfg(test)]
mod tests {
    use crate::algorithm::{
        all::all, 
        average::average, 
        factorial::Factorial
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

    #[test]
    fn test_average() {
        assert_eq!(average(vec![1.0, 2.0, 3.0, 4.0]), 2.5);
        assert_eq!(average([1.0, 2.0, 3.0]), 2.0);
        assert_eq!(average([5.0]), 5.0);
        assert_eq!(average(std::iter::empty::<f64>()), 0.0);
    }
}

fn main() {}
