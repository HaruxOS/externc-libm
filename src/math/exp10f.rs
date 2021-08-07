// SPDX-License-Identifier: MPL-2.0
/*
 * File: src/math/exp10f.rs
 *
 * The exp10f function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn exp10f(x: f32) -> f32 {
    libm::exp10f(x)
}