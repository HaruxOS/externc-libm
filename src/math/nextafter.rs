// SPDX-License-Identifier: MPL-2.0
/*
 * File: src/math/nextafter.rs
 *
 * The nextafter function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn nextafter(x: f64, y: f64) -> f64 {
    libm::nextafter(x, y)
}