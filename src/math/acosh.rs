// SPDX-License-Identifier: MPL-2.0
/*
 * File: src/math/acos.rs
 *
 * The acosh function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn acosh(x: f64) -> f64 {
    libm::acosh(x)
}