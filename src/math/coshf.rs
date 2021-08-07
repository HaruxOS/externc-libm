// SPDX-License-Identifier: MPL-2.0
/*
 * File: src/math/coshf.rs
 *
 * The coshf function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn coshf(x: f32) -> f32 {
    libm::coshf(x)
}