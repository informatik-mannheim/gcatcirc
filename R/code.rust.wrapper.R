igraph_factory <- function(vertices_edges_list, in.code) {
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

    return(add.code.coloring(g, in.code))
  }
}

add.code.coloring <- function(g, in.code) {

  v.coloring <- function(v, code) {
    for (word in code) {
      if (v == word) {
        return(T)
      }
      if (stringr::str_starts(v, word)) {
        new.v <- substring(v, nchar(word) + 1)
        return(v.coloring(new.v, code))
      }
    }

    return(F)
  }

  coloring <- function(g, code) {
    is.colored <- c()
    for (v in  names(igraph::V(g))) {
      is.colored <- c(is.colored, v.coloring(v, code))
    }

    igraph::V(g)$color <- ifelse(is.colored, "red", "white")

    return(g)
  }

  coloring(g, in.code)
}


#' Prepares a R igraph object of a graph associated to a code.
#'
#' This function factors a igraph (<http://igraph.org/r/>) object of an representing graph of a circular code.
#' The following definition describes a directed graph to an  code.
#' Recall from graph theory (Clark and Holton, 1991) that a graph G consists of
#' a finite set of vertices (nodes) V and a finite set of edges E. Here, an edge is a set \{v,w\} of vertices
#' from V . The graph is called oriented if the edges have an orientation, i.e. edges are considered to be
#' ordered pairs [v,w] in this case.\cr
#' Definition Let X be a code. We define a directed graph G(X) =
#' (V (X),E(X)) with set of vertices V (X) and set of edges E(X) as follows:
#' V (X) = \{N1...Ni,Ni+1...Nn : N1N2N3...Nn in X, 0 < i < n\}\cr
#' E(X) = \{[N1...Ni,Ni+1...Nn] : N1N2N3...Nn in X, 0 < i < n\}\cr
#' The graph G(X) is called the representing graph of X or the graph associated to X.\cr
#' Basically, the graph G(X) associated to a code X interprets n-tuples from X in (n−1) ways
#' by pairs of i-tuples and (n-i)-tuples for 0 < i < n.\cr
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
  return(igraph_factory(g.obj, code))
}


#' plots a R igraph object of a graph associated to a code.
#'
#' This function factors a igraph (<http://igraph.org/r/>) object of an representing graph of a circular code.
#' The following definition describes a directed graph to an  code.
#' Recall from graph theory (Clark and Holton, 1991) that a graph G consists of
#' a finite set of vertices (nodes) V and a finite set of edges E. Here, an edge is a set \{v,w\} of vertices
#' from V . The graph is called oriented if the edges have an orientation, i.e. edges are considered to be
#' ordered pairs [v,w] in this case.\cr
#' Definition Let X be a code. We define a directed graph G(X) =
#' (V (X),E(X)) with set of vertices V (X) and set of edges E(X) as follows:
#' V (X) = \{N1...Ni,Ni+1...Nn : N1N2N3...Nn in X, 0 < i < n\}\cr
#' E(X) = \{[N1...Ni,Ni+1...Nn] : N1N2N3...Nn in X, 0 < i < n\}\cr
#' The graph G(X) is called the representing graph of X or the graph associated to X.\cr
#' Basically, the graph G(X) associated to a code X interprets n-tuples from X in (n−1) ways
#' by pairs of i-tuples and (n-i)-tuples for 0 < i < n.\cr
#' \emph{2007 E. FIMMEL, C. J. MICHEL, AND L. STRÜNGMANN. N-nucleotide circular codes in graph theory}
#'
#' @param code is A gcatbase::gcat.code object.
#' @param show_cycles A bool value. If true the all edges which are part of a cycle are colored red.
#' @param show_longest_path A bool value. If true the all edges part of the longest path are colored blue.
#'
#' @return returns an integer, the id of the plot, this can be used to manipulate it from the command line. tk_canvas returns tkwin object, the Tk canvas..
#'
#' @examples
#' code <- gcatbase::code(c("ACG", "CGG", "AC"))
#' h <- plot_representing_graph(code, TRUE, TRUE)
#'
#' @export
plot_representing_graph <- function(code, show_cycles = F, show_longest_path = F) {
  g.obj <- get_representing_graph_obj(code, show_cycles = show_cycles, show_longest_path = show_longest_path)
  G <- igraph_factory(g.obj, code)
  igraph::tkplot(G)
}


