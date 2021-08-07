// SPDX-License-Identifier: MPL-2.0
/*
 * File: src/math/log1pf.rs
 *
 * The log1pf function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn log1pf(x: f32) -> f32 {
    libm::log1pf(x)
}