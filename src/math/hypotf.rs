// SPDX-License-Identifier: MIT
/*
 * File: src/math/hypotf.rs
 *
 * The hypotf function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn hypotf(x: f32, y: f32) -> f32 {
    libm::hypotf(x, y)
}