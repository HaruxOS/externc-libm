// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/math/ceil.rs
 *
 * The ceil function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn ceil(x: f64) -> f64 {
    libm::ceil(x)
}