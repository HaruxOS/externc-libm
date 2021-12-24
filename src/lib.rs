// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/lib.rs
 *
 * The externc-libm library.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#![no_std]

pub mod math {
    pub mod acos;
    pub mod acosf;
    pub mod acosh;
    pub mod acoshf;
    pub mod asin;
    pub mod asinf;
    pub mod asinh;
    pub mod asinhf;
    pub mod atan;
    pub mod atan2;
    pub mod atan2f;
    pub mod atanf;
    pub mod atanh;
    pub mod atanhf;
    pub mod cbrt;
    pub mod cbrtf;
    pub mod ceil;
    pub mod ceilf;
    pub mod copysign;
    pub mod copysignf;
    pub mod cos;
    pub mod cosf;
    pub mod cosh;
    pub mod coshf;
    pub mod erf;
    pub mod erfc;
    pub mod erfcf;
    pub mod erff;
    pub mod exp;
    pub mod exp2;
    pub mod exp2f;
    pub mod exp10;
    pub mod exp10f;
    pub mod expf;
    pub mod expm1;
    pub mod expm1f;
    pub mod fabs;
    pub mod fabsf;
    pub mod fdim;
    pub mod fdimf;
    pub mod floor;
    pub mod floorf;
    pub mod fma;
    pub mod fmaf;
    pub mod fmax;
    pub mod fmaxf;
    pub mod fmin;
    pub mod fminf;
    pub mod fmod;
    pub mod fmodf;
    pub mod frexp;
    pub mod frexpf;
    pub mod hypot;
    pub mod hypotf;
    pub mod ilogb;
    pub mod ilogbf;
    pub mod j0;
    pub mod j0f;
    pub mod j1;
    pub mod j1f;
    pub mod jn;
    pub mod jnf;
    pub mod ldexp;
    pub mod ldexpf;
    pub mod lgamma;
    pub mod lgamma_r;
    pub mod lgammaf;
    pub mod lgammaf_r;
    pub mod log;
    pub mod log1p;
    pub mod log1pf;
    pub mod log2;
    pub mod log2f;
    pub mod log10;
    pub mod log10f;
    pub mod logf;
    pub mod modf;
    pub mod modff;
    pub mod nextafter;
    pub mod nextafterf;
    pub mod pow;
    pub mod powf;
    pub mod remainder;
    pub mod remainderf;
    pub mod remquo;
    pub mod remquof;
    pub mod round;
    pub mod roundf;
    pub mod scalbn;
    pub mod scalbnf;
    pub mod sin;
    pub mod sincos;
    pub mod sincosf;
    pub mod sinf;
    pub mod sinh;
    pub mod sinhf;
    pub mod sqrt;
    pub mod sqrtf;
    pub mod tan;
    pub mod tanf;
    pub mod tanh;
    pub mod tanhf;
    pub mod tgamma;
    pub mod tgammaf;
    pub mod trunc;
    pub mod truncf;
    pub mod y0;
    pub mod y0f;
    pub mod y1;
    pub mod y1f;
    pub mod yn;
    pub mod ynf;
}



// FIXME: Scuffed
#[no_mangle]
extern "C" fn __truncdfsf2(_x: f64) -> f32 {
    0.0
}


