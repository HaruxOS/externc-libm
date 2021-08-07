// SPDX-License-Identifier: MPL-2.0
/*
 * File: src/math/cbrt.rs
 *
 * The cbrt function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn cbrt(x: f64) -> f64 {
    libm::cbrt(x)
}