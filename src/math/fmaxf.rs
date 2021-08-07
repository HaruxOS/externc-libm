// SPDX-License-Identifier: MPL-2.0
/*
 * File: src/math/fmaxf.rs
 *
 * The fmaxf function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn fmaxf(x: f32, y: f32) -> f32 {
    libm::fmaxf(x, y)
}