use extendr_api::prelude::*;

use crate::lib_utils::new_code_from_vec;


/// Returns a rust graph-object
///
/// @param tuples A gcatbase::gcat.code object
///
/// @return A Boolean. If true the code is a code
///
/// @examples
/// code <- gcatbase::code(c("ACG", "CGG", "AC"))
/// g <- get_representing_graph_obj(code)
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
}