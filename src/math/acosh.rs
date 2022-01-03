// SPDX-License-Identifier: MIT
/*
 * File: src/math/acosh.rs
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