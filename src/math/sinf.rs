// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/math/sinf.rs
 *
 * The sinf function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn sinf(x: f32) -> f32 {
    libm::sinf(x)
}