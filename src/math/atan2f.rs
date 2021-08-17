// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/math/atan2f.rs
 *
 * The atan2f function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn atan2f(y: f32, x: f32) -> f32 {
    libm::atan2f(y, x)
}