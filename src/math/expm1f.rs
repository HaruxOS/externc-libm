// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/math/expm1f.rs
 *
 * The expm1f function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn expm1f(x: f32) -> f32 {
    libm::expm1f(x)
}