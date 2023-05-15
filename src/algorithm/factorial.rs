/// A trait for types that can be used for factorial calculation.
pub trait Factorial {
    /// Calculates the factorial of self and returns the same type.
    ///
    /// # Examples
    ///
    /// ```
    /// use factorial::Factorial;
    ///
    /// let five = 5u128;
    /// assert_eq!(120, five.factorial());
    /// ```
    fn factorial(self) -> Self;
}

/// An implementation of the Factorial trait for unsigned integers.
impl Factorial for u128 {
    fn factorial(self) -> Self {
        if self == 0 {
            1 // Base case: 0! = 1
        } else {
            // Recursive case: n! = n * (n - 1)!
            // Use the factorial method on self - 1, which is also an u128
            self * (self - 1).factorial()
        }
    }
}
