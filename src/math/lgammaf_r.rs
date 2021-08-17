// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/math/lgammaf_r.rs
 *
 * The lgammaf_r function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn lgammaf_r(x: f32) -> (f32, i32) {
    libm::lgammaf_r(x)
}