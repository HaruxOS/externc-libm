// SPDX-License-Identifier: MIT
/*
 * File: src/math/asinh.rs
 *
 * The asinh function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn asinh(x: f64) -> f64 {
    libm::asinh(x)
}