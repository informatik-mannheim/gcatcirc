use extendr_api::prelude::*;
use rust_gcatcirc_lib::graph_circ::CircGraph;

use crate::lib_utils::new_code_from_vec;


/// Returns the graph associated to a code
///
/// @param tuples a gcatbase::gcat.code object
/// @param show_cycles a boolean, if true all edges in all cyclic paths a red
/// @param show_longest_path a boolean, if true all edges in all longest paths a red
///
/// @return a rust graph-object associated to a code
///
/// @examples
/// code <- gcatbase::code(c("ACG", "CGG", "AC"))
/// g <- get_representing_graph_obj(code,2)
///
#[extendr]
pub fn get_representing_graph_obj(tuples: Vec<String>, show_cycles: bool, show_longest_path: bool) -> Robj {
    let code = new_code_from_vec(tuples);
    let g = match code.get_associated_graph() {
        Ok(graph) => graph,
        Err(e) => {
            rprintln!("Graph is corrupted: {}", e);
            R!(stop("Graph is corrupted")).unwrap();
            return list!()
        }
    };

    return representing_graph_obj_factory(g,show_cycles,show_longest_path);
}


/// Returns a i-component associated to a code
///
/// @param tuples a gcatbase::gcat.code object
/// @param i a integer the component index
/// @param show_cycles a boolean, if true all edges in all cyclic paths a red
/// @param show_longest_path a boolean, if true all edges in all longest paths a red
///
/// @return a i-component rust graph-object associated to a code
///
/// @examples
/// code <- gcatbase::code(c("ACGC", "CGGG", "AC"))
/// g <- get_representing_component_obj(code,2)
///
#[extendr]
pub fn get_representing_component_obj(tuples: Vec<String>, i: i32, show_cycles: bool, show_longest_path: bool) -> Robj {
    let code = new_code_from_vec(tuples);
    let g = match code.get_associated_graph() {
        Ok(graph) =>  graph,
        Err(e) => {
            rprintln!("Graph is corrupted: {}", e);
            R!(stop("Graph is corrupted")).unwrap();
            return list!()
        }
    };

    match g.component(i as u32) {
        Ok(graph) =>  return representing_graph_obj_factory(graph,show_cycles,show_longest_path),
        Err(e) => {
            rprintln!("Graph is corrupted: {}", e);
            R!(stop("Graph is corrupted")).unwrap();
            return list!()
        }
    }
}

/// Returns a list of all longest paths
///
/// This function returns all longest paths
/// in the graph associated to a set of words \emph{X}.
///
/// @param tuples A gcatbase::gcat.code object
///
/// @return A list of String vectors with all longest paths.
///
/// @seealso \link{get_representing_graph}
///
/// @examples
/// code <- gcatbase::code(c("ACG", "CGG", "AC"))
/// lp <- get_longest_paths(code)
///
/// @export
#[extendr]
pub fn get_longest_paths(tuples: Vec<String>) -> Vec<Robj> {
    let code = new_code_from_vec(tuples);
    let g = match code.get_associated_graph() {
        Ok(graph) =>  graph,
        Err(e) => {
            rprintln!("Graph is corrupted: {}", e);
            R!(stop("Graph is corrupted")).unwrap();
            return vec![]
        }
    };


    if let Some(l_paths) = g.all_longest_paths_as_vertex_vec() {
        return l_paths.iter().map(|x|  x.iter().collect_robj()).collect::<Vec<Robj>>()
    }

    return vec![]
}

/// Returns a list of all cyclic paths
///
/// This function returns all cyclic paths
/// in the graph associated to a set of words \emph{X}.
///
/// @param tuples A gcatbase::gcat.code object
///
/// @return A list of String vectors with all cyclic paths
///
/// @seealso \link{get_representing_graph}
///
/// @examples
/// code <- gcatbase::code(c("ACG", "CGA", "CA"))
/// lp <- get_cyclic_paths(code)
///
/// @export
#[extendr]
pub fn get_cyclic_paths(tuples: Vec<String>) -> Vec<Robj> {
    let code = new_code_from_vec(tuples);
    let g = match code.get_associated_graph() {
        Ok(graph) =>  graph,
        Err(e) => {
            rprintln!("Graph is corrupted: {}", e);
            R!(stop("Graph is corrupted")).unwrap();
            return vec![]
        }
    };

    if let Some(l_paths) = g.all_cycles_as_vertex_vec() {
        return l_paths.iter().map(|x|  x.iter().collect_robj()).collect::<Vec<Robj>>()
    }

    return vec![]
}

fn representing_graph_obj_factory(g: CircGraph, show_cycles: bool, show_longest_path: bool) -> Robj {
    let edges = g.get_edges();
    let cyclic_paths = match show_cycles {
        true => {
            if let Ok(s_g) = g.all_cycles_as_sub_graph() {
                s_g.get_edges()
            } else {
                vec![]
            }
        }
        false => vec![],
    };

    let longest_paths = match show_longest_path {
        true => {
            if let Ok(s_g) = g.all_longest_paths_as_sub_graph() {
                s_g.get_edges()
            } else {
                vec![]
            }
        }
        false => vec![],
    };

    let edges = edges.into_iter().filter(|x| !longest_paths.contains(x) && !cyclic_paths.contains(x)).flatten().collect::<Vec<String>>();


    return list!(vertices = g.get_vertices(),
    edges = edges,
    circular_path_edges = cyclic_paths.into_iter().flatten().collect::<Vec<String>>(),
    longest_path_edges = longest_paths.into_iter().flatten().collect::<Vec<String>>());

}


// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C rust_gcatcirc_lib.code in `entrypoint.c`.
extendr_module! {
    mod graph;
    fn get_representing_graph_obj;
    fn get_representing_component_obj;
    fn get_longest_paths;
    fn get_cyclic_paths;
}