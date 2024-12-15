// Copyright 2024 the CI's \n "cool" .^$*+?()[{\| $? Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! This is a simple test project that won't compile to Wasm.

#[cfg(target_family = "wasm")]
compile_error!("`test_no_wasm` can't be compiled to Wasm!");
