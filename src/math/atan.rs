// SPDX-License-Identifier: MPL-2.0
/*
 * File: src/math/atan.rs
 *
 * The atan function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn atan(x: f64) -> f64 {
    libm::atan(x)
}