// SPDX-License-Identifier: MIT
/*
 * File: src/math/asinhf.rs
 *
 * The asinhf function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn asinhf(x: f32) -> f32 {
    libm::asinhf(x)
}