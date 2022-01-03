// SPDX-License-Identifier: MIT
/*
 * File: src/math/asin.rs
 *
 * The asin function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn asin(x: f64) -> f64 {
    libm::asin(x)
}