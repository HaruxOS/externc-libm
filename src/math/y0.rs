// SPDX-License-Identifier: MPL-2.0
/*
 * File: src/math/y0.rs
 *
 * The y0 function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn y0(x: f64) -> f64 {
    libm::y0(x)
}