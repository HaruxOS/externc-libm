// SPDX-License-Identifier: MPL-2.0
/*
 * File: src/math/hypot.rs
 *
 * The hypot function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn hypot(x: f64, y: f64) -> f64 {
    libm::hypot(x, y)
}