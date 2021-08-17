// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/math/tanh.rs
 *
 * The tanh function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn tanh(x: f64) -> f64 {
    libm::tanh(x)
}