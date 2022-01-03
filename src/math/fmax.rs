// SPDX-License-Identifier: MIT
/*
 * File: src/math/fmax.rs
 *
 * The fmax function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn fmax(x: f64, y: f64) -> f64 {
    libm::fmax(x, y)
}