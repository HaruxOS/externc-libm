// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/math/sqrtf.rs
 *
 * The sqrtf function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn sqrtf(x: f32) -> f32 {
    libm::sqrtf(x)
}