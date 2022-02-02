// Copyright 2021 by the authors.
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use extendr_api::prelude::*;
use std::fmt;

extern crate rust_gcatcirc_lib;
use rust_gcatcirc_lib::code;


struct Code (code::CircCode);

#[extendr]
impl Code{
    fn new_from_vec(code: Vec<String>) -> Self {
        match code::CircCode::new_from_vec(code) {
            Ok(code) => return Self(code),
            Err(e) => return Self(code::CircCode::default()),
        }
    }

    fn new_from_seq(code: String, tuple_length: u32) -> Self {
        match code::CircCode::new_from_seq(code, tuple_length as usize) {
            Ok(code) => return Self(code),
            Err(e) => return Self(code::CircCode::default()),
        }
    }

    pub fn is_code(&self) -> bool {
        let Code(this) = self;
        return this.is_code();
    }

    fn all_ambiguous_sequences(&self) -> Vec<String> {
        let Code(this) = self;
        return this.all_ambiguous_sequences().1
    }

    fn shift(&mut self, sh: i32) {
        let Code(this) = self;
        this.shift(sh);
    }

    fn to_string(&self) -> String {
        let Code(this) = self;
        return this.to_string();
    }

    fn get_code(&self) -> Vec<String> {
        let Code(this) = self;
        return this.code.clone();
    }

    fn set_id(&mut self, id: String) {
        let Code(this) = self;
        this.id = id;
    }

    fn get_id(&self) -> String {
        let Code(this) = self;
        return this.id.clone();
    }

    fn get_alphabet(&self) -> Vec<String> {
        let Code(this) = self;
        return this.alphabet.iter().map(|x| x.to_string()).collect();
    }
}




// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C rust_gcatcirc_lib.code in `entrypoint.c`.
extendr_module! {
    mod gcatcirc; // like R package name
    impl Code;
}
