use std::fmt;
use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;
use crate::code;
use code::CircCode;
use crate::graph_circ::elements::{Edge, Vertex};

pub(crate) mod elements;

#[derive(Debug, PartialEq)]
pub enum CircGraphErr {
    VertexErr,
    EmptyCode,
    EdgeErr,
    NoSubErr,
}

impl fmt::Display for CircGraphErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use CircGraphErr::*;
        match self {
            EmptyCode => write!(f, "Empty Code"),
            VertexErr => write!(f, "Vertex Code"),
            EdgeErr => write!(f, "Edge Error"),
            NoSubErr => write!(f, "Graph is no subgraph!"),
        }
    }
}

/// A directed graph <i>G</i> associated to a circular code. A graph <i>G</i> consists of a finite set of vertices (nodes) V and a finite set of edges E.
/// An edge is a tuple \[v,w\] of vertices in V . The graph is called oriented if the edges have an orientation, i.e. edges are considered
/// to be ordered pairs \[v,w\] in this case.
/// Definition: Let <i>A</i> be finite alphabet and X a subset of <i>A</i><sup>+</sup>.
/// We define a directed graph G(X) = (V (X),E(X)) with set of vertices V(X) and set of edges E(X)
/// as follows:
///
/// V (X) = {N1...Ni,Ni+1...Nn : N1N2N3...Nn in X, 0 < i < n}<br>
/// E(X) = {\[N1...Ni,Ni+1...Nn\] : N1N2N3...Nn in X, 0 < i < n}
///
/// The graph G(X) is called the representing graph of X or the graph associated to X.
/// Basically, the graph G(X) associated to a code X interprets n-tuple words from X in (n−1) ways by pairs of i-tuples and (n-i)-tuples for 0 < i < n.
/// See: 2007 E. FIMMEL, C. J. MICHEL, AND L. STRÜNGMANN. N-nucleotide circular codes in graph theory
#[derive(PartialEq)]
pub struct CircGraph {
    /// A reference to the alphabet of the code.
    alphabet: Vec<char>,
    /// The set of vertices <br>
    /// V (X) = {N1...Ni,Ni+1...Nn : N1N2N3...Nn in X, 0 < i < n}
    v: Vec<Rc<elements::Vertex>>,
    /// The set of Edges <br>
    // E(X) = {\[N1...Ni,Ni+1...Nn\] : N1N2N3...Nn in X, 0 < i < n}
    e: Vec<Rc<elements::Edge>>,
}

impl CircGraph {
    /// Returns a Graph <i>G</i>  associated to a given [CircCode](crate::code::CircCode).
    ///
    /// # Arguments
    /// * `x` - A [CircCode](crate::code::CircCode) object.
    ///
    /// # Errors
    /// * `CircGraphErr::EmptyCode` if Graph would be empty
    /// * `CircGraphErr::VertexErr` if a label is off alphabet
    ///
    /// # Example
    /// ```
    /// use rust_gcatcirc_lib::code::CircCode;
    /// use rust_gcatcirc_lib::graph_circ::CircGraph;
    ///
    /// fn main() {
    ///     let code = match CircCode::new_from_vec(vec!["ADB".to_string(), "BA".to_string(), "AAD".to_string()]) {
    ///          Ok(code) => code,
    ///          _ => unimplemented!() //No error handling in the example
    ///     };
    ///
    ///     let graph = match CircGraph::new(&code) {
    ///         Ok(graph) => graph,
    ///         _ => unimplemented!() //No error handling in the example
    ///     };
    /// }
    /// ```
    pub fn new(x: &CircCode) -> Result<Self, CircGraphErr> {
        if x.code.is_empty() {
            return Err(CircGraphErr::EmptyCode);
        }
        let mut g: CircGraph = CircGraph {
            alphabet: x.alphabet.clone(),
            v: vec![],
            e: vec![],
        };

        for w in &x.code {
            g.push_tuple(w.clone())?;
        }
        g.v.sort_by(|a, b| a.index.cmp(&b.index));
        g.e.sort_by(|a, b| a.from.index.cmp(&b.from.index));
        Ok(g)
    }

