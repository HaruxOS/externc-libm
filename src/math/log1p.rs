// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/math/log1p.rs
 *
 * The log1p function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn log1p(x: f64) -> f64 {
    libm::log1p(x)
}