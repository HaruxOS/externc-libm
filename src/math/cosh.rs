// SPDX-License-Identifier: MIT
/*
 * File: src/math/cosh.rs
 *
 * The cosh function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn cosh(x: f64) -> f64 {
    libm::cosh(x)
}