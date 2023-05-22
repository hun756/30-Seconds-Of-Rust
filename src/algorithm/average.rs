/// Returns the average of two or more numbers.
///
/// This function takes an iterator of numbers and returns the arithmetic mean of them.
/// If the iterator is empty, it returns zero.
///
/// # Examples
///
/// ```
/// use average::average;
///
/// assert_eq!(average(vec![1.0, 2.0, 3.0, 4.0]), 2.5);
/// assert_eq!(average([1.0, 2.0, 3.0]), 2.0);
/// assert_eq!(average([5.0]), 5.0);
/// assert_eq!(average(std::iter::empty::<f64>()), 0.0);
/// ```
///
/// # Panics
///
/// This function does not panic.
///
/// # Errors
///
/// This function does not return any errors.
///
/// # Safety
///
/// This function does not use any unsafe code.
pub fn average<T: IntoIterator<Item = f64>>(nums: T) -> f64 {
    let (sum, count) = nums
        .into_iter()
        .fold((0.0, 0), |(acc, cnt), val| (acc + val, cnt + 1));

    if count == 0 {
        0.0
    } else {
        sum / count as f64
    }
}
