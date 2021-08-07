// SPDX-License-Identifier: MPL-2.0
/*
 * File: src/math/exp10.rs
 *
 * The exp10 function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn exp10(x: f64) -> f64 {
    libm::exp10(x)
}