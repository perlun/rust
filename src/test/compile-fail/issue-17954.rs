// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(cfg_target_thread_local, thread_local)]

#[cfg_attr(target_thread_local, thread_local)]
static FOO: u8 = 3;

// On platforms *without* `#[thread_local]`, create
// a reference to a temporary to fake the same error.
#[cfg(not(target_thread_local))]
macro_rules! let_ {
    ($name:ident = $tls:expr) => {
        let dummy = *$tls;
        let $name = &dummy;
    }
}

#[cfg(target_thread_local)]
macro_rules! let_ {
    ($name:ident = $tls:expr) => {
        let $name = $tls;
    }
}

fn main() {
    let_!(a = &FOO);
    //~^ ERROR borrowed value does not live long enough
    //~| does not live long enough
    //~| NOTE borrowed value must be valid for the static lifetime

    std::thread::spawn(move || {
        println!("{}", a);
    });
} //~ temporary value only lives until here
