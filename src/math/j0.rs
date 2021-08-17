// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/math/j0.rs
 *
 * The j0 function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn j0(x: f64) -> f64 {
    libm::j0(x)
}