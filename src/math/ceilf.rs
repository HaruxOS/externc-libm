// SPDX-License-Identifier: MIT
/*
 * File: src/math/ceilf.rs
 *
 * The ceilf function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn ceilf(x: f32) -> f32 {
    libm::ceilf(x)
}