// SPDX-License-Identifier: MPL-2.0
/*
 * File: src/math/truncf.rs
 *
 * The truncf function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn truncf(x: f32) -> f32 {
    libm::truncf(x)
}