#' Prepares a R igraph object of a i-component of a graph associated to a code.
#'
#' This function factors a igraph (<http://igraph.org/r/>) object of an representing graph of a circular code.
#' The following definition describes a directed graph to an code.
#' Recall from graph theory (Clark and Holton, 1991) that a graph G consists of
#' a finite set of vertices (nodes) V and a finite set of edges E. Here, an edge is a set \{v,w\} of vertices
#' from V . The graph is called oriented if the edges have an orientation, i.e. edges are considered to be
#' ordered pairs [v,w] in this case.\cr
#' Definition Let X be a code and i be a positive integer, the component index.
#' We define a directed graph G(X) =
#' (V (X),E(X)) with set of vertices V (X) and set of edges E(X) as follows:
#' V (X) = \{N1...Ni,Ni+1...Nn : N1N2N3...Nn in X\} for a given i \cr
#' E(X) = \{[N1...Ni,Ni+1...Nn] : N1N2N3...Nn in X\} for a given i\cr
#' The graph G(X) is called the i-component of the representing graph of X or the graph associated to X.\cr
#' \emph{2007 E. FIMMEL, C. J. MICHEL, AND L. STRÜNGMANN. N-nucleotide circular codes in graph theory}
#'
#' @param code is A gcatbase::gcat.code object.
#' @param i the component index.
#' @param show_cycles A bool value. If true the all edges which are part of a cycle are colored red.
#' @param show_longest_path A bool value. If true the all edges part of the longest path are colored blue.
#'
#' @return A igraph (<http://igraph.org/r/>) object: A graph representing a circular code.
#'
#' @examples
#' code <- gcatbase::code(c("ACG", "CGG", "AC"))
#' G <- get_component_of_representing_graph(code, i, TRUE, TRUE)
#' igraph::tkplot(G)
#'
#' @export
get_component_of_representing_graph <- function(code, i, show_cycles = F, show_longest_path = F) {
  g.obj <- get_representing_component_obj(code, i, show_cycles = show_cycles, show_longest_path = show_longest_path)
  return(igraph_factory(g.obj, code))
}


#' Plots a R igraph object of a i-component of a graph associated to a code.
#'
#' This function plots a igraph (<http://igraph.org/r/>) object of an representing graph of a circular code.
#' The following definition describes a directed graph to an code.
#' Recall from graph theory (Clark and Holton, 1991) that a graph G consists of
#' a finite set of vertices (nodes) V and a finite set of edges E. Here, an edge is a set \{v,w\} of vertices
#' from V . The graph is called oriented if the edges have an orientation, i.e. edges are considered to be
#' ordered pairs [v,w] in this case.\cr
#' Definition Let X be a code and i be a positive integer, the component index.
#' We define a directed graph G(X) =
#' (V (X),E(X)) with set of vertices V (X) and set of edges E(X) as follows:
#' V (X) = \{N1...Ni,Ni+1...Nn : N1N2N3...Nn in X\} for a given i \cr
#' E(X) = \{[N1...Ni,Ni+1...Nn] : N1N2N3...Nn in X\} for a given i\cr
#' The graph G(X) is called the i-component of the representing graph of X or the graph associated to X.\cr
#' \emph{2007 E. FIMMEL, C. J. MICHEL, AND L. STRÜNGMANN. N-nucleotide circular codes in graph theory}
#'
#' @param code is A gcatbase::gcat.code object.
#' @param i the component index.
#' @param show_cycles A bool value. If true the all edges which are part of a cycle are colored red.
#' @param show_longest_path A bool value. If true the all edges part of the longest path are colored blue.
#'
#' @return returns an integer, the id of the plot, this can be used to manipulate it from the command line. tk_canvas returns tkwin object, the Tk canvas.
#'
#' @examples
#' code <- gcatbase::code(c("ACGG", "CGGC", "AC"))
#' h <- plot_component_of_representing_graph(code, 2, TRUE, TRUE)

#'
#' @export
plot_component_of_representing_graph <- function(code, i, show_cycles = F, show_longest_path = F) {
  g.obj <- get_representing_component_obj(code, i, show_cycles = show_cycles, show_longest_path = show_longest_path)
  G <- igraph_factory(g.obj, code)
  igraph::tkplot(G)
}
