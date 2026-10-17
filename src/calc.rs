// This is a temporal comment describing the bug this PR fixes.
// The old add() returned the wrong result; now it adds 2.
// TODO: remove this comment before merging.
pub fn add(a: i32, b: i32) -> i32 {
    // TODO: handle overflow
    a + b
}

/// Useful for the test suite.
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
