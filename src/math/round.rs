// SPDX-License-Identifier: MPL-2.0
/*
 * File: src/math/round.rs
 *
 * The round function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn round(x: f64) -> f64 {
    libm::round(x)
}