// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/math/sinhf.rs
 *
 * The sinhf function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn sinhf(x: f32) -> f32 {
    libm::sinhf(x)
}