// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Test that we DO warn when lifetime name is not used at all.

#![deny(unused_lifetimes)]
#![allow(dead_code)]
#![allow(unused_variables)]

struct Foo { }

impl<'a> Foo { } //~ ERROR `'a` never used

fn main() { }
