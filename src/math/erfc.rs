// SPDX-License-Identifier: MIT
/*
 * File: src/math/erfc.rs
 *
 * The erfc function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn erfc(x: f64) -> f64 {
    libm::erfc(x)
}