    /// Returns a subgraph <i>G<sup>*</sup></i> of the graph <i>G</i>  associated to a given Code.
    ///
    /// # Arguments
    /// * `edges` - A subset of Edges object.
    ///
    /// # Errors
    /// * `CircGraphErr::EmptyCode` if Graph would be empty
    /// * `CircGraphErr::VertexErr` if a label is off alphabet
    /// * `CircGraphErr::NoSubErr` if list of edges contains an edge which is not in <i>G</i>
    ///
    /// # Example
    /// ```
    /// use rust_gcatcirc_lib::code::CircCode;
    /// use rust_gcatcirc_lib::graph_circ::CircGraph;
    ///
    /// fn main() {
    ///     let code = match CircCode::new_from_vec(vec!["ADB".to_string(), "BA".to_string(), "AAD".to_string()]) {
    ///          Ok(code) => code,
    ///          _ => unimplemented!() //No error handling in the example
    ///     };
    ///
    ///     let graph = match CircGraph::new(&code) {
    ///         Ok(graph) => graph,
    ///         _ => unimplemented!() //No error handling in the example
    ///     };
    ///
    ///     let (is_cyclic, edges) =  graph.all_cycles();
    ///     if is_cyclic {
    ///         let subgraph = match graph.subgraph_from_list_of_edges(edges[0].clone()) {
    ///             Ok(graph) => graph,
    ///             _ => unimplemented!() //No error handling in the example
    ///         };
    ///     }
    /// }
    /// ```
    pub fn subgraph_from_list_of_edges(&self, edges: Vec<Rc<Edge>>) -> Result<Self, CircGraphErr> {
        let mut g: CircGraph = CircGraph {
            alphabet: self.alphabet.clone(),
            v: vec![],
            e: vec![],
        };

        for e in &edges {
            if self.e.contains(e) {
                let v_to = Rc::new((*e.to).clone());
                let v_from = Rc::new((*e.from).clone());
                g.v.push(v_to.clone());
                g.v.push(v_from.clone());
                g.push_edge(v_from, v_to);
            } else {
                return Err(CircGraphErr::NoSubErr);
            }
        }

        return Ok(g);
    }

    /// Returns a component <i>G<sup>i</sup></i> of the graph <i>G</i>  associated to a given Code.
    ///
    /// The component (subgraph) taht contains only  the edges and vertices for one give 0 < i < n.
    ///
    /// V<sup>i</sup>(X) = {N1...Ni,Ni+1...Nn : N1N2N3...Nn in X}<br>
    /// E<sup>i</sup>(X) = {\[N1...Ni,Ni+1...Nn\] : N1N2N3...Nn in X}
    ///
    /// # Arguments
    /// * `i` - a positive integer to specify the component.
    ///
    /// # Errors
    /// * `CircGraphErr::EmptyCode` if Graph would be empty
    ///
    /// # Example
    /// ```
    /// use rust_gcatcirc_lib::code::CircCode;
    /// use rust_gcatcirc_lib::graph_circ::CircGraph;
    ///
    /// fn main() {
    ///     let code = match CircCode::new_from_vec(vec!["ADBD".to_string(), "BADD".to_string(), "AAAD".to_string()]) {
    ///          Ok(code) => code,
    ///          _ => unimplemented!() //No error handling in the example
    ///     };
    ///
    ///     let graph = match CircGraph::new(&code) {
    ///         Ok(graph) => graph,
    ///         _ => unimplemented!() //No error handling in the example
    ///     };
    ///
    ///     let subgraph = match graph.component(1) {
    ///         Ok(graph) => graph,
    ///         _ => unimplemented!() //No error handling in the example
    ///     };
    /// }
    /// ```
    pub fn component(&self, i: u32) -> Result<Self, CircGraphErr> {
        let mut g: CircGraph = CircGraph {
            alphabet: self.alphabet.clone(),
            v: vec![],
            e: vec![],
        };

        for e in &self.e {
            if e.to.label.len() == i as usize || e.from.label.len() == i as usize {
                let v_to = Rc::new((*e.to).clone());
                let v_from = Rc::new((*e.from).clone());
                g.v.push(v_to.clone());
                g.v.push(v_from.clone());
                g.push_edge(v_from, v_to);
            }
        }

        if g.e.is_empty() {
            return Err(CircGraphErr::EmptyCode);
        }

        return Ok(g);
    }

    /// Returns if the orientated graph <i>G</i> is cyclic
    ///
    /// If the orientated graph <i>G(X)</i> associated to a code <i>X</i> is cyclic, i.e., the graph contains at least one
    /// orientated cyclic path it must follow that the code <i>X</i> is non-circular. Even more precise, one can say
    /// that <i>X</i> is circular if and only if <i>G(X)</i> is acyclic.
    ///
    ///
    /// # Example
    /// ```
    /// use rust_gcatcirc_lib::code::CircCode;
    /// use rust_gcatcirc_lib::graph_circ::CircGraph;
    ///
    /// fn main() {
    ///     let code = match CircCode::new_from_vec(vec!["ADBD".to_string(), "BADD".to_string(), "AAAD".to_string()]) {
    ///          Ok(code) => code,
    ///          _ => unimplemented!() //No error handling in the example
    ///     };
    ///
    ///     let graph = match CircGraph::new(&code) {
    ///         Ok(graph) => graph,
    ///         _ => unimplemented!() //No error handling in the example
    ///     };
    ///
    ///     if !graph.is_cyclic() {
    ///         println!("{} is circular", code);
    ///     } else {
    ///         println!("{} is non-circular", code);
    ///     }
    /// }
    /// ```
    pub fn is_cyclic(&self) -> bool {
        return self.start_reg_is_cyclic(false, None);
    }

