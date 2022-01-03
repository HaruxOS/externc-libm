// SPDX-License-Identifier: MIT
/*
 * File: src/math/tgammaf.rs
 *
 * The tgammaf function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn tgammaf(x: f32) -> f32 {
    libm::tgammaf(x)
}