// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/math/ldexp.rs
 *
 * The ldexp function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn ldexp(x: f64, n: i32) -> f64 {
    libm::ldexp(x, n)
}