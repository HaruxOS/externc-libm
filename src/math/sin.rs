// SPDX-License-Identifier: MPL-2.0
/*
 * File: src/math/sin.rs
 *
 * The sin function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn sin(x: f64) -> f64 {
    libm::sin(x)
}