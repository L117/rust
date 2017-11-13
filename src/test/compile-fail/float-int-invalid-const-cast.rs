// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(i128_type)]
#![allow(const_err)] // this test is only about hard errors

use std::{f32, f64};

// Forces evaluation of constants, triggering hard error
fn force<T>(_: T) {}

fn main() {
    { const X: u16 = -1. as u16; force(X); } //~ ERROR constant evaluation error
    { const X: u128 = -100. as u128; force(X); } //~ ERROR constant evaluation error

    { const X: i8 = f32::NAN as i8; force(X); } //~ ERROR constant evaluation error
    { const X: i32 = f32::NAN as i32; force(X); } //~ ERROR constant evaluation error
    { const X: u64 = f32::NAN as u64; force(X); } //~ ERROR constant evaluation error
    { const X: u128 = f32::NAN as u128; force(X); } //~ ERROR constant evaluation error

    { const X: i8 = f32::INFINITY as i8; force(X); } //~ ERROR constant evaluation error
    { const X: u32 = f32::INFINITY as u32; force(X); } //~ ERROR constant evaluation error
    { const X: i128 = f32::INFINITY as i128; force(X); } //~ ERROR constant evaluation error
    { const X: u128 = f32::INFINITY as u128; force(X); } //~ ERROR constant evaluation error

    { const X: u8 = f32::NEG_INFINITY as u8; force(X); } //~ ERROR constant evaluation error
    { const X: u16 = f32::NEG_INFINITY as u16; force(X); } //~ ERROR constant evaluation error
    { const X: i64 = f32::NEG_INFINITY as i64; force(X); } //~ ERROR constant evaluation error
    { const X: i128 = f32::NEG_INFINITY as i128; force(X); } //~ ERROR constant evaluation error

    { const X: i8 = f64::NAN as i8; force(X); } //~ ERROR constant evaluation error
    { const X: i32 = f64::NAN as i32; force(X); } //~ ERROR constant evaluation error
    { const X: u64 = f64::NAN as u64; force(X); } //~ ERROR constant evaluation error
    { const X: u128 = f64::NAN as u128; force(X); } //~ ERROR constant evaluation error

    { const X: i8 = f64::INFINITY as i8; force(X); } //~ ERROR constant evaluation error
    { const X: u32 = f64::INFINITY as u32; force(X); } //~ ERROR constant evaluation error
    { const X: i128 = f64::INFINITY as i128; force(X); } //~ ERROR constant evaluation error
    { const X: u128 = f64::INFINITY as u128; force(X); } //~ ERROR constant evaluation error

    { const X: u8 = f64::NEG_INFINITY as u8; force(X); } //~ ERROR constant evaluation error
    { const X: u16 = f64::NEG_INFINITY as u16; force(X); } //~ ERROR constant evaluation error
    { const X: i64 = f64::NEG_INFINITY as i64; force(X); } //~ ERROR constant evaluation error
    { const X: i128 = f64::NEG_INFINITY as i128; force(X); } //~ ERROR constant evaluation error

    { const X: u8 = 256. as u8; force(X); } //~ ERROR constant evaluation error
    { const X: i8 = -129. as i8; force(X); } //~ ERROR constant evaluation error
    { const X: i8 = 128. as i8; force(X); } //~ ERROR constant evaluation error
    { const X: i32 = 2147483648. as i32; force(X); } //~ ERROR constant evaluation error
    { const X: i32 = -2147483904. as i32; force(X); } //~ ERROR constant evaluation error
    { const X: u32 = 4294967296. as u32; force(X); } //~ ERROR constant evaluation error
    { const X: u128 = 1e40 as u128; force(X); } //~ ERROR constant evaluation error
    { const X: i128 = 1e40 as i128; force(X); } //~ ERROR constant evaluation error
}