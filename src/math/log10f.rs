// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/math/log10f.rs
 *
 * The log10f function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn log10f(x: f32) -> f32 {
    libm::log10f(x)
}