// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/math/fdim.rs
 *
 * The fdim function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn fdim(x: f64, y: f64) -> f64 {
    libm::fdim(x, y)
}