    /// Returns if the all longest paths in the graph <i>G</i>
    ///
    /// If <i>G</i> is cyclic it returns None. For mor details on whether <i>G</i> is cyclic see [CircGraph::is_cyclic()].
    ///
    ///
    /// # Example
    /// ```
    /// use rust_gcatcirc_lib::code::CircCode;
    /// use rust_gcatcirc_lib::graph_circ::CircGraph;
    ///
    /// fn main() {
    ///     let code = match CircCode::new_from_vec(vec!["ADB".to_string(), "BC".to_string(), "AAD".to_string()]) {
    ///          Ok(code) => code,
    ///          _ => unimplemented!() //No error handling in the example
    ///     };
    ///
    ///     let graph = match CircGraph::new(&code) {
    ///         Ok(graph) => graph,
    ///         _ => unimplemented!() //No error handling in the example
    ///     };
    ///
    ///     if let longest_paths = graph.all_longest_paths() {
    ///         let subgraph = match graph.subgraph_from_list_of_edges(longest_paths[0].clone()) {
    ///             Ok(graph) => graph,
    ///             _ => unimplemented!() //No error handling in the example
    ///         };
    ///     }
    /// }
    /// ```
    pub fn all_longest_paths(&self) -> Option<Vec<Vec<Rc<elements::Edge>>>> {
        if self.is_cyclic() {return None}
        let start_edges = self.get_path_start_edges();
        let all_paths: Rc<RefCell<Vec<Vec<Rc<elements::Edge>>>>> = Rc::new(RefCell::new(Vec::new()));
        for e in start_edges {
            self.rec_find_all_longest_paths(vec![e], all_paths.clone());
        }

        let mut all_paths = all_paths.borrow_mut().clone();
        all_paths.sort_by(|x, y| x.len().cmp(&y.len()));
        let last_path_len = all_paths.last().unwrap().len();
        return Some(all_paths.into_iter().filter(|x| x.len() == last_path_len ).collect());
    }

    fn rec_find_all_longest_paths(&self, current_path: Vec<Rc<elements::Edge>>,all_paths: Rc<RefCell<Vec<Vec<Rc<elements::Edge>>>>>) {
        if let Some(current_pos) = current_path.last() {
            let targets = self.get_all_outgoing_edges_of_vertices(&vec![&current_pos.to]);
            for t in targets {
                let mut current_path = current_path.clone();
                current_path.push(t.clone());
                self.rec_find_all_longest_paths(current_path, all_paths.clone());
            }

            all_paths.borrow_mut().push(current_path);
        }
    }

    /// This function does the same as [CircGraph::all_longest_paths()], it just formats the return type.
    ///
    /// # Example
    /// ```
    /// use rust_gcatcirc_lib::code::CircCode;
    /// use rust_gcatcirc_lib::graph_circ::CircGraph;
    ///
    /// fn main() {
    ///     let code = match CircCode::new_from_vec(vec!["ADB".to_string(), "BA".to_string(), "AAD".to_string()]) {
    ///          Ok(code) => code,
    ///          _ => unimplemented!() //No error handling in the example
    ///     };
    ///
    ///     let graph = match CircGraph::new(&code) {
    ///         Ok(graph) => graph,
    ///         _ => unimplemented!() //No error handling in the example
    ///     };
    ///
    ///     if let Some(longest_paths) =  graph.all_longest_paths_as_string_vec() {
    ///         todo!();
    ///     }
    ///
    /// }
    /// ```
    pub fn all_longest_paths_as_string_vec(&self) -> Option<Vec<String>> {
        if let Some(all_cycles) = self.all_longest_paths() {
            return Some(all_cycles.into_iter().map(|x| Self::path_as_string(&x)).collect())
        }
        return None
    }

