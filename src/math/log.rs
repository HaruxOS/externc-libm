// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/math/log.rs
 *
 * The log function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn log(x: f64) -> f64 {
    libm::log(x)
}