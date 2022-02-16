igraph_factory <- function(vertices_edges_list) {
  g <- igraph::make_empty_graph();
  vertices_vec <- vertices_edges_list$vertices
  edges_vec <- vertices_edges_list$edges
  if (length(vertices_vec) > 0) {
    g <- g + igraph::vertex(vertices_vec, color = "white")

    g <- g + igraph::edges(edges_vec, color = "black")
    if (!is.null(vertices_edges_list$circular_path_edges)) {
      g <- g + igraph::edges(vertices_edges_list$circular_path_edges, color = "red")
    }
    if (!is.null(vertices_edges_list$longest_path_edges)) {
      g <- g + igraph::edges(vertices_edges_list$longest_path_edges, color = "green")
    }

    return(g)
  }
}


#' Prepares a R igraph object of a graph associated to a code.
#'
#' This function factors a igraph (<http://igraph.org/r/>) object of an representing graph of a circular code.
#' The following definition describes a directed graph to an n-nucleotide code.
#' Recall from graph theory (Clark and Holton, 1991) that a graph G consists of
#' a finite set of vertices (nodes) V and a finite set of edges E. Here, an edge is a set \{v,w\} of vertices
#' from V . The graph is called oriented if the edges have an orientation, i.e. edges are considered to be
#' ordered pairs [v,w] in this case.\cr
#' Definition Let X be a code. We define a directed graph G(X) =
#' (V (X),E(X)) with set of vertices V (X) and set of edges E(X) as follows:
#' N-NUCLEOTIDE CIRCULAR CODES IN GRAPH THEORY 5\cr
#' V (X) = \{N1...Ni,Ni+1...Nn : N1N2N3...Nn in X, 0 < i < n\}\cr
#' E(X) = \{[N1...Ni,Ni+1...Nn] : N1N2N3...Nn in X, 0 < i < n\}\cr
#' The graph G(X) is called the representing graph of X or the graph associated to X.\cr
#' Basically, the graph G(X) associated to a code X interprets n-nucleotide words from X in (n−1) ways
#' by pairs of i-nucleotides and (n-i)-nucleotides for 0 < i < n.\cr
#' \emph{2007 E. FIMMEL, C. J. MICHEL, AND L. STRÜNGMANN. N-nucleotide circular codes in graph theory}
#'
#' @param code is A gcatbase::gcat.code object.
#' @param show_cycles A bool value. If true the all edges which are part of a cycle are colored red.
#' @param show_longest_path A bool value. If true the all edges part of the longest path are colored blue.
#'
#' @return A igraph (<http://igraph.org/r/>) object: A graph representing a circular code.
#'
#' @examples
#' code <- gcatbase::code(c("ACG", "CGG", "AC"))
#' G <- get_representing_graph(code, TRUE, TRUE)
#' igraph::tkplot(G)
#'
#' @export
get_representing_graph <- function(code, show_cycles = F, show_longest_path = F) {
  g.obj <- get_representing_graph_obj(code, show_cycles = show_cycles, show_longest_path = show_longest_path)
  return(igraph_factory(g.obj))
}
