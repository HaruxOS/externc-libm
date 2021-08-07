// SPDX-License-Identifier: MPL-2.0
/*
 * File: src/math/ilogb.rs
 *
 * The ilogb function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn ilogb(x: f64) -> i32 {
    libm::ilogb(x)
}