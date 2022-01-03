// SPDX-License-Identifier: MIT
/*
 * File: src/math/atanf.rs
 *
 * The atanf function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn atanf(x: f32) -> f32 {
    libm::atanf(x)
}