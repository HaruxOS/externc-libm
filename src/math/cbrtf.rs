// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/math/cbrtf.rs
 *
 * The cbrtf function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn cbrtf(x: f32) -> f32 {
    libm::cbrtf(x)
}