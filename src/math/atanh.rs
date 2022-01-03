// SPDX-License-Identifier: MIT
/*
 * File: src/math/atanh.rs
 *
 * The atanh function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn atanh(x: f64) -> f64 {
    libm::atanh(x)
}