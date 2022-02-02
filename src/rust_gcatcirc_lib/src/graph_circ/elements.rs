use std::rc::Rc;
use std::fmt;


pub enum GraphElementsErr {
    NotInAlphabet
}

impl fmt::Display for GraphElementsErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use GraphElementsErr::*;
        match self {
            NotInAlphabet => write!(f, "Wrong word nod in alphabet!"),
        }
    }
}

/// A vertex or node in a [CircGraph](crate::graph_circ::CircGraph)
#[derive(Debug, Clone)]
pub struct Vertex {
    /// Label of the vertex
    pub(crate) label: String,
    /// Index calculated based on the vertex. This value is used to compare and order the vertices
    pub(crate) index: i32,
}

impl fmt::Display for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label)
    }
}

impl PartialEq<Vertex> for Vertex {
    fn eq(&self, other: &Vertex) -> bool {
        self.index == other.index
    }
}

/// Returns an index based on the label of a vertex.
///
/// # Errors
/// * `NotInAlphabet` if label is off alphabet
///
/// # Arguments
/// * `label` the label of the vertex.
/// * `alphabet` the alphabet over all vertices in the graph.
fn calculate_idx(label: &String, alphabet: &Vec<char>) -> Result<i32, GraphElementsErr> {
    let mut pos = 1;
    let mut index = 0;
    for l in label.chars() {
        let pos_val = match alphabet.iter().position(|&c| c == l) {
            Some(val) => val + 1,
            None => return Err(GraphElementsErr::NotInAlphabet)
        };

        index += pos_val as i32 * pos;
        pos *= alphabet.len() as i32 + 1;
    }

    return Ok(index);
}

impl Vertex {
    /// Returns a Rc reference to a new vertex
    ///
    /// # Errors
    /// * `NotInAlphabet` if label is off alphabet
    ///
    /// # Arguments
    /// * `label` the label of the vertex.
    /// * `alphabet` the alphabet over all vertices in the graph.
    pub(crate) fn new(label: String, alphabet: &Vec<char>) -> Result<Rc<Self>, GraphElementsErr> {
        let index = calculate_idx(&label, alphabet)?;
        let new_self = Rc::new(Self { label: label, index: index });
        Ok(new_self)
    }
}


/// An edge or arc in a [CircGraph](crate::graph_circ::CircGraph)
#[derive(Debug)]
pub struct Edge {
    /// Origen vertex
    pub(crate) from: Rc<Vertex>,
    /// Target vertex
    pub(crate) to: Rc<Vertex>,
    /// Word: concatenation of from label and to label
    pub(crate) label: String,
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -{}-> {}", self.from, self.label, self.to)
    }
}


impl PartialEq<Edge> for Edge {
    fn eq(&self, other: &Edge) -> bool {
        self.to.index == other.to.index && self.from.index == other.from.index
    }
}

impl Edge {
    /// Returns a Rc reference to a new edge between v1 and v2
    ///
    /// # Arguments
    /// * `v1` Rc reference to the from vertex
    /// * `v2` Rc reference to the target vertex
    pub(crate) fn new(v1: Rc<Vertex>, v2: Rc<Vertex>) -> Rc<Self> {
        let w = v1.label.clone() + &v2.label.clone();
        Rc::new(Self { from: v1, to: v2, label: w })
    }
}


#[cfg(test)]
mod tests {
    use crate::graph_circ::elements::{Edge, Vertex};

    #[test]
    fn new_vertex() {
        {
            let al = vec!['A', 'B'];
            let label = "AB".to_string();
            let v = match Vertex::new(label, &al) {
                Ok(ver) => ver,
                _ => unimplemented!(),
            };

            assert_eq!(v.index, 7);
        }
        {
            let al = vec!['A', 'B', 'C'];
            let label = "CAB".to_string();
            let v = match Vertex::new(label, &al) {
                Ok(ver) => ver,
                _ => unimplemented!(),
            };

            assert_eq!(v.index, 3 * 1 + 1 * 4 + 2 * 16);
        }
        {
            let al = vec!['A', 'B'];
            let label = "CAB".to_string();
            let res = match Vertex::new(label, &al) {
                Err(e) => e,
                _ => unimplemented!(),
            };

            assert_eq!(res.to_string(), "Wrong word nod in alphabet!");
        }
    }

    #[test]
    fn new_edge() {
        {
            let al = vec!['A', 'B'];
            let label = "AB".to_string();
            let v1 = match Vertex::new(label, &al) {
                Ok(ver) => ver,
                _ => unimplemented!(),
            };

            let label = "ABB".to_string();
            let v2 = match Vertex::new(label, &al) {
                Ok(ver) => ver,
                _ => unimplemented!(),
            };

            let e = Edge::new(v1, v2);
            assert_eq!(e.label, "ABABB");

        }
        {
            let al = vec!['A', 'B', 'C'];
            let label = "CAB".to_string();
            let v = match Vertex::new(label, &al) {
                Ok(ver) => ver,
                _ => unimplemented!(),
            };

            assert_eq!(v.index, 3 * 1 + 1 * 4 + 2 * 16);
        }
    }
}