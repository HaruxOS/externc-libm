// SPDX-License-Identifier: MIT
/*
 * File: src/math/expf.rs
 *
 * The expf function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn expf(x: f32) -> f32 {
    libm::expf(x)
}