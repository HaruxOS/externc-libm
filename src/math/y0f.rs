// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/math/y0f.rs
 *
 * The y0f function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn y0f(x: f32) -> f32 {
    libm::y0f(x)
}