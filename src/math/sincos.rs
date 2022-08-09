// SPDX-License-Identifier: MIT
/*
 * File: src/math/sincos.rs
 *
 * The sincos function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

pub fn sincos(x: f64) -> (f64, f64) {
    libm::sincos(x)
}
