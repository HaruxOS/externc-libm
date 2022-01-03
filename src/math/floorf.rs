// SPDX-License-Identifier: MIT
/*
 * File: src/math/floorf.rs
 *
 * The floorf function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn floorf(x: f32) -> f32 {
    libm::floorf(x)
}