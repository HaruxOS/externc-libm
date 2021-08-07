// SPDX-License-Identifier: MPL-2.0
/*
 * File: src/math/scalbnf.rs
 *
 * The scalbnf function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn scalbnf(x: f32, n: i32) -> f32 {
    libm::scalbnf(x, n)
}