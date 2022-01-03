// SPDX-License-Identifier: MIT
/*
 * File: src/math/fabsf.rs
 *
 * The fabsf function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn fabsf(x: f32) -> f32 {
    libm::fabsf(x)
}