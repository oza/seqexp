// Copyright 2017 Tsuyoshi Ozawa
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

use std::env;
use std::process;

fn main() {
    let mut args = env::args();

    args.next();
    let base = match args.next() {
        Some(v) => v.parse::<i32>().unwrap(),
        None => process::exit(1)
    };
    if base < 0 {
        process::exit(1);
    }

    let from = match args.next() {
        Some(v) => v.parse::<i32>().unwrap(),
        None => process::exit(1)
    };
    if from < 0 {
        process::exit(1);
    }

    let to = match args.next() {
        Some(v) => v.parse::<i32>().unwrap(),
        None => process::exit(1)
    };
    if from > to {
        process::exit(1);
    }


    let mut i = from;
    let mut output = "".to_string();
    let mut v = 1;
    while i <= to {
        if i == 0 {
            v = 1;
        } else {
            v *= base;
        }
        let vstr = &v.to_string();
        output = output + &vstr + " ";
        i += 1;
    }
    println!("{}", output);
}
