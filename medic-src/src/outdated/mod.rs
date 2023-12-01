#[cfg(test)]
mod check_test;
#[cfg(test)]
mod summary_test;

mod check;
mod summary;

pub use check::OutdatedCheck;
