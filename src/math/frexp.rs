// SPDX-License-Identifier: MIT
/*
 * File: src/math/frexp.rs
 *
 * The frexp function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

pub fn frexp(x: f64) -> (f64, i32) {
    libm::frexp(x)
}
