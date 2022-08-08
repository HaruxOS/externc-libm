// SPDX-License-Identifier: MIT
/*
 * File: src/math/lgammaf.rs
 *
 * The lgammaf function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

pub fn lgammf(x: f32) -> f32 {
    libm::lgammaf(x)
}
