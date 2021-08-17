// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/math/fmin.rs
 *
 * The fmin function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn fmin(x: f64, y: f64) -> f64 {
    libm::fmin(x, y)
}