    /// This function does the same as [CircGraph::all_longest_paths()], it just formats the return type.
    ///
    ///
    ///
    /// # Errors
    /// * `CircGraphErr::EmptyCode` if Graph would be empty
    /// * `CircGraphErr::VertexErr` if a label is off alphabet
    /// * `CircGraphErr::NoSubErr` if list of edges contains an edge which is not in <i>G</i>
    ///
    /// # Example
    /// ```
    /// use rust_gcatcirc_lib::code::CircCode;
    /// use rust_gcatcirc_lib::graph_circ::CircGraph;
    ///
    /// fn main() {
    ///     let code = match CircCode::new_from_vec(vec!["ADB".to_string(), "BA".to_string(), "AAD".to_string()]) {
    ///          Ok(code) => code,
    ///          _ => unimplemented!() //No error handling in the example
    ///     };
    ///
    ///     let graph = match CircGraph::new(&code) {
    ///         Ok(graph) => graph,
    ///         _ => unimplemented!() //No error handling in the example
    ///     };
    ///
    ///     let subgraph = match graph.all_longest_paths_as_sub_graph()  {
    ///         Ok(graph) => graph,
    ///         _ => unimplemented!() //No error handling in the example
    ///     };
    ///
    /// }
    /// ```
    pub fn all_longest_paths_as_sub_graph(&self) -> Result<Self, CircGraphErr> {
        if let Some(all_cycles) = self.all_longest_paths() {
            let all_cycles = all_cycles.into_iter().flatten().collect();
            return self.subgraph_from_list_of_edges(all_cycles);
        }

        return Err(CircGraphErr::EmptyCode);
    }

    /// This function does the same as [CircGraph::all_longest_paths()], it just formats the return type.
    ///
    /// # Example
    /// ```
    /// use rust_gcatcirc_lib::code::CircCode;
    /// use rust_gcatcirc_lib::graph_circ::CircGraph;
    ///
    /// fn main() {
    ///     let code = match CircCode::new_from_vec(vec!["ADB".to_string(), "BA".to_string(), "AAD".to_string()]) {
    ///          Ok(code) => code,
    ///          _ => unimplemented!() //No error handling in the example
    ///     };
    ///
    ///     let graph = match CircGraph::new(&code) {
    ///         Ok(graph) => graph,
    ///         _ => unimplemented!() //No error handling in the example
    ///     };
    ///
    ///     if let Some(longest_paths) =  graph.all_longest_paths_as_vertex_vec() {
    ///         todo!();
    ///     }
    ///
    /// }
    /// ```
    pub fn all_longest_paths_as_vertex_vec(&self) -> Option<Vec<Vec<String>>> {
        if let Some(all_cycles) = self.all_longest_paths() {
            return Some(all_cycles.into_iter().map(|x| Self::path_as_vertex_vec(&x)).collect())
        }
        return None
    }

    /// Returns if the orientated graph <i>G</i> is cyclic and if so, all cyclic paths
    ///
    /// For mor details on whether <i>G</i> is cyclic see [CircGraph::is_cyclic()].
    ///
    ///
    /// # Example
    /// ```
    /// use rust_gcatcirc_lib::code::CircCode;
    /// use rust_gcatcirc_lib::graph_circ::CircGraph;
    ///
    /// fn main() {
    ///     let code = match CircCode::new_from_vec(vec!["ADB".to_string(), "BA".to_string(), "AAD".to_string()]) {
    ///          Ok(code) => code,
    ///          _ => unimplemented!() //No error handling in the example
    ///     };
    ///
    ///     let graph = match CircGraph::new(&code) {
    ///         Ok(graph) => graph,
    ///         _ => unimplemented!() //No error handling in the example
    ///     };
    ///
    ///     let (is_cyclic, cyclic_paths) =  graph.all_cycles();
    ///     if is_cyclic {
    ///         let subgraph = match graph.subgraph_from_list_of_edges(cyclic_paths[0].clone()) {
    ///             Ok(graph) => graph,
    ///             _ => unimplemented!() //No error handling in the example
    ///         };
    ///     }
    /// }
    /// ```
    pub fn all_cycles(&self) -> (bool, Vec<Vec<Rc<elements::Edge>>>) {
        let all_cycles = Rc::new(RefCell::new(Vec::new()));
        let res = self.start_reg_is_cyclic(true, Some(all_cycles.clone()));
        let mut all_cycles = all_cycles.borrow_mut().clone();
        all_cycles.sort_by(|x, y| x.len().cmp(&y.len()));
        return (res, all_cycles);
    }

    /// This function does the same as [CircGraph::all_cycles()], it just formats the return type.
    ///
    /// # Example
    /// ```
    /// use rust_gcatcirc_lib::code::CircCode;
    /// use rust_gcatcirc_lib::graph_circ::CircGraph;
    ///
    /// fn main() {
    ///     let code = match CircCode::new_from_vec(vec!["ADB".to_string(), "BA".to_string(), "AAD".to_string()]) {
    ///          Ok(code) => code,
    ///          _ => unimplemented!() //No error handling in the example
    ///     };
    ///
    ///     let graph = match CircGraph::new(&code) {
    ///         Ok(graph) => graph,
    ///         _ => unimplemented!() //No error handling in the example
    ///     };
    ///
    ///     let (is_cyclic, cyclic_paths) =  graph.all_cycles_as_string_vec();
    ///     if is_cyclic {
    ///         for cyclic_path in cyclic_paths {
    ///             println!("{}", cyclic_path)
    ///         }
    ///     }
    /// }
    /// ```
    pub fn all_cycles_as_string_vec(&self) -> (bool, Vec<String>) {
        let (res, all_cycles) = self.all_cycles();
        return (res, all_cycles.into_iter().map(|x| Self::path_as_string(&x)).collect());
    }

