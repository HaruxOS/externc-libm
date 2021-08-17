// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/math/atanhf.rs
 *
 * The atanhf function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn atanhf(x: f32) -> f32 {
    libm::atanhf(x)
}