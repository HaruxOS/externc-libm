// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/math/sinh.rs
 *
 * The sinh function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn sinh(x: f64) -> f64 {
    libm::sinh(x)
}