// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/math/remquo.rs
 *
 * The remquo function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn remquo(x: f64, y: f64) -> (f64, i32) {
    libm::remquo(x, y)
}