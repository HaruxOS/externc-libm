// SPDX-License-Identifier: MIT
/*
 * File: src/math/y1.rs
 *
 * The y1 function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn y1(x: f64) -> f64 {
    libm::y1(x)
}