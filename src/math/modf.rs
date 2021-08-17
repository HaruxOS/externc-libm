// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/math/modf.rs
 *
 * The modf function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn modf(x: f64) -> (f64, f64) {
    libm::modf(x)
}