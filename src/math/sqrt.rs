// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/math/sqrt.rs
 *
 * The sqrt function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn sqrt(x: f64) -> f64 {
    libm::sqrt(x)
}