    /// This function does the same as [CircGraph::all_cycles()], it just formats the return type.
    ///
    ///
    ///
    /// # Errors
    /// * `CircGraphErr::EmptyCode` if Graph would be empty
    /// * `CircGraphErr::VertexErr` if a label is off alphabet
    /// * `CircGraphErr::NoSubErr` if list of edges contains an edge which is not in <i>G</i>
    ///
    /// # Example
    /// ```
    /// use rust_gcatcirc_lib::code::CircCode;
    /// use rust_gcatcirc_lib::graph_circ::CircGraph;
    ///
    /// fn main() {
    ///     let code = match CircCode::new_from_vec(vec!["ADB".to_string(), "BA".to_string(), "AAD".to_string()]) {
    ///          Ok(code) => code,
    ///          _ => unimplemented!() //No error handling in the example
    ///     };
    ///
    ///     let graph = match CircGraph::new(&code) {
    ///         Ok(graph) => graph,
    ///         _ => unimplemented!() //No error handling in the example
    ///     };
    ///
    ///     let (is_cyclic, subgraph) =  graph.all_cycles_as_sub_graph();
    ///
    /// }
    /// ```
    pub fn all_cycles_as_sub_graph(&self) -> Result<(bool, Self), CircGraphErr> {
        let (res, all_cycles) = self.all_cycles();
        let all_cycles = all_cycles.into_iter().flatten().collect();
        let graph = self.subgraph_from_list_of_edges(all_cycles)?;
        return Ok((res, graph));
    }


    /// This function does the same as [CircGraph::all_cycles()], it just formats the return type.
    ///
    /// # Example
    /// ```
    /// use rust_gcatcirc_lib::code::CircCode;
    /// use rust_gcatcirc_lib::graph_circ::CircGraph;
    ///
    /// fn main() {
    ///     let code = match CircCode::new_from_vec(vec!["ADB".to_string(), "BA".to_string(), "AAD".to_string()]) {
    ///          Ok(code) => code,
    ///          _ => unimplemented!() //No error handling in the example
    ///     };
    ///
    ///     let graph = match CircGraph::new(&code) {
    ///         Ok(graph) => graph,
    ///         _ => unimplemented!() //No error handling in the example
    ///     };
    ///
    ///     let (is_cyclic, cyclic_paths) =  graph.all_cycles_as_vertex_vec();
    ///
    /// }
    /// ```
    pub fn all_cycles_as_vertex_vec(&self) -> (bool, Vec<Vec<String>>) {
        let (res, all_cycles) = self.all_cycles();
        return (res, all_cycles.into_iter().map(|x| Self::path_as_vertex_vec(&x)).collect());
    }

    /// Starts the recursive process to check whether the graph is cyclic
    ///
    /// Depending on `find_all_paths` the function terminates either after it has discovered on cyclic path in <i>G</i>
    /// or after it has walked all possible paths.
    ///
    /// # Arguments
    /// * `find_all_paths` a boolean value. If true it walks all possible path and stores all found cyclic pathways into all_paths.
    /// * `all_paths` A reference to an vector of paths. If not none the function stores all found cyclic pathways into the referenced vector.
    fn start_reg_is_cyclic(&self, find_all_paths: bool, all_paths: Option<Rc<RefCell<Vec<Vec<Rc<elements::Edge>>>>>>) -> bool {
        let visited_edges = Rc::new(RefCell::new(vec![]));

        let all_paths = match all_paths {
            Some(all_paths) => all_paths,
            None => Rc::new(RefCell::new(Vec::new())),
        };

        let mut start_edges = self.get_path_start_edges();
        start_edges.append(&mut self.e.clone());
        let is_acyclic = Rc::new(RefCell::new(false));
        for vertex in start_edges {
            if !visited_edges.borrow().contains(&vertex) {
                visited_edges.borrow_mut().push(vertex.clone());
                if self.reg_is_cyclic(vec![vertex.clone()], visited_edges.clone(), is_acyclic.clone(), find_all_paths, all_paths.clone()) {
                    if !find_all_paths { return true; };
                    *is_acyclic.borrow_mut() = true;
                }
            }
        }

        return *is_acyclic.borrow().deref();
    }

