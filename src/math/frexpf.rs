// SPDX-License-Identifier: MIT
/*
 * File: src/math/frexpf.rs
 *
 * The frexpf function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn frexpf(x: f32) -> (f32, i32) {
    libm::frexpf(x)
}