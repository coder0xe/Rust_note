//! My-Crate
//! 
//! My-Crate is a test demo

/// Adds one to the number given
/// 
/// # Example
/// ```
/// let arg = 5;
/// let answer = crates_demo::add_one(arg);
/// assert_eq!(6, answer);
/// ```
/// 
pub fn add_one(x: i32) -> i32 {
    x + 1
}