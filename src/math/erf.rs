// SPDX-License-Identifier: MPL-2.0
/*
 * File: src/math/erf.rs
 *
 * The erf function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn erf(x: f64) -> f64 {
    libm::erf(x)
}