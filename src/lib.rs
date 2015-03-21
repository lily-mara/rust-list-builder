#![feature(core)]

/// Generate a new iterable witn a list comprehension
/// ```
/// let x: Vec<i32> = gen!(x*4 => x in [1, 2, 3, 4]);
/// ```
#[macro_export]
macro_rules! gen {
    [$e:expr => $variable:ident in $iterable:expr] => (
        $iterable.iter().cloned().map(|$variable| $e).collect()
    );
    [$e:expr => $variable:ident in $iterable:expr, $condition:expr] => (
        $iterable.iter().cloned().filter(|$variable| $condition).map(|$variable| $e).collect()
    );
}

#[cfg(test)]
mod test;
