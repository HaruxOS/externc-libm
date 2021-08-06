// SPDX-License-Identifier: MPL-2.0
/*
 * File: src/math/acos.rs
 *
 * The acoshf function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn acoshf(x: f32) -> f32 {
    libm::acoshf(x)
}