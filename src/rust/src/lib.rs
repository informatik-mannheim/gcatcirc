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

extern crate rust_gcatcirc_lib;
use rust_gcatcirc_lib::code;


struct CircCode(code::CircCode);

/// This struct wraps a set of tuple/words into a circular code.
#[extendr]
impl CircCode {

    /// Returns a new [CircCode]
    ///
    /// Establishes all used tuple lengths and stores them into `tuple_length`. It also collects the `alphabet`.
    ///
    /// # Arguments
    /// * `code` a set of words
    fn new_from_vec(code: Vec<String>) -> Self {
        match code::CircCode::new_from_vec(code) {
            Ok(code) => return Self(code),
            Err(_e) => return Self(code::CircCode::default()),
        }
    }

    /// Returns a new [CircCode]
    ///
    /// Establishes all used tuple lengths and stores them into `tuple_length`. It also collects the `alphabet`.
    ///
    /// # Arguments
    /// * `sequence` a sequence
    /// * `tuple_length` the tuple length used to separate the sequence
    fn new_from_seq(code: String, tuple_length: u32) -> Self {
        match code::CircCode::new_from_seq(code, tuple_length as usize) {
            Ok(code) => return Self(code),
            Err(_e) => return Self(code::CircCode::default()),
        }
    }
    /// Returns a set of tuples/words.
    fn get_code(&self) -> Vec<String> {
        let CircCode(this) = self;
        return this.get_code();
    }
    /// Returns a ordered list of all  of all tuple lengths.
    fn get_tuple_length(&self)  -> Vec<usize>{
        let CircCode(this) = self;
        return this.get_tuple_length();
    }

    /// Sets (replaces old id) the id of the code.
    fn set_id(&mut self, id: String) {
        let CircCode(this) = self;
        this.id = id;
    }

    /// Returns the id of the code.
    fn get_id(&self) -> String {
        let CircCode(this) = self;
        return this.id.clone();
    }

    /// Returns the alphabet used for all tuple in the code.
    fn get_alphabet(&self) -> Vec<String> {
        let CircCode(this) = self;
        return this.get_alphabet().iter().map(|x| x.to_string()).collect();
    }

    /// Checks whether the set wof words is a code or not
    pub fn is_code(&self) -> bool {
        let CircCode(this) = self;
        return this.is_code();
    }

    /// Checks whether the set of words is a code or not.
    ///
    /// If not it returns all ambiguous_sequences
    fn all_ambiguous_sequences(&self) -> Vec<String> {
        let CircCode(this) = self;
        return this.all_ambiguous_sequences().1
    }

    /// Shifts each tuple by `sh` positions
    ///
    /// Let X={123, 332}, then c.shift(2) results in {312, 233}
    fn shift(&mut self, sh: i32) {
        let CircCode(this) = self;
        this.shift(sh);
    }

    // Returns a printable string
    ///
    /// The string is in form of "[Id](CircCode::get_id()) -> [Code](CircCode::get_code()) Alphabet = [Alphabet](CircCode::get_alphabet())"
    fn to_string(&self) -> String {
        let CircCode(this) = self;
        return this.to_string();
    }


}




// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C rust_gcatcirc_lib.code in `entrypoint.c`.
extendr_module! {
    mod gcatcirc; // like R package name
    impl CircCode;
}
