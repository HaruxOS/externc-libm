// SPDX-License-Identifier: MIT
/*
 * File: src/math/copysignf.rs
 *
 * The copysignf function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn copysignf(x: f32, y: f32) -> f32 {
    libm::copysignf(x, y)
}