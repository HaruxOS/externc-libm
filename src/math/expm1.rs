// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/math/expm1.rs
 *
 * The expm1 function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn expm1(x: f64) -> f64 {
    libm::expm1(x)
}