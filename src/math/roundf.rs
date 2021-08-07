// SPDX-License-Identifier: MPL-2.0
/*
 * File: src/math/roundf.rs
 *
 * The roundf function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn roundf(x: f32) -> f32 {
    libm::roundf(x)
}