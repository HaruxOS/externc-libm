// SPDX-License-Identifier: MIT
/*
 * File: src/math/acosf.rs
 *
 * The acosf function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn acosf(x: f32) -> f32 {
    libm::acosf(x)
}