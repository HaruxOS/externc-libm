// SPDX-License-Identifier: MPL-2.0
/*
 * File: src/math/exp2f.rs
 *
 * The exp2f function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn exp2f(x: f32) -> f32 {
    libm::exp2f(x)
}