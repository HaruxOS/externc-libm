// SPDX-License-Identifier: MPL-2.0
/*
 * File: src/math/floorf.rs
 *
 * The floorf function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn floor(x: f32) -> f32 {
    libm::floorf(x)
}