// SPDX-License-Identifier: MIT
/*
 * File: src/math/exp.rs
 *
 * The exp function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn exp(x: f64) -> f64 {
    libm::exp(x)
}