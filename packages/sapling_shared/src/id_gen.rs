//! Functions for generating unique identifiers

/// Counter for generating unique IDs
static mut COUNTER: usize = 0;

/// Generate a unique numbered ID
pub fn get_numbered_id() -> usize {
    // SAFETY: This is safe because we only use this counter
    // for generating unique IDs and don't care about exact values
    unsafe {
        COUNTER += 1;
        COUNTER
    }
}

/// Reset the ID counter (mainly for testing)
#[cfg(test)]
pub fn reset_counter() {
    unsafe {
        COUNTER = 0;
    }
}
