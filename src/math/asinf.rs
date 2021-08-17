// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/math/asinf.rs
 *
 * The asinf function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn asinf(x: f32) -> f32 {
    libm::asinf(x)
}