// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/math/remainderf.rs
 *
 * The remainderf function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn remainderf(x: f32, y: f32) -> f32 {
    libm::remainderf(x, y)
}