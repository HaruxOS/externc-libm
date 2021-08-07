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
#[path = "math/atan.rs"]
pub mod atan;
#[path = "math/atan2.rs"]
pub mod atan2;
#[path = "math/atan2f.rs"]
pub mod atan2f;
#[path = "math/atanf.rs"]
pub mod atanf;
#[path = "math/atanh.rs"]
pub mod atanh;
#[path = "math/atanhf.rs"]
pub mod atanhf;
#[path = "math/cbrt.rs"]
pub mod cbrt;
#[path = "math/cbrtf.rs"]
pub mod cbrtf;
#[path = "math/ceil.rs"]
pub mod ceil;
#[path = "math/ceilf.rs"]
pub mod ceilf;
#[path = "math/copysign.rs"]
pub mod copysign;
#[path = "math/copysignf.rs"]
pub mod copysignf;
#[path = "math/cos.rs"]
pub mod cos;
#[path = "math/cosf.rs"]
pub mod cosf;
#[path = "math/cosh.rs"]
pub mod cosh;
#[path = "math/coshf.rs"]
pub mod coshf;
#[path = "math/erf.rs"]
pub mod erf;
#[path = "math/erfc.rs"]
pub mod erfc;
#[path = "math/erfcf.rs"]
pub mod erfcf;
#[path = "math/erff.rs"]
pub mod erff;
#[path = "math/exp.rs"]
pub mod exp;
#[path = "math/exp2.rs"]
pub mod exp2;
#[path = "math/exp2f.rs"]
pub mod exp2f;
#[path = "math/exp10.rs"]
pub mod exp10;
#[path = "math/exp10f.rs"]
pub mod exp10f;
#[path = "math/expf.rs"]
pub mod expf;
#[path = "math/expm1.rs"]
pub mod expm1;
#[path = "math/expm1f.rs"]
pub mod expm1f;