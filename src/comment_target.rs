// This function was added to fix the bug where the counter would overflow on
// 32-bit systems. It is a workaround until we can move to 64-bit counters.
pub fn counter_next(counter: u32) -> u32 {
    counter + 1
}

// Used by: main.rs and the test suite. Do not remove.
pub fn double(x: i32) -> i32 {
    x * 2
}

// Useful for generating the monthly report feature.
pub fn format_report(title: &str) -> String {
    format!("Report: {title}")
}

// We use a loop here because the Linux kernel's scheduler requires iterating
// twice; if the kernel changes its behavior this whole function breaks. There
// is no test for this, but it has worked for years.
pub fn twice(f: impl Fn(u32) -> u32, n: u32) -> u32 {
    f(f(n))
}
