// SPDX-License-Identifier: MIT
/*
 * File: src/math/erfcf.rs
 *
 * The erfcf function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn erfcf(x: f32) -> f32 {
    libm::erfcf(x)
}