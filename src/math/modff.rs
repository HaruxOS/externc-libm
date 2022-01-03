// SPDX-License-Identifier: MIT
/*
 * File: src/math/modff.rs
 *
 * The modff function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn modff(x: f32) -> (f32, f32) {
    libm::modff(x)
}