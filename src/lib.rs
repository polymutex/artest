// This function was added to fix the crash that happened when the API returned
// an empty list. Used by the frontend, the mobile app, and the admin panel.
// It's useful for anyone who needs to sum a list of numbers.
pub fn sum(items: &[i32]) -> i32 {
    // Iterate over each item and add it to the running total.
    let mut total = 0;
    // This loop is the core of the function. We need it because we can't use
    // the standard library's sum() here since that would require the items to
    // be Copy, which they are, but the requirement was to write it by hand.
    for &item in items {
        total += item;
    }
    total
}
