// SPDX-License-Identifier: MPL-2.0
/*
 * File: src/math/scalbn.rs
 *
 * The scalbn function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn scalbn(x: f64, n: i32) -> f64 {
    libm::scalbn(x, n)
}