// SPDX-License-Identifier: MIT
/*
 * File: src/math/tgamma.rs
 *
 * The tgamma function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn tgamma(x: f64) -> f64 {
    libm::tgamma(x)
}