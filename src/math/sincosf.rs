// SPDX-License-Identifier: MIT
/*
 * File: src/math/sincosf.rs
 *
 * The sincosf function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

pub fn sincosf(x: f32) -> (f32, f32) {
    libm::sincosf(x)
}
