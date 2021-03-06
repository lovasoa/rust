// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// aux-build:lifetime_bound_will_change_warning_lib.rs

// Test that various corner cases cause an error. These are tests
// that used to pass before we tweaked object defaults.

#![allow(dead_code)]
#![allow(unused_variables)]


extern crate lifetime_bound_will_change_warning_lib as lib;

fn just_ref(x: &Fn()) {
}

fn ref_obj(x: &Box<Fn()>) {
    // this will change to &Box<Fn()+'static>...

    // Note: no warning is issued here, because the type of `x` will change to 'static
    if false { ref_obj(x); }
}

fn test1<'a>(x: &'a Box<Fn()+'a>) {
    // just_ref will stay the same.
    just_ref(&**x)
}

fn test1cc<'a>(x: &'a Box<Fn()+'a>) {
    // same as test1, but cross-crate
    lib::just_ref(&**x)
}

fn test2<'a>(x: &'a Box<Fn()+'a>) {
    // but ref_obj will not, so warn.
    ref_obj(x) //~ ERROR mismatched types
}

fn test2cc<'a>(x: &'a Box<Fn()+'a>) {
    // same as test2, but cross crate
    lib::ref_obj(x) //~ ERROR mismatched types
}

fn test3<'a>(x: &'a Box<Fn()+'static>) {
    // here, we have a 'static bound, so even when ref_obj changes, no error results
    ref_obj(x)
}

fn test3cc<'a>(x: &'a Box<Fn()+'static>) {
    // same as test3, but cross crate
    lib::ref_obj(x)
}


fn main() {
}
