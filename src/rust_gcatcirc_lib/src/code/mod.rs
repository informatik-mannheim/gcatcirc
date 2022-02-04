use std::fmt;
use crate::graph_code::CodeGraph;
use crate::graph_circ::{CircGraph, CircGraphErr};

mod code_tests;

#[derive(Debug, PartialEq)]
pub enum CircCodeErr {
    EmptyCode,
    EmptyWord,
}

impl fmt::Display for CircCodeErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use CircCodeErr::*;
        match self {
            EmptyCode => write!(f, "Empty Code"),
            EmptyWord => write!(f, "Empty Word"),
        }
    }
}

/// This struct wraps a set of tuple/words into a circular code.
#[derive(Debug, Clone)]
pub struct CircCode {
    /// A id or name of the code.
    pub id: String,
    /// A set of tuples/words.
    pub(crate) code: Vec<String>,
    /// A ordered list of all  of all tuple lengths.
    pub(crate) tuple_length: Vec<usize>,
    /// The alphabet used for all tuple in the code.
    pub(crate) alphabet: Vec<char>,
}

impl Default for CircCode {
    fn default() -> Self {
        return CircCode {
            code: vec!["A".to_string()],
            tuple_length: vec![1],
            id: "no id".to_string(),
            alphabet: vec!['A'],
        };
    }
}

impl CircCode {
    /// Returns a new [CircCode]
    ///
    /// Establishes all used tuple lengths and stores them into `tuple_length`. It also collects the `alphabet`.
    ///
    /// # Arguments
    /// * `code` a set of words
    ///
    /// # Errors
    /// * `CircCodeErr::EmptyCode` if the code vector is empty
    /// * `CircCodeErr::EmptyWord` if the code vector contains a empty word
    ///
    /// # Example
    /// ```
    /// use rust_gcatcirc_lib::code::CircCode;
    ///
    /// fn main() {
    ///     let code = match CircCode::new_from_vec(vec!["ADB".to_string(), "BA".to_string(), "AAD".to_string()]) {
    ///         Ok(code) => code,
    ///         Err(e) =>  {
    ///             eprintln!("{}", e);
    ///             return;
    ///         }
    ///     };
    /// }
    /// ```
    pub fn new_from_vec(code: Vec<String>) -> Result<Self, CircCodeErr> {
        if code.is_empty() { return Err(CircCodeErr::EmptyCode); }
        let mut tuple_length = Vec::new();
        let code_ptr = &code;
        for w in code_ptr {
            tuple_length.push(w.len())
        }
        tuple_length.sort();
        tuple_length.dedup();

        if tuple_length.contains(&0) { return Err(CircCodeErr::EmptyWord); }
        let mut code = code.clone();
        code.dedup();

        let mut alphabet: Vec<char> = code_ptr.join("").chars().collect();
        alphabet.sort();
        alphabet.dedup();

        Ok(CircCode {
            code: code,
            id: format!("unknown"),
            tuple_length: tuple_length,
            alphabet: alphabet,
        })
    }

    /// Returns a new [CircCode]
    ///
    /// Establishes all used tuple lengths and stores them into `tuple_length`. It also collects the `alphabet`.
    ///
    /// # Arguments
    /// * `sequence` a sequence
    /// * `tuple_length` the tuple length used to separate the sequence
    ///
    /// # Errors
    /// * `CircCodeErr::EmptyCode` if the code vector is empty
    ///
    /// # Example
    /// ```
    /// use rust_gcatcirc_lib::code::CircCode;
    ///
    /// fn main() {
    ///     //Code -> {ADB, BAA}
    ///     let code = match CircCode::new_from_seq("ADBBAAAD".to_string(), 3) {
    ///         Ok(code) => code,
    ///         Err(e) =>  {
    ///             eprintln!("{}", e);
    ///             return;
    ///         }
    ///     };
    /// }
    /// ```
    pub fn new_from_seq(sequence: String, tuple_length: usize) -> Result<Self, CircCodeErr> {
        if sequence.is_empty() { return Err(CircCodeErr::EmptyCode); }
        let v: Vec<char> = sequence.chars().collect();
        let mut alphabet = v.clone();
        alphabet.sort();
        alphabet.dedup();

        if tuple_length > v.len() { return Err(CircCodeErr::EmptyCode); }

        let mut new_code: Vec<String> = Vec::new();

        for i in (tuple_length..=v.len()).step_by(tuple_length) {
            let a = (&v[(i - tuple_length)..i]).to_vec();
            new_code.push(a.iter().collect())
        }

        new_code.dedup();

        Ok(CircCode {
            code: new_code,
            id: format!("unknown"),
            tuple_length: [tuple_length].into(),
            alphabet: alphabet,
        })
    }

    /// Returns a set of tuples/words.
    pub fn get_code(&self) -> Vec<String> { return self.code.clone(); }
    ///  Returns a ordered list of all  of all tuple lengths.
    pub fn get_tuple_length(&self) -> Vec<usize> { return self.tuple_length.clone(); }
    /// Returns the alphabet used for all tuple in the code.
    pub fn get_alphabet(&self) -> Vec<char> { return self.alphabet.clone(); }

    /// Checks whether the set wof words is a code or not
    pub fn is_code(&self) -> bool {
        let graph = CodeGraph::new(self);
        return graph.is_code();
    }

    /// Checks whether the set of words is a code or not.
    ///
    /// If not it returns all ambiguous_sequences
    ///
    /// # Example
    /// ```
    /// use rust_gcatcirc_lib::code::CircCode;
    ///
    /// fn main() {
    ///     let code = match CircCode::new_from_vec(vec!["ADBB".to_string(), "BB".to_string(), "AD".to_string()]) {
    ///         Ok(code) => code,
    ///         _ => unimplemented!(),
    ///     };
    ///
    ///     let (is_code, ambiguous_sequences) = code.all_ambiguous_sequences();
    ///     if !is_code {
    ///         // Ambiguous sequences -> ADBB
    ///         println!("Ambiguous sequences: {:?}", ambiguous_sequences);
    ///     }
    /// }
    /// ```
    pub fn all_ambiguous_sequences(&self) -> (bool, Vec<String>) {
        let graph = CodeGraph::new(self);
        return graph.all_ambiguous_sequences();
    }

    /// Returns the associated [Graph <i>G</i>](CircGraph)
    ///
    /// # Errors
    /// See [CircGraph::new()]
    pub fn get_associated_graph(&self) -> Result<CircGraph, CircGraphErr> {
        match CircGraph::new(self) {
            Ok(graph) => return Ok(graph),
            Err(e) => return Err(e),
        };
    }

    /// This function checks if a code is circular.
    ///
    /// A set of tuples X is a circular code if every concatenation of words w in X<sup>+</sup>
    /// written on a circle has only a single decomposition into words from X.
    pub fn is_circular(&self) -> bool {
        let graph = match CircGraph::new(self) {
            Ok(graph) => graph,
            _ => return false
        };
        return !graph.is_cyclic();
    }

    /// This function checks if a code is comma free
    ///
    /// Comma free codes are a more restrictive codes from the circular code family.
    /// A comma free code X is a code in which no concatenation of a
    /// nonempty suffix of any word from X and a nonempty prefix of any word from X forms a word from X.
    pub fn is_comma_free(&self) -> bool {
        let graph = match CircGraph::new(self) {
            Ok(graph) => graph,
            _ => return false
        };

        if let Some(longest_paths) = graph.all_longest_paths() {
            return longest_paths[0].len() <= 2;
        }
        return false;
    }

    /// This function checks if a code is strong comma free
    ///
    /// Strong comma free codes are a more restrictive codes from the circular code family.
    /// A strong comma free code X is a code in which no nonempty suffix of any word from X
    /// is a nonempty prefix of any word from X.
    pub fn is_strong_comma_free(&self) -> bool {
        let graph = match CircGraph::new(self) {
            Ok(graph) => graph,
            _ => return false
        };

        if let Some(longest_paths) = graph.all_longest_paths() {
            return longest_paths[0].len() == 1;
        }
        return false;
    }

    /// This function checks if a code is circular.
    ///
    /// K circle codes are a less restrictive code from the family of circle codes. These codes only ensure that for every
    /// concatenation of less than k tuples from X written on a circle, there is only one partition in tuples from X.
    pub fn get_exact_k_circular(&self) -> u32 {
        let graph = match CircGraph::new(self) {
            Ok(graph) => graph,
            _ => return 0
        };
        let (is_cyclic, all_paths) = graph.all_cycles();
        if !is_cyclic { return u32::MAX; } else if let Some(cycle) = all_paths.last() {
            if cycle.len() % 2 == 0 {
                return (cycle.len() as u32 / 2) - 1;
            } else {
                return cycle.len() as u32 - 1;
            }
        }

        return u32::MAX;
    }

    /// This function checks if a code is Cn-circular.
    ///
    /// That all circular permutations of the code (of all tuples) are circular codes again. This is an extended property of circular codes.
    pub fn is_cn_circular(&self) -> bool {
        let mut copy_code = self.clone();
        for _i in 1..*self.tuple_length.last().unwrap() {
            copy_code.shift(1);
            if !copy_code.is_circular() { return false; }
        }

        return self.is_circular();
    }

    /// Shifts each tuple by `sh` positions
    ///
    /// Let X={123, 332}, then c.shift(2) results in {312, 233}
    pub fn shift(&mut self, sh: i32) {
        self.code = self.code.iter().map(|w| {
            let sh: usize = (w.len() as i32 + (sh % w.len() as i32)) as usize % w.len();
            let prefix = (w[sh..]).to_string();
            return prefix + &w[..sh];
        }).collect();
    }
}

impl fmt::Display for CircCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {{ {} }} Alphabet = [{}]",
               self.id,
               self.code.join(", "),
               self.alphabet.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", "))
    }
}

impl From<Vec<String>> for CircCode {
    fn from(code: Vec<String>) -> Self {
        match CircCode::new_from_vec(code) {
            Ok(cc) => return cc,
            _ => return CircCode::default(),
        }
    }
}

impl From<(String, usize)> for CircCode {
    fn from(tuple: (String, usize)) -> Self {
        match CircCode::new_from_seq(tuple.0, tuple.1) {
            Ok(cc) => return cc,
            _ => return CircCode::default(),
        }
    }
}

impl PartialEq<CircCode> for CircCode {
    fn eq(&self, other: &CircCode) -> bool {
        let mut code_a = self.code.clone();
        let mut code_b = other.code.clone();
        code_a.sort();
        code_b.sort();
        code_a == code_b
    }
}

