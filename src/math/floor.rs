// SPDX-License-Identifier: MPL-2.0
/*
 * File: src/math/floor.rs
 *
 * The floor function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn floor(x: f64) -> f64 {
    libm::floor(x)
}