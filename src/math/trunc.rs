// SPDX-License-Identifier: MPL-2.0
/*
 * File: src/math/trunc.rs
 *
 * The trunc function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn trunc(x: f64) -> f64 {
    libm::trunc(x)
}