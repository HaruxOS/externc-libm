// SPDX-License-Identifier: MPL-2.0
/*
 * File: src/lib.rs
 *
 * The externc-libm library.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#![no_std]

#[path = "math/acos.rs"]
pub mod acos;
#[path = "math/acosf.rs"]
pub mod acosf;
#[path = "math/acosh.rs"]
pub mod acosh;
#[path = "math/acoshf.rs"]
pub mod acoshf;
#[path = "math/asin.rs"]
pub mod asin;
#[path = "math/asinf.rs"]
pub mod asinf;
#[path = "math/asinh.rs"]
pub mod asinh;
#[path = "math/asinhf.rs"]
pub mod asinhf;