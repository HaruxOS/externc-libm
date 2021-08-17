// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/math/ilogbf.rs
 *
 * The ilogbf function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn ilogbf(x: f32) -> i32 {
    libm::ilogbf(x)
}