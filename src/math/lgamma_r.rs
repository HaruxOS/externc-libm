// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/math/lgamma_r.rs
 *
 * The lgamma_r function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn lgamma_r(x: f64) -> (f64, i32) {
    libm::lgamma_r(x)
}