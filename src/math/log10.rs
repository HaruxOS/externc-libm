// SPDX-License-Identifier: MIT
/*
 * File: src/math/log10.rs
 *
 * The log10 function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn log10(x: f64) -> f64 {
    libm::log10(x)
}