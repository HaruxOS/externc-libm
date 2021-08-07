// SPDX-License-Identifier: MPL-2.0
/*
 * File: src/math/j1.rs
 *
 * The j1 function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn j1(x: f64) -> f64 {
    libm::j1(x)
}