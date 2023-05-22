mod algorithm;

#[cfg(test)]
mod tests {
    use crate::algorithm::{
        accumulate::accumulate,
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

    #[test]
    fn test_accumulate_sum() {
        let v = vec![1, 2, 3, 4, 5];
        let sum = accumulate(v.into_iter(), 0, |a, b| a + b);
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_accumulate_product() {
        let v = vec![1, 2, 3, 4, 5];
        let product = accumulate(v.into_iter(), 1, |a, b| a * b);
        assert_eq!(product, 120);
    }

    #[test]
    fn test_gcd() {
        let v = vec![10, 2, 4, 6];
        let gcd = accumulate(v.into_iter(), 0, |a, b| helper::gcd(a, b));
        assert_eq!(gcd, 2);
    }

    #[test]
    #[should_panic]
    fn test_empty() {
        let v: Vec<i32> = vec![];
        let _result = accumulate(v.into_iter(), 0, |a, b| a + b);
        let v: Vec<i32> = vec![];
        let _result = accumulate(v.into_iter(), None.unwrap(), |a, b| a + b);

    }

    mod helper {
        pub fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }
    }
}

fn main() {}
