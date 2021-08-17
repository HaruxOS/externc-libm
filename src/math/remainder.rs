// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/math/remainder.rs
 *
 * The remainder function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn remainder(x: f64, y: f64) -> f64 {
    libm::remainder(x, y)
}