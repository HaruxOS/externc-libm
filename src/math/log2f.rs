// SPDX-License-Identifier: MPL-2.0
/*
 * File: src/math/log2f.rs
 *
 * The log2f function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn log2f(x: f32) -> f32 {
    libm::log2f(x)
}