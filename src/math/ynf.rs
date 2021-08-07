// SPDX-License-Identifier: MPL-2.0
/*
 * File: src/math/ynf.rs
 *
 * The ynf function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn ynf(n: i32, x: f32) -> f32 {
    libm::ynf(n, x)
}