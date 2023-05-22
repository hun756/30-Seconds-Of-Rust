/// Performs a left fold operation on an iterator with a given initial value and a closure.
///
/// This function is similar to [`std::iter::Iterator::fold`], but it takes the closure as the last argument
/// and allows using the function name as a higher-order function.
///
/// # Examples
///
/// ```
/// use std::num::Int;
///
/// // compute the sum of a vector of numbers
/// let v = vec![1, 2, 3, 4, 5];
/// let sum = accumulate(v.into_iter(), 0, |a, b| a + b);
/// assert_eq!(sum, 15);
///
/// // compute the product of a vector of numbers
/// let v = vec![1, 2, 3, 4, 5];
/// let product = accumulate(v.into_iter(), 1, |a, b| a * b);
/// assert_eq!(product, 120);
///
/// // compute the GCD of a vector of numbers
/// let v = vec![10, 2, 4, 6];
/// let gcd = accumulate(v.into_iter(), 0, |a, b| a.gcd(b));
/// assert_eq!(gcd, 2);
/// ```
///
/// [`std::iter::Iterator::fold`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold
pub fn accumulate<I, T, F>(iter: I, init: T, f: F) -> T
where
    I: Iterator<Item = T>,
    F: Fn(T, T) -> T,
{
    // use fold to apply the closure to each element and the accumulator
    iter.fold(init, f)
}
