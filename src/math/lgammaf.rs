// SPDX-License-Identifier: MIT
/*
 * File: src/math/lgammaf.rs
 *
 * The lgammaf function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn lgammf(x: f32) -> f32 {
    libm::lgammaf(x)
}