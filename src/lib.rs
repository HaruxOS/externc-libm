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
#[path = "math/fabs.rs"]
pub mod fabs;
#[path = "math/fabsf.rs"]
pub mod fabsf;
#[path = "math/fdim.rs"]
pub mod fdim;
#[path = "math/fdimf.rs"]
pub mod fdimf;
#[path = "math/floor.rs"]
pub mod floor;
#[path = "math/floorf.rs"]
pub mod floorf;
#[path = "math/fma.rs"]
pub mod fma;
#[path = "math/fmaf.rs"]
pub mod fmaf;
#[path = "math/fmax.rs"]
pub mod fmax;
#[path = "math/fmaxf.rs"]
pub mod fmaxf;
#[path = "math/fmin.rs"]
pub mod fmin;
#[path = "math/fminf.rs"]
pub mod fminf;
#[path = "math/fmod.rs"]
pub mod fmod;
#[path = "math/fmodf.rs"]
pub mod fmodf;
#[path = "math/frexp.rs"]
pub mod frexp;
#[path = "math/frexpf.rs"]
pub mod frexpf;
#[path = "math/hypot.rs"]
pub mod hypot;
#[path = "math/hypotf.rs"]
pub mod hypotf;
#[path = "math/ilogb.rs"]
pub mod ilogb;
#[path = "math/ilogbf.rs"]
pub mod ilogbf;
#[path = "math/j0.rs"]
pub mod j0;
#[path = "math/j0f.rs"]
pub mod j0f;
#[path = "math/j1.rs"]
pub mod j1;
#[path = "math/j1f.rs"]
pub mod j1f;
#[path = "math/jn.rs"]
pub mod jn;
#[path = "math/jnf.rs"]
pub mod jnf;
#[path = "math/ldexp.rs"]
pub mod ldexp;
#[path = "math/ldexpf.rs"]
pub mod ldexpf;
#[path = "math/lgamma.rs"]
pub mod lgamma;
#[path = "math/lgamma_r.rs"]
pub mod lgamma_r;
#[path = "math/lgammaf.rs"]
pub mod lgammaf;
#[path = "math/lgammaf_r.rs"]
pub mod lgammaf_r;
#[path = "math/log.rs"]
pub mod log;
#[path = "math/log1p.rs"]
pub mod log1p;
#[path = "math/log1pf.rs"]
pub mod log1pf;
#[path = "math/log2.rs"]
pub mod log2;
#[path = "math/log2f.rs"]
pub mod log2f;
#[path = "math/log10.rs"]
pub mod log10;
#[path = "math/log10f.rs"]
pub mod log10f;
#[path = "math/logf.rs"]
pub mod logf;