    /// The recursive process to check whether the graph is cyclic
    ///
    /// Depending on `find_all_paths` the function terminates either after it has discovered on cyclic path in <i>G</i>
    /// or after it has walked all possible paths.
    ///
    /// # Arguments
    /// * `current_path` the edges that have been walked by the previous steps .
    /// * `visited_edges` the edges that have been walked by all previous steps (not just the curren path).
    /// * `is_acyclic` boolean if the graph is acyclic. Only used if `find_all_paths` is true.
    /// * `find_all_paths` a boolean value. If true it walks all possible path and stores all found cyclic pathways into `all_paths`.
    /// * `all_paths` A reference to an vector of paths. If not none the function stores all found cyclic pathways into the referenced vector.
    fn reg_is_cyclic(&self, current_path: Vec<Rc<elements::Edge>>, visited_edges: Rc<RefCell<Vec<Rc<elements::Edge>>>>, is_acyclic: Rc<RefCell<bool>>, find_all_paths: bool, all_paths: Rc<RefCell<Vec<Vec<Rc<elements::Edge>>>>>) -> bool {
        if !find_all_paths && *is_acyclic.borrow() {
            return true;
        }

        let current_pos = match current_path.last() {
            Some(current_pos) => current_pos,
            None => return true,
        };

        // println!("current_path: {:?}", CircGraph::path_as_string(&current_path));
        let end_pos = current_path.iter().position(|edge| edge.from.eq(&current_pos.to));
        if end_pos.is_some() || current_pos.from == current_pos.to {
            if find_all_paths {
                let mut c_path: Vec<Rc<Edge>>;
                if current_pos.from == current_pos.to {
                    c_path = vec![current_pos.clone()];
                } else if let Some(end_pos) = end_pos {
                    let mut res = u32::MAX;
                    let mut min_idx = 0;
                    c_path = current_path.iter().skip(end_pos).enumerate().map(|edge| {
                        if res > edge.1.from.index as u32 {
                            res = edge.1.from.index as u32;
                            min_idx = edge.0;
                        };
                        edge.1.clone()
                    }).collect();
                    c_path.rotate_left(min_idx);
                } else { c_path = vec![]; }

                // println!("cyclic path in : {:?}", CircGraph::path_as_string(& c_path));
                if !all_paths.borrow_mut().contains(&c_path) {
                    all_paths.borrow_mut().push(c_path);
                }
            };

            *is_acyclic.borrow_mut() = true;
            return true;
        }

        let targets = self.get_all_outgoing_edges_of_vertices(&vec![&current_pos.to]);

        for edge in targets {
            if !visited_edges.borrow().contains(&edge) {
                visited_edges.borrow_mut().push(edge.clone());
            }
            let mut new_path = current_path.clone();
            new_path.push(edge.clone());

            let res = self.reg_is_cyclic(new_path, visited_edges.clone(), is_acyclic.clone(), find_all_paths, all_paths.clone());
            if res && !find_all_paths {
                return true;
            }
        }

        return *is_acyclic.borrow().deref();
    }


    /// Returns a vector the vertices of a vector of edges.
    ///
    /// # Arguments
    /// `edges` Vector of edges. Make sure that the edges are in the correct order.
    fn path_as_vertex_vec(edges: &Vec<Rc<Edge>>) -> Vec<String> {
        let mut res = edges.iter().map(|x| x.from.to_string()).collect::<Vec<String>>();
        res.push(edges.last().unwrap().to.to_string());
        return res;
    }

    /// Returns a path as string.
    ///
    /// # Arguments
    /// * `edges` Vector of edges. Make sure that the edges are in the correct order.
    fn path_as_string(edges: &Vec<Rc<Edge>>) -> String {
        return Self::path_as_vertex_vec(edges).join(" -> ");
    }

    /// Adds a tuple <i>w</i> to the Graph
    ///
    /// This function adds all edges for on tuple, i.e., all pairs of i-tuples and (n-i)-tuples for 0 < i < n
    /// V(X) = {N1...Ni,Ni+1...Nn : N1N2N3...Nn = <i>w</i>, , 0 < i < n}<br>
    ///  E(X) = {\[N1...Ni,Ni+1...Nn\] : N1N2N3...Nn <i>w</i>, 0 < i < n}
    ///
    /// # Arguments
    /// * `w` a tuple in <i>X</i> as String.
    fn push_tuple(&mut self, w: String) -> Result<(), CircGraphErr> {
        for s in 1..w.len() {
            let (prefix, suffix) = w.split_at(s);
            let v1 = self.push_vertex(prefix.to_string())?;
            let v2 = self.push_vertex(suffix.to_string())?;
            self.push_edge(v1, v2);
        }

        return Ok(());
    }

