use std::cell::RefCell;
use std::rc::Rc;
use crate::code;
use code::CircCode;

const ROOT: char = '_';

/// A graph representing a set of words. A graph <i>G</i> consists of a finite set of vertices (nodes) V and a finite set of edges E.
/// And  labeling function L which maps L:E -> E the labels to the edges.
/// An edge is a tuple \[v,w\] of vertices in V . The graph is called oriented if the edges have an orientation, i.e. edges are considered
/// to be ordered pairs \[v,w\] in this case.
///
/// Definition: Let <i>A</i> be finite alphabet and X a subset of <i>A</i><sup>+</sup>.
/// We define G(X) = (V (X),E(X), L) as a directed graph. Each word in X as a directed loop
/// from a ROOT vertex to the root vertex. Each vertex represent the position of in the word. The label Each edge the
/// letter of the word at label
/// A with set of vertices V(X) and set of edges E(X)
pub struct CodeGraph {
    e: Vec<Vec<char>>,
}

impl CodeGraph {
    /// Returns a Graph <i>G</i>  associated to a given Code.
    ///
    /// # Arguments
    /// * `x` - A [CircCode](crate::code::CircCode) object.
    ///
    /// # Example
    /// ```
    /// use rust_gcatcirc_lib::code::CircCode;
    /// use rust_gcatcirc_lib::graph_code::CodeGraph;
    ///
    /// fn main() {
    ///     let code = match CircCode::new_from_vec(vec!["ADB".to_string(), "BA".to_string(), "AAD".to_string()]) {
    ///          Ok(code) => code,
    ///          _ => unimplemented!() //No error handling in the example
    ///     };
    ///
    ///     let graph = match CodeGraph::new(&code) {
    ///         Ok(graph) => graph,
    ///         _ => unimplemented!() //No error handling in the example
    ///     };
    /// }
    /// ```
    pub fn new(x: &CircCode) -> Self {
        let mut edges: Vec<Vec<char>> = vec![vec![ROOT]];

        for w in &x.code {
            let w = ROOT.to_string() + w;
            edges.push(w.chars().collect());
        }


        return CodeGraph { e: edges };
    }


    /// Returns if the [CircCode](crate::code::CircCode) represented by <i>G</i> is a code.
    ///
    /// If the orientated graph <i>G(X)</i> associated to a code <i>X</i> has equal walks from root to root the set of words in CircCode is not a code.
    /// Equal walks are walks have to start at root and end at root.
    /// Two walks are considered equal if the concatenation of edge labels walked through is the same for both walks at the end.
    ///
    ///
    /// # Example
    /// ```
    /// use rust_gcatcirc_lib::code::CircCode;
    /// use rust_gcatcirc_lib::graph_code::CodeGraph;
    ///
    /// fn main() {
    ///     let code = match CircCode::new_from_vec(vec!["ADB".to_string(), "BA".to_string(), "AAD".to_string()]) {
    ///          Ok(code) => code,
    ///          _ => unimplemented!() //No error handling in the example
    ///     };
    ///
    ///     let graph = match CodeGraph::new(&code) {
    ///         Ok(graph) => graph,
    ///         _ => unimplemented!() //No error handling in the example
    ///     };
    ///
    ///     if !graph.is_code() {
    ///         println!("{} is is a code", code);
    ///     } else {
    ///         println!("{} is only a set of words (no code)", code);
    ///     }
    /// }
    /// ```
    pub fn is_code(&self) -> bool {
        return self.start_reg_is_code(false, None);
    }

    /// Returns if the represented [CircCode](crate::code::CircCode) is a code and if not so, all ambiguous sequences
    ///
    /// For mor details on whether <i>G</i> is cyclic see [CodeGraph::is_code()].
    ///
    ///
    /// # Example
    /// ```
    /// use rust_gcatcirc_lib::code::CircCode;
    /// use rust_gcatcirc_lib::graph_code::CodeGraph;
    ///
    /// fn main() {
    ///     let code = match CircCode::new_from_vec(vec!["ADB".to_string(), "BA".to_string(), "AAD".to_string()]) {
    ///          Ok(code) => code,
    ///          _ => unimplemented!() //No error handling in the example
    ///     };
    ///
    ///     let graph = match CodeGraph::new(&code) {
    ///         Ok(graph) => graph,
    ///         _ => unimplemented!() //No error handling in the example
    ///     };
    ///
    ///     let (is_code, ambiguous_sequences) =  graph.all_ambiguous_sequences();
    ///     if !is_code {
    ///         print!("{:?}", ambiguous_sequences);
    ///     }
    /// }
    /// ```
    pub fn all_ambiguous_sequences(&self) -> (bool, Vec<String>) {
        let ambiguous_paths = Rc::new(RefCell::new(Vec::new()));
        let res = self.start_reg_is_code(true, Some(ambiguous_paths.clone()));
        return (res, ambiguous_paths.borrow().clone());
    }

    /// Starts the recursive process to check whether a [CircCode](crate::code::CircCode) is a code
    ///
    /// Depending on `find_all_paths` the function terminates either after it has discovered two equal walks in <i>G</i>
    /// or after it has walked all possible paths and found all ambiguous walks.
    ///
    /// # Arguments
    /// * `find_all_paths` a boolean value. If true it walks all possible path and stores all found ambiguous walks into all_paths.
    /// * `all_paths` A reference to an vector of paths. If not none the function stores all found ambiguous walks into the referenced vector.
    fn start_reg_is_code(&self, find_all_paths: bool, all_paths: Option<Rc<RefCell<Vec<String>>>>) -> bool {
        let mut is_code = true;
        for cod_idx_0 in 1..(self.e.len() - 1) {
            for cod_idx_1 in (cod_idx_0 + 1)..self.e.len() {
                if !self.reg_is_code([(cod_idx_0, 0), (cod_idx_1, 0)], vec![], find_all_paths, all_paths.clone(), vec![]) {
                    if !find_all_paths { return false; };
                    is_code = false;
                }
            }
        }

        return is_code;
    }

