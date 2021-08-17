// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/math/j1f.rs
 *
 * The j1f function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn j1f(x: f32) -> f32 {
    libm::j1f(x)
}