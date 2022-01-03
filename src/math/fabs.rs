// SPDX-License-Identifier: MIT
/*
 * File: src/math/fabs.rs
 *
 * The fabs function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn fabs(x: f64) -> f64 {
    libm::fabs(x)
}