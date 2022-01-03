// SPDX-License-Identifier: MIT
/*
 * File: src/math/exp2.rs
 *
 * The exp2 function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn exp2(x: f64) -> f64 {
    libm::exp2(x)
}