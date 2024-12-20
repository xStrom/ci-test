// Copyright 2024 the CI's \n "cool" .^$*+?()[{\| $? Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! This is a simple test project so the CI has something to verify.

/// Super cool function.
///
/// ```
/// //panic!("Fails on purpose");
/// let x = 5;
/// ```
pub fn cool() -> bool {
    #[cfg(debug_assertions)]
    return true;
    #[cfg(not(debug_assertions))]
    return false;
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn generate_data() {
        let mut one = File::open("data/one.txt").unwrap();
        let mut contents = String::new();
        one.read_to_string(&mut contents).unwrap();
        if contents != "The One" {
            let mut file = File::create("data/two.txt").unwrap();
            file.write_all(b"The Two").unwrap();
            panic!("Unexpected contents!");
        }
        s;
    }
}
