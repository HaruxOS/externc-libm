// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/math/y1f.rs
 *
 * The y1f function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn y1f(x: f32) -> f32 {
    libm::y1f(x)
}