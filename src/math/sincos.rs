// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/math/sincos.rs
 *
 * The sincos function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn sincos(x: f64) -> (f64, f64) {
    libm::sincos(x)
}