// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/math/pow.rs
 *
 * The pow function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn pow(x: f64, y: f64) -> f64 {
    libm::pow(x, y)
}