    /// Adds one orientated edge from <i>v1</i> to <i>v1</i> to the Graph
    ///
    /// # Arguments
    /// * `v1` outgoing Vertex
    /// * `v2` ingoing Vertex
    fn push_edge(&mut self, v1: Rc<Vertex>, v2: Rc<Vertex>) {
        let new_edge = elements::Edge::new(v1, v2);
        self.e.push(new_edge);
    }

    /// Adds a new vertex to the Graph if id does not exits.
    /// It returns a reference to the vertex, either the new one or the
    /// existing one wit the same label.
    ///
    /// # Error
    /// * `CircGraphErr::VertexErr` if label is off alphabet
    ///
    /// # Arguments
    /// * `label` the label of the vertex as String
    fn push_vertex(&mut self, label: String) -> Result<Rc<Vertex>, CircGraphErr> {
        let v_res = elements::Vertex::new(label, &self.alphabet);
        let v1: Rc<Vertex> = match v_res {
            Ok(new_v1) => new_v1,
            _ => return Err(CircGraphErr::VertexErr),
        };

        match self.v.iter().position(|c| c == &v1) {
            Some(idx) => return Ok(Rc::clone(self.v.get(idx).unwrap())),
            None => {
                self.v.push(v1);
                return Ok(Rc::clone(self.v.last().unwrap()));
            }
        }
    }

    /// Returns all outgoing edges of all vertices with no ingoing edges.
    fn get_path_start_edges(&self) -> Vec<Rc<Edge>> {
        let mut path_start_vertices = vec![];
        for vertex in &self.v {
            let mut has_no_incoming = true;
            for edge in &self.e {
                if edge.to.eq(vertex) {
                    has_no_incoming = false;
                    break;
                }
            }

            if has_no_incoming {
                path_start_vertices.push(vertex);
            }
        }

        return self.get_all_outgoing_edges_of_vertices(&path_start_vertices);
    }

    /// Returns all outgoing edges of all vertices in path_start_vertices `path_start_vertices`.
    ///
    /// # Arguments
    /// * `path_start_vertices` is a list of vertices.
    fn get_all_outgoing_edges_of_vertices(&self, path_start_vertices: &Vec<&Rc<elements::Vertex>>) -> Vec<Rc<Edge>> {
        return self.e.iter().filter(|edge| path_start_vertices.contains(&&edge.from)).map(|edge| edge.clone()).collect();
    }
}

impl fmt::Display for CircGraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Graph")
    }
}


#[cfg(test)]
mod tests {
    use crate::code::CircCode;
    use crate::graph_circ::{CircGraph, CircGraphErr};


    #[test]
    fn new_graph() {
        let code = match CircCode::new_from_vec(vec!["ABB".to_string(), "AB".to_string(), "AAB".to_string()]) {
            Ok(code) => code,
            _ => unimplemented!()
        };

        let graph = match CircGraph::new(&code) {
            Ok(graph) => graph,
            _ => unimplemented!()
        };

        assert_eq!(graph.v.iter().map(|x| x.label.clone()).collect::<Vec<String>>(), vec!["A", "B", "AA", "AB", "BB"])
    }

    #[test]
    fn is_acyclic() {
        {
            let code = match CircCode::new_from_vec(vec!["ABB".to_string(), "AB".to_string(), "AAB".to_string()]) {
                Ok(code) => code,
                _ => unimplemented!()
            };

            let graph = match CircGraph::new(&code) {
                Ok(graph) => graph,
                _ => unimplemented!()
            };

            assert_eq!(graph.is_cyclic(), false);
        }
        {
            let code = match CircCode::new_from_vec(vec!["ABB".to_string(), "BA".to_string(), "AAB".to_string()]) {
                Ok(code) => code,
                _ => unimplemented!()
            };

            let graph = match CircGraph::new(&code) {
                Ok(graph) => graph,
                _ => unimplemented!()
            };

            assert_eq!(graph.is_cyclic(), true);
        }
    }

