// SPDX-License-Identifier: MIT
/*
 * File: src/math/fmodf.rs
 *
 * The fmodf function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn fmodf(x: f32, y: f32) -> f32 {
    libm::fmodf(x, y)
}