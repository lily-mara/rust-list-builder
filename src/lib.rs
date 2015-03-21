#![feature(core)]

/// Generate a new iterable witn a list comprehension. This macro tries to follow the syntax of
/// Python's list comprehensions. This is a very flexable macro that allows the generation of any
/// iterable that implements `std::iter::FromIterator`. The resulting type will be determined by
/// the type of the variable that you are attempting to assign to. You can create a `Vec`:
///
/// ```ignore
/// let x: Vec<i32> = gen![i*30 => i in [1, 2, 3, 4, 5]];
/// ```
///
/// You can generate a `HashSet`:
///
/// ```ignore
/// let x: HashSet<i32> = gen![i*30 => i in [1, 2, 3, 4, 5]];
/// ```
///
/// You can even use conditionals to generate stuff:
///
/// ```ignore
/// let x: HashSet<i32> = gen![i => i in [1, 2, 3, 4, 5], x % 2 == 0];
/// assert_eq!(x, vec![2, 4]);
/// ```
///
/// Comparisson to Python's list comprehension
/// ===
///
/// Python
/// ---
/// ```python
/// x = [i*4 for i in range(1, 5)]
/// ```
///
/// Rust with gen! macro
/// ---
/// ```ignore
/// let x: Vec<i32> = gen!(x*4 => x in [1, 2, 3, 4]);
/// ```
#[macro_export]
#[macro_use]
macro_rules! gen {
    [$e:expr => $variable:ident in $iterable:expr] => (
        $iterable.iter().cloned().map(|$variable| $e).collect()
    );
    [$e:expr => $variable:ident in $iterable:expr, $condition:expr] => (
        $iterable.iter().cloned().filter(|$variable| $condition).map(|$variable| $e).collect()
    );
}
