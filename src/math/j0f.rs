// SPDX-License-Identifier: MIT
/*
 * File: src/math/j0f.rs
 *
 * The j0f function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn j0f(x: f32) -> f32 {
    libm::j0f(x)
}