// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/math/fma.rs
 *
 * The fma function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn fma(x: f64, y: f64, z: f64) -> f64 {
    libm::fma(x, y, z)
}