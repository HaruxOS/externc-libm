// SPDX-License-Identifier: MIT
/*
 * File: src/math/modf.rs
 *
 * The modf function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

pub fn modf(x: f64) -> (f64, f64) {
    libm::modf(x)
}
