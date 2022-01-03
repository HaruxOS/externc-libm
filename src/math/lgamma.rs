// SPDX-License-Identifier: MIT
/*
 * File: src/math/lgamma.rs
 *
 * The lgamma function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn lgamma(x: f64) -> f64 {
    libm::lgamma(x)
}