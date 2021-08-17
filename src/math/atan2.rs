// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/math/atan2.rs
 *
 * The atan2 function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn atan2(y: f64, x: f64) -> f64 {
    libm::atan2(y, x)
}