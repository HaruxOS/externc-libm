// SPDX-License-Identifier: MPL-2.0
/*
 * File: src/math/fdimf.rs
 *
 * The fdimf function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn fdimf(x: f32, y: f32) -> f32 {
    libm::fdimf(x, y)
}