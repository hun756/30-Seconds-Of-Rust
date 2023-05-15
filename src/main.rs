mod algorithm;

#[cfg(test)]
mod tests {
    use crate::algorithm::factorial::Factorial;
    
    #[test]
    fn test_factorial_u128() {
        // Test some cases and compare the results with the expected values
        assert_eq!(1, 0u128.factorial());
        assert_eq!(120, 5u128.factorial());
        assert_eq!(3628800, 10u128.factorial());
    }
}

fn main() { }