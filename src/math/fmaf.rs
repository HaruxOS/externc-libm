// SPDX-License-Identifier: MPL-2.0
/*
 * File: src/math/fmaf.rs
 *
 * The fmaf function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn fmaf(x: f32, y: f32, z: f32) -> f32 {
    libm::fmaf(x, y, z)
}