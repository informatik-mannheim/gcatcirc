use extendr_api::prelude::*;


use rust_gcatcirc_lib::code;
use rust_gcatcirc_lib::graph_circ::CircGraph as LibGraph;
use crate::lib_utils::new_code_from_vec;


pub struct CircGraph(LibGraph);

///'
///' @param tuples A gcatbase::gcat.code object
///'
///' @return A Boolean. If true the code is a code
///'
///' @examples
///' code <- gcatbase::code(c("ACG", "CGG", "AC"))
///' g <- get_representing_graph(code)
///'
///' @export
#[extendr]
pub fn get_representing_graph(tuples: Vec<String>) -> CircGraph {
    let code = new_code_from_vec(tuples);
    let graph = match code.get_associated_graph() {
        Ok(g) => g,
        Err(e) => {
            rprintln!("Graph is corrupted: {}", e);
            R!("stop(Graph is corrupted)");
            LibGraph::new(&code::CircCode::default()).unwrap()
        }
    };
    return CircGraph(graph);
}

///' @export
#[extendr]
impl CircGraph {

    pub fn get_edges(&self) -> Vec<String> {
        let  CircGraph(this) = self;
        return this.get_edges().into_iter().flatten().collect();
    }

    pub fn get_vertices(&self) -> Vec<String> {
        let  CircGraph(this) = self;
        return this.get_vertices().into_iter().flatten().collect();
    }

    pub fn get_longest_paths_sub_graph(&self) -> Self {
        let  CircGraph(this) = self;
        return Self(this.all_longest_paths_as_sub_graph());
    }

    pub fn all_cycles_as_sub_graph(&self) -> Self {
        let  CircGraph(this) = self;
        return Self(this.all_cycles_as_sub_graph());
    }

}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C rust_gcatcirc_lib.code in `entrypoint.c`.
extendr_module! {
    mod graph;
    fn get_representing_graph;
    impl CircGraph;
}