    #[test]
    fn get_all_cyclic() {
        {
            let code = match CircCode::new_from_vec(vec!["ADB".to_string(), "BA".to_string(), "AAD".to_string()]) {
                Ok(code) => code,
                _ => unimplemented!()
            };

            let graph = match CircGraph::new(&code) {
                Ok(graph) => graph,
                _ => unimplemented!()
            };

            let (res, cycles) = graph.all_cycles();

            assert_eq!(res, true);
            assert_eq!(cycles.len(), 1);
        }
        {
            let code = match CircCode::new_from_vec(vec!["ADB".to_string(), "BA".to_string(), "AAD".to_string(), "DAA".to_string()]) {
                Ok(code) => code,
                _ => unimplemented!()
            };

            let graph = match CircGraph::new(&code) {
                Ok(graph) => graph,
                _ => unimplemented!()
            };

            let (res, cycles) = graph.all_cycles();

            assert_eq!(res, true);
            assert_eq!(cycles.len(), 2);

            let (res, cycles_string) = graph.all_cycles_as_string_vec();
            assert_eq!(res, true);
            assert_eq!(cycles_string.len(), 2);
            assert_eq!(cycles_string[0], "D -> AA -> D");
            assert_eq!(cycles_string[1], "A -> AD -> B -> A");

            let (res, cycles_string) = graph.all_cycles_as_vertex_vec();
            assert_eq!(res, true);
            assert_eq!(cycles_string[0], vec!["D", "AA", "D"]);
            assert_eq!(cycles_string[1], vec!["A", "AD", "B", "A"]);

            let new_graph = match graph.subgraph_from_list_of_edges(cycles[0].clone()) {
                Ok(graph) => graph,
                _ => unimplemented!()
            };
            assert_eq!(new_graph.e, cycles[0]);

            let (_, new_graph) = match graph.all_cycles_as_sub_graph() {
                Ok(graph) => graph,
                _ => unimplemented!()
            };

            assert_eq!(new_graph.e.len(), 5);
        }
        {
            let code = match CircCode::new_from_vec(vec!["ACB".to_string(), "BDC".to_string(), "ABC".to_string(), "DDC".to_string(), "BAA".to_string(), "BBB".to_string(), "BDA".to_string(), "ACD".to_string(), "ADA".to_string(), "BBC".to_string(), "DDB".to_string(), "AAD".to_string(), "CDC".to_string(), "ADC".to_string(), "CAD".to_string(), "CBD".to_string(), "ACA".to_string(), "BCA".to_string(), "CCD".to_string(), "DCD".to_string(), "ABA".to_string(), "BCC".to_string(), "ADB".to_string(), "CAA".to_string(), "DCB".to_string(), "DBB".to_string(), "CBA".to_string(), "CDD".to_string(), "DAD".to_string(), "CDB".to_string()]) {
                Ok(code) => code,
                _ => unimplemented!()
            };

            let graph = match CircGraph::new(&code) {
                Ok(graph) => graph,
                _ => unimplemented!()
            };

            let (res, cycles) = graph.all_cycles();

            assert_eq!(cycles.len(), 838);

            assert_eq!(res, true);
        }
    }

    #[test]
    fn component() {
        let code = match CircCode::new_from_vec(vec!["ADBD".to_string(), "BADD".to_string(), "AAAD".to_string()]) {
            Ok(code) => code,
            _ => unimplemented!() //No error handling in the example
        };

        let graph = match CircGraph::new(&code) {
            Ok(graph) => graph,
            _ => unimplemented!() //No error handling in the example
        };
        let subgraph = match graph.component(1) {
            Ok(graph) => graph,
            _ => unimplemented!() //No error handling in the example
        };

        assert_eq!(subgraph.e.len(), 6);

        let subgraph = match graph.component(5) {
            Err(e) => e,
            _ => unimplemented!()
        };
        assert_eq!(subgraph, CircGraphErr::EmptyCode);
    }

    #[test]
    fn all_longest_paths() {
        let code = match CircCode::new_from_vec(vec!["ABC".to_string(), "BCD".to_string(), "DEF".to_string(), "EFG".to_string()]) {
            Ok(code) => code,
            _ => unimplemented!() //No error handling in the example
        };

        let graph = match CircGraph::new(&code) {
            Ok(graph) => graph,
            _ => unimplemented!() //No error handling in the example
        };

        let a = graph.all_longest_paths().unwrap();
        assert_eq!(a[0].len(), 4);

        let code  = CircCode::new_from_vec(vec!["AAC".to_string(), "AAG".to_string(), "AAT".to_string(), "ACC".to_string(), "ACG".to_string(), "ACT".to_string(), "AGC".to_string(), "AGG".to_string(), "AGT".to_string(), "ATT".to_string(), "CCG".to_string(), "CCT".to_string(), "CGG".to_string(), "CGT".to_string(), "CTT".to_string(), "GCT".to_string(), "GGT".to_string(), "GTT".to_string(), "TCA".to_string(), "TGA".to_string()]).unwrap_or_default();

        let graph = match CircGraph::new(&code) {
            Ok(graph) => graph,
            _ => unimplemented!() //No error handling in the example
        };

        let a = graph.all_longest_paths().unwrap();
        assert_eq!(a.len(), 16);
        assert_eq!(a[0].len(), 8);

        let code  = CircCode::new_from_vec(vec!["AAC".to_string(), "CAA".to_string()]).unwrap_or_default();

        let graph = match CircGraph::new(&code) {
            Ok(graph) => graph,
            _ => unimplemented!() //No error handling in the example
        };

        assert_eq!(graph.all_longest_paths(), None);
    }
}
