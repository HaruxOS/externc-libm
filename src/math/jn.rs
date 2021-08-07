// SPDX-License-Identifier: MPL-2.0
/*
 * File: src/math/jn.rs
 *
 * The jn function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn jn(n: i32, x: f64) -> f64 {
    libm::jn(n, x)
}