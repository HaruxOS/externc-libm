// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/math/remquof.rs
 *
 * The remquof function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn remquof(x: f32, y: f32) -> (f32, i32) {
    libm::remquof(x, y)
}