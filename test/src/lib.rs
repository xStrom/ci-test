//! This is a simple test project so the CI has something to verify.

/// Super cool function.
pub fn cool() -> bool {
    false
}

#[cfg(test)]
mod tests {
    // CI will fail unless cargo nextest can execute at least one test per workspace.
    // Delete this dummy test once we have an actual real test.
    #[test]
    fn dummy_test_until_we_have_a_real_test() {}
}
