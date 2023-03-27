// MIT License
//
// Copyright (c) 2023 Sergio Arroutbi
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref VERBOSE: Mutex<Verbose> = Mutex::new(Verbose::new());
}

pub struct Verbose {
    verbose: bool,
}

pub fn is_verbose(v: bool) {
    VERBOSE.lock().unwrap().verbose = v;
}

pub fn dump_verbose(s: &String) {
    if VERBOSE.lock().unwrap().verbose {
        println!("{}", s);
    }
}

impl Verbose {
    fn new() -> Verbose {
        Verbose { verbose: true }
    }
}
