/// Returns true if the provided predicate function returns true for all elements in a collection, false otherwise.
///
/// This function takes a slice of any type and a predicate function that operates on the elements of the slice.
/// It uses the [`Iterator::all`] method to test if all elements in the slice satisfy the predicate function.
///
/// # Examples
///
/// ```no_run
/// use crate::all;
///
/// // Test if all numbers in the slice are positive
/// let numbers = [1, 2, 3, 4];
/// assert_eq!(all(&numbers, |x| x > 0), true);
///
/// // Test if all booleans in the slice are true
/// let booleans = [true, false, true];
/// assert_eq!(all(&booleans, |x| x), false);
///
/// // Test if all strings in the slice have length 1
/// let strings = ["a", "b", "c"];
/// assert_eq!(all(&strings, |x| x.len() == 1), true);
/// ```
///
/// [`Iterator::all`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.all
pub fn all<T, F>(arr: &[T], mut func: F) -> bool
where
    T: Clone,
    F: FnMut(T) -> bool,
{
    arr.iter().cloned().all(func)
}