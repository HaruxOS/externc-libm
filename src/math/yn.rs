// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/math/yn.rs
 *
 * The yn function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn yn(n: i32, x: f64) -> f64 {
    libm::yn(n, x)
}