    /// The recursive process to check whether a [CircCode](crate::code::CircCode) is a code
    ///
    /// Depending on `find_all_paths` the function terminates either after it has discovered two equal walks in <i>G</i>
    /// or after it has walked all possible paths and found all ambiguous walks.
    ///
    /// # Arguments
    /// * `pos` two position tuples of the two so far equal walks. Each tuple can be read as (word index, letter index).
    /// * `history` all edges walked by previous positions.
    /// * `find_all_paths` a boolean value. If true it walks all possible path and stores all found cyclic pathways into `all_paths`.
    /// * `all_paths` A reference to an vector of paths. If not none the function stores all found cyclic pathways into the referenced vector.
    /// * `current_path` the concatenated labels of the walk.
    fn reg_is_code(&self, mut pos: [(usize, usize); 2], mut history: Vec<[(usize, usize); 2]>, find_all_paths: bool, all_paths: Option<Rc<RefCell<Vec<String>>>>, mut current_path: Vec<char>) -> bool {
        let all_paths = match all_paths {
            Some(all_paths) => all_paths,
            _ => Rc::new(RefCell::new(Vec::new())),
        };
        let [mut p0, mut p1] = pos;
        pos.sort();
        if history.contains(&pos) {
            return true;
        }
        history.push(pos);
        for (p0, p1) in [(p0, p1), (p1, p0)] {
            let mut is_code = true;
            if p0 == (0, 0) || self.e[p0.0].len() - 1 == p0.1 {
                for cod_idx in 1..self.e.len() {
                    if !self.reg_is_code([(cod_idx, 0), p1], history.clone(), find_all_paths, Some(all_paths.clone()), current_path.clone()) {
                        if !find_all_paths { return false; };
                        is_code = false;
                    }
                }

                return is_code;
            }
        }


        p0.1 = p0.1 + 1;
        p1.1 = p1.1 + 1;

        if self.e[p0.0][p0.1] == self.e[p1.0][p1.1] {
            current_path.push(self.e[p0.0][p0.1]);
            if self.e[p0.0].len() - 1 == p0.1 && self.e[p1.0].len() - 1 == p1.1 {
                if find_all_paths {
                    let word: String = current_path.iter().map(|x| x.to_string()).collect();
                    all_paths.borrow_mut().push(word);
                }
                return false;
            }
            return self.reg_is_code([p0, p1], history.clone(), find_all_paths, Some(all_paths.clone()), current_path);
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use crate::code::CircCode;
    use crate::graph_code::{CodeGraph, ROOT};

    #[test]
    fn new_graph() {
        {
            let a = match CircCode::new_from_vec(vec!["BDC".to_string(), "CA".to_string(), "DB".to_string()]) {
                Ok(code) => code,
                _ => unimplemented!() //No error handling in the example
            };

            let b = CodeGraph::new(&a);

            assert_eq!(b.e[0], vec![ROOT]);
            assert_eq!(b.e[1], vec![ROOT, 'B', 'D', 'C']);
            assert_eq!(b.e[2], vec![ROOT, 'C', 'A']);
            assert_eq!(b.e[3], vec![ROOT, 'D', 'B']);

            assert_eq!(b.is_code(), true);
        }
    }

    #[test]
    fn is_code_graph() {
        {
            let a = match CircCode::new_from_vec(vec!["BDCC".to_string(), "BD".to_string(), "CC".to_string()]) {
                Ok(code) => code,
                _ => unimplemented!() //No error handling in the example
            };

            let b = CodeGraph::new(&a);

            assert_eq!(b.is_code(), false);
        }
        {
            let a = match CircCode::new_from_vec(vec!["BDADCC".to_string(), "AD".to_string(), "BD".to_string(), "CC".to_string()]) {
                Ok(code) => code,
                _ => unimplemented!() //No error handling in the example
            };

            let b = CodeGraph::new(&a);

            assert_eq!(b.is_code(), false);
        }
        {
            let a = match CircCode::new_from_vec(vec!["BDADA".to_string(), "AD".to_string(), "BD".to_string(), "ACC".to_string(), "CC".to_string()]) {
                Ok(code) => code,
                _ => unimplemented!() //No error handling in the example
            };

            let b = CodeGraph::new(&a);

            assert_eq!(b.is_code(), false);
        }
        {
            let a = match CircCode::new_from_vec(vec!["AC".to_string(), "ACA".to_string(), "CAA".to_string()]) {
                Ok(code) => code,
                _ => unimplemented!() //No error handling in the example
            };

            let b = CodeGraph::new(&a);

            assert_eq!(b.is_code(), true);
        }
    }

    #[test]
    fn ambiguous_sequences_graph() {
        {
            let a = match CircCode::new_from_vec(vec!["BDADCC".to_string(), "AD".to_string(), "BD".to_string(), "CC".to_string(), "ADCC".to_string()]) {
                Ok(code) => code,
                _ => unimplemented!() //No error handling in the example
            };

            let b = CodeGraph::new(&a);
            let (is_code, an_seq) = b.all_ambiguous_sequences();

            assert_eq!(is_code, false);
            assert_eq!(an_seq, vec!["BDADCC".to_string(), "BDADCC".to_string(), "ADCC".to_string()]);
        }
    }
}
