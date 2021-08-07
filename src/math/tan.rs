// SPDX-License-Identifier: MPL-2.0
/*
 * File: src/math/tan.rs
 *
 * The tan function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn tan(x: f64) -> f64 {
    libm::tan(x)
}