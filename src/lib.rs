#![no_std]

#[cfg(debug_assertions)]
#[macro_export]
/// Execute this statement in debug but not in release build.
macro_rules! at_debug {
    ($statement:stmt) => {
        $statement;
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
/// Execute this statement in debug but not in release build.
macro_rules! at_debug {
    ($statement:stmt) => {};
}
