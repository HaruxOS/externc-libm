// SPDX-License-Identifier: MPL-2.0
/*
 * File: src/math/erff.rs
 *
 * The erff function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn erff(x: f32) -> f32 {
    libm::erff(x)
}