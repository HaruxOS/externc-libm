// SPDX-License-Identifier: MPL-2.0
/*
 * File: src/math/log2.rs
 *
 * The log2 function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn log2(x: f64) -> f64 {
    libm::log2(x)
}