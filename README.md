# Genetic Code Analysis Toolkit for R - GCAT Circular package (gcatcirc)

Warning: version 0.2.x is only a beta and not ready for public use.

This project contains the source code for the R version of the Genetic Code Analysis Toolkit (GCAT) project for circular codes. Please refer also to the [cammbio homepage](https://www.cammbio.hs-mannheim.de/institute.html) for more information.

![Genetic Code Analysis Toolkit Logo](./man/resources/bio/gcat/logo.png?raw=true)

## Installation
`gcatcirc` is available for R version 4.1 and higher. It requires a [Rust](https://www.rust-lang.org/) 1.57 (or later) compiler installed on your machine. Furthermore the current version of devtools has to be installed on your computer. If you are using Microsoft Windows, 
then you need to install [Rtools](https://cran.r-project.org/bin/windows/Rtools/).

A common error is that rust does not have the target installed.

```CMD
rustup target add [YOUR_TARGET]
```

Starting a new R console and run:
```R
install.packages("devtools")
devtools::install_github("informatik-mannheim/gcatcirc")

```

### Windows- Step by Step 

-Download & Install:
 1) R: https://cran.r-project.org/bin/windows/base/R-4.3.2-win.exe
 2) RStudio: https://download1.rstudio.org/electron/windows/RStudio-2023.09.1-494.exe
 3) Rust: https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe
 4) RTools: https://cran.r-project.org/bin/windows/Rtools/rtools43/files/rtools43-5863-5818.exe

-Reboot

-Open Command Prompt
 1) rustup  target add x86_64-pc-windows-gnu

-Open RStudio

 1) install.packages("devtools")
 2) devtools::install_github("informatik-mannheim/gcatbase")
 3) devtools::install_github("informatik-mannheim/gcatcirc")

-Done

## Usage

Once you installed `gcatcirc`, you may read its help pages. The file [./example/Tutorial.Rmd](./example/Tutorial.Rmd) is a good start for an introduction and a tutorial. This markdown document can be executed. The executed tutorial is available [online](https://oc.informatik.hs-mannheim.de/s/FrXnTHNw3gPZJTK/download).


## Function outline

<!--outline-start-->
### [General code tools](#general-code-tools)

[is_code](#is_code)<br>
[all_ambiguous_sequences](#all_ambiguous_sequences)<br>
[circular_shift](#circular_shift)<br>

### [Circular code tools](#general-circular-codes)

[is_code_circular](#is_code_circular)<br>
[is_code_cn_circular](#is_code_cn_circular)<br>
[is_code_comma_free](#is_code_comma_free)<br>
[is_code_strong_comma_free](#is_code_strong_comma_free)<br>
[get_exact_k_circular](#get_exact_k_circular)<br>
[get_k_graph_circular](#get_k_graph_circular)<br>

### [C<sup>3</sup> codes](#c3-codes)

[c3_codes](#c3_codes)<br>
[c3_code](#c3_code)<br>
[c3_equiv_class](#c3_equiv_class)<br>
[c3_equivmatrix](#c3_equivmatrix)<br>
[c3_in_class](#c3_in_class)<br>

### [General graph tools](#general-graph-codes)

[get_representing_graph](#get_representing_graph)<br>
[get_component_of_representing_graph](#get_component_of_representing_graph)<br>
[plot_representing_graph](#plot_representing_graph)<br>
[plot_component_of_representing_graph](#plot_component_of_representing_graph)<br>
[get_cyclic_paths](#get_cyclic_paths)<br>
[get_longest_paths](#get_longest_paths)<br>

<!--outline-end-->

<!--doc-start-->
## General code tools

### is_code

#### Usage
```R 
is_code(tuples)
```

#### Arguments
 
*tuples*	A gcatbase::gcat.code object<br>


#### Return
 
A Boolean. If true the code is a code


#### Description
 
This function returns true if a set of words is by
definition a code. A code *X* is a set of words so that
any sequence has at most one decomposition in words of *X*


#### Examples
```R 
code <- gcatbase::code(c("ACG", "CGG", "AC"))
is_code(code)

```
<hr>

### all_ambiguous_sequences

#### Usage
```R 
all_ambiguous_sequences(tuples)
```

#### Arguments
 
*tuples*	A gcatbase::gcat.code object<br>


#### Return
 
A String vector with all ambiguous sequences.


#### Description
 
This function returns all ambiguous sequences
which only exist if a set of words *X* is by
definition not a code. Such a sequence can be decomposed in
at least two disjoint sets of words of *X*.


#### Examples
```R 
code <- gcatbase::code(c("ACG", "CGG", "AC"))
all_ambiguous_sequences(code)

```
<hr>

### circular_shift

#### Usage
```R 
circular_shift(tuples, sh)
```

#### Arguments
 
*tuples*	A gcatbase::gcat.code object<br>

*sh*	A integer, the shift index, i.e. the number of shifts.<br>


#### Return
 
Boolean value. True if the code is circular.


#### Description
 
Under the concept shift is understood a circular permutation, i.e.
let *X*={123, 332}, then c.shift(2) results in {312, 233}


#### Examples
```R 
code <- gcatbase::code(c("ACG", "CGG", "AC"))
circular_shift(code, 2)

```
<hr>

## Circular code tools

### is_code_circular

#### Usage
```R 
is_code_circular(tuples)
```

#### Arguments
 
*tuples*	A gcatbase::gcat.code object<br>


#### Return
 
Boolean value. True if the code is circular.


#### Description
 
This function checks if a code is circular. Circular codes are sets of
tuples *X* of different tuple length where
every concatenation of words *w* in *X* written on a circle
has only a single decomposition into words from *X*.<br>
For more info on this subject read:<br>
*https://www.ncbi.nlm.nih.gov/pmc/articles/PMC5492142/*,<br>
*http://dpt-info.u-strasbg.fr/~c.michel/Circular_Codes.pdf*,<br>
*2007 Christian MICHEL. CIRCULAR CODES IN GENES*


#### Examples
```R 
code <- gcatbase::code(c("ACG", "CGG", "AC"))
is_code_circular(code)

```
<hr>

### is_code_cn_circular

#### Usage
```R 
is_code_cn_circular(tuples)
```

#### Arguments
 
*tuples*	A gcatbase::gcat.code object<br>


#### Return
 
Boolean value. True if the code is Cn circular.


#### Description
 
A code is cn circular if all circular permutations of the code (of all tuples) are circular codes again.
In total, this function checks 'x' circular permutations where 'x' is the least
common multiple of all tuple lengths used. This is an extended property of circular codes.


#### Examples
```R 
code <- gcatbase::code(c("ACG", "CGG", "AC"))
k <- is_code_cn_circular(code)

```
<hr>

### is_code_comma_free

#### Usage
```R 
is_code_comma_free(tuples)
```

#### Arguments
 
*tuples*	A gcatbase::gcat.code object<br>


#### Return
 
Boolean value. True if the code is comma free.


#### Description
 
This function checks if a code is comma free.
Comma free codes are a more restrictive codes from the circular code family.
A comma free code *X* is a code in which no concatenation of a
nonempty suffix of any word from *X* and a nonempty prefix of any word from *X* forms a word from *X*.<br>
This is an extended property of the circular codes. See *is_code_circular* for more details.<br>
For more info on this subject read:<br>
*https://www.ncbi.nlm.nih.gov/pmc/articles/PMC5492142/*,<br>
*http://dpt-info.u-strasbg.fr/~c.michel/Circular_Codes.pdf*,<br>
*2007 Christian MICHEL. CIRCULAR CODES IN GENES*


#### Examples
```R 
code <- gcatbase::code(c("ACG", "CGG", "AC"))
is_code_comma_free(code)

```
<hr>

### is_code_strong_comma_free

#### Usage
```R 
is_code_strong_comma_free(tuples)
```

#### Arguments
 
*tuples*	A gcatbase::gcat.code object<br>


#### Return
 
Boolean value. True if the code is strong comma free.


#### Description
 
This function checks if a code is strong comma free.
Strong comma free codes are a more restrictive codes from the circular code family.
A strong comma free code *X* is a code in which no nonempty suffix of any word from *X*
is a nonempty prefix of any word from *X*.<br>
This is an extended property of the circular codes. See *is_code_comma_free* for more details.<br>
For more info on this subject read:<br>
*https://www.ncbi.nlm.nih.gov/pmc/articles/PMC5492142/*,<br>
*http://dpt-info.u-strasbg.fr/~c.michel/Circular_Codes.pdf*,<br>
*2007 Christian MICHEL. CIRCULAR CODES IN GENES*


#### Examples
```R 
code <- gcatbase::code(c("ACG", "CGG", "AC"))
is_code_strong_comma_free(code)

```
<hr>

### get_exact_k_circular

#### Usage
```R 
get_exact_k_circular(tuples)
```

#### Arguments
 
*tuples*	A gcatbase::gcat.code object<br>


#### Return
 
Integer value, the exact k value of the k-circularity.


#### Description
 
K circle codes are a less restrictive code from the family of circle codes. These codes only ensure that for every
concatenation of less than k tuples from *X* written on a circle, there is only one partition in tuples from *X*.<br>
For mor details see: *https://link.springer.com/article/10.1007/s11538-020-00770-7*


#### Examples
```R 
code <- gcatbase::code(c("ACG", "CGG", "AC"))
k <- get_exact_k_circular(code)

```
<hr>

### get_k_graph_circular

#### Usage
```R 
get_k_graph_circular(tuples)
```

#### Arguments
 
*tuples*	A gcatbase::gcat.code object<br>


#### Return
 
Integer value, the k-graph value of the k-graph-circularity. If code is not k-graph circle it returns -1.


#### Description
 
K-graph circle codes are a more restrictive than k-circle codes.
These codes only ensure that all cycles in the representing graph have a common length.
If the code is circular or the code contains cycles of
different length the teh function returns -1. <br>
For mor details see: *https://link.springer.com/article/10.1007/s11538-020-00770-7*


#### Examples
```R 
code <- gcatbase::code(c("ACG", "CGG", "AC"))
k <- get_k_graph_circular(code)

```
<hr>

## C<sup>3</sup> codes

### c3_codes

#### Usage
```R 
c3_codes(is = seq_along(all_c3_codes))
```

#### Arguments
 
*is*	a vector of Integer. for all i in is: 1 <= i <= 216.<br>


#### Return
 
A list of C3 codes


#### Description
 
Returns a list of maximal self-complementary C3 codes. Without paramet it returns all C3 codes.


#### Examples
```R \-```
<hr>

### c3_code

#### Usage
```R 
c3_code(i)
```

#### Arguments
 
*i*	Integer 1 <= i <= 216. The number of the C3 code<br>


#### Return
 
A C3 code


#### Description
 
Returns the i-th maximal self-complmentary C3 code. There are exactly 216 such codes.


#### Examples
```R \-```
<hr>

### c3_equiv_class

#### Usage
```R 
c3_equiv_class(cid)
```

#### Arguments
 
*cid*	Integer 0 < i < 217. The number of the C3 code.<br>


#### Return
 
Its equivalence class.


#### Description
 
Equivalence class for a C3 code number.


#### Examples
```R \-```
<hr>

### c3_equivmatrix

#### Usage
```R 
c3_equivmatrix
```

#### Arguments
 \-

#### Return
 
First column: C3 code number, second row: equivalence class.


#### Description
 
Table for mapping of code numbers to equivalence classes.


#### Examples
```R \-```
<hr>

### c3_in_class

#### Usage
```R 
c3_in_class(eid)
```

#### Arguments
 
*eid*	Equivalence class.<br>


#### Return
 
List of C3 code numbers.


#### Description
 
All code numbers for given equivalence class


#### Examples
```R \-```
<hr>

## General graph tools

### get_representing_graph

#### Usage
```R 
get_representing_graph(code, show_cycles = F, show_longest_path = F)
```

#### Arguments
 
*code*	is A gcatbase::gcat.code object.<br>

*show_cycles*	A bool value. If true the all edges which are part of a cycle are colored red.<br>

*show_longest_path*	A bool value. If true the all edges part of the longest path are colored blue.<br>


#### Return
 
A igraph (\url{http://igraph.org/r/}) object: A graph representing a circular code.


#### Description
 
This function factors a igraph (\url{http://igraph.org/r/}) object of an representing graph of a circular code.
The following definition describes a directed graph to an  code.
Recall from graph theory (Clark and Holton, 1991) that a graph G consists of
a finite set of vertices (nodes) V and a finite set of edges E. Here, an edge is a set \{v,w\} of vertices
from V . The graph is called oriented if the edges have an orientation, i.e. edges are considered to be
ordered pairs *v,w* in this case.<br>
Definition Let X be a code. We define a directed graph G(X) =
(V (X),E(X)) with set of vertices V (X) and set of edges E(X) as follows:
V (X) = \{N1...Ni,Ni+1...Nn : N1N2N3...Nn in X, 0 < i < n\}<br>
E(X) = \{*N1...Ni,Ni+1...Nn* : N1N2N3...Nn in X, 0 < i < n\}<br>
The graph G(X) is called the representing graph of X or the graph associated to X.<br>
Basically, the graph G(X) associated to a code X interprets n-tuples from X in (n−1) ways
by pairs of i-tuples and (n-i)-tuples for 0 < i < n.<br>
*2007 E. FIMMEL, C. J. MICHEL, AND L. STRÜNGMANN. N-nucleotide circular codes in graph theory*


#### Examples
```R 
code <- gcatbase::code(c("ACG", "CGG", "AC"))
G <- get_representing_graph(code, TRUE, TRUE)
igraph::tkplot(G)

```
<hr>

### get_component_of_representing_graph

#### Usage
```R 
get_component_of_representing_graph(
  code,
  i,
  show_cycles = F,
  show_longest_path = F
)
```

#### Arguments
 
*code*	is A gcatbase::gcat.code object.<br>

*i*	the component index.<br>

*show_cycles*	A bool value. If true the all edges which are part of a cycle are colored red.<br>

*show_longest_path*	A bool value. If true the all edges part of the longest path are colored blue.<br>


#### Return
 
A igraph (\url{http://igraph.org/r/}) object: A graph representing a circular code.


#### Description
 
This function factors a igraph (\url{http://igraph.org/r/}) object of an representing graph of a circular code.
The following definition describes a directed graph to an code.
Recall from graph theory (Clark and Holton, 1991) that a graph G consists of
a finite set of vertices (nodes) V and a finite set of edges E. Here, an edge is a set \{v,w\} of vertices
from V . The graph is called oriented if the edges have an orientation, i.e. edges are considered to be
ordered pairs *v,w* in this case.<br>
Definition Let X be a code and i be a positive integer, the component index.
We define a directed graph G(X) =
(V (X),E(X)) with set of vertices V (X) and set of edges E(X) as follows:
V (X) = \{N1...Ni,Ni+1...Nn : N1N2N3...Nn in X\} for a given i <br>
E(X) = \{*N1...Ni,Ni+1...Nn* : N1N2N3...Nn in X\} for a given i<br>
The graph G(X) is called the i-component of the representing graph of X or the graph associated to X.<br>
*2007 E. FIMMEL, C. J. MICHEL, AND L. STRÜNGMANN. N-nucleotide circular codes in graph theory*


#### Examples
```R 
code <- gcatbase::code(c("ACG", "CGG", "AC"))
G <- get_component_of_representing_graph(code, i, TRUE, TRUE)
igraph::tkplot(G)

```
<hr>

### plot_representing_graph

#### Usage
```R 
plot_representing_graph(code, show_cycles = F, show_longest_path = F)
```

#### Arguments
 
*code*	is A gcatbase::gcat.code object.<br>

*show_cycles*	A bool value. If true the all edges which are part of a cycle are colored red.<br>

*show_longest_path*	A bool value. If true the all edges part of the longest path are colored blue.<br>


#### Return
 
returns an integer, the id of the plot, this can be used to manipulate it from the command line. tk_canvas returns tkwin object, the Tk canvas..


#### Description
 
This function factors a igraph (\url{http://igraph.org/r/}) object of an representing graph of a circular code.
The following definition describes a directed graph to an  code.
Recall from graph theory (Clark and Holton, 1991) that a graph G consists of
a finite set of vertices (nodes) V and a finite set of edges E. Here, an edge is a set \{v,w\} of vertices
from V . The graph is called oriented if the edges have an orientation, i.e. edges are considered to be
ordered pairs *v,w* in this case.<br>
Definition Let X be a code. We define a directed graph G(X) =
(V (X),E(X)) with set of vertices V (X) and set of edges E(X) as follows:
V (X) = \{N1...Ni,Ni+1...Nn : N1N2N3...Nn in X, 0 < i < n\}<br>
E(X) = \{*N1...Ni,Ni+1...Nn* : N1N2N3...Nn in X, 0 < i < n\}<br>
The graph G(X) is called the representing graph of X or the graph associated to X.<br>
Basically, the graph G(X) associated to a code X interprets n-tuples from X in (n−1) ways
by pairs of i-tuples and (n-i)-tuples for 0 < i < n.<br>
*2007 E. FIMMEL, C. J. MICHEL, AND L. STRÜNGMANN. N-nucleotide circular codes in graph theory*


#### Examples
```R 
code <- gcatbase::code(c("ACG", "CGG", "AC"))
h <- plot_representing_graph(code, TRUE, TRUE)

```
<hr>

### plot_component_of_representing_graph

#### Usage
```R 
plot_component_of_representing_graph(
  code,
  i,
  show_cycles = F,
  show_longest_path = F
)
```

#### Arguments
 
*code*	is A gcatbase::gcat.code object.<br>

*i*	the component index.<br>

*show_cycles*	A bool value. If true the all edges which are part of a cycle are colored red.<br>

*show_longest_path*	A bool value. If true the all edges part of the longest path are colored blue.<br>


#### Return
 
returns an integer, the id of the plot, this can be used to manipulate it from the command line. tk_canvas returns tkwin object, the Tk canvas.


#### Description
 
This function plots a igraph (\url{http://igraph.org/r/}) object of an representing graph of a circular code.
The following definition describes a directed graph to an code.
Recall from graph theory (Clark and Holton, 1991) that a graph G consists of
a finite set of vertices (nodes) V and a finite set of edges E. Here, an edge is a set \{v,w\} of vertices
from V . The graph is called oriented if the edges have an orientation, i.e. edges are considered to be
ordered pairs *v,w* in this case.<br>
Definition Let X be a code and i be a positive integer, the component index.
We define a directed graph G(X) =
(V (X),E(X)) with set of vertices V (X) and set of edges E(X) as follows:
V (X) = \{N1...Ni,Ni+1...Nn : N1N2N3...Nn in X\} for a given i <br>
E(X) = \{*N1...Ni,Ni+1...Nn* : N1N2N3...Nn in X\} for a given i<br>
The graph G(X) is called the i-component of the representing graph of X or the graph associated to X.<br>
*2007 E. FIMMEL, C. J. MICHEL, AND L. STRÜNGMANN. N-nucleotide circular codes in graph theory*


#### Examples
```R 
code <- gcatbase::code(c("ACGG", "CGGC", "AC"))
h <- plot_component_of_representing_graph(code, 2, TRUE, TRUE)

```
<hr>

### get_cyclic_paths

#### Usage
```R 
get_cyclic_paths(tuples)
```

#### Arguments
 
*tuples*	A gcatbase::gcat.code object<br>


#### Return
 
A list of String vectors with all cyclic paths


#### Description
 
This function returns all cyclic paths
in the graph associated to a set of words *X*.


#### Examples
```R 
code <- gcatbase::code(c("ACG", "CGA", "CA"))
lp <- get_cyclic_paths(code)

```
<hr>

### get_longest_paths

#### Usage
```R 
get_longest_paths(tuples)
```

#### Arguments
 
*tuples*	A gcatbase::gcat.code object<br>


#### Return
 
A list of String vectors with all longest paths.


#### Description
 
This function returns all longest paths
in the graph associated to a set of words *X*.


#### Examples
```R 
code <- gcatbase::code(c("ACG", "CGG", "AC"))
lp <- get_longest_paths(code)

```
<hr>

<!--doc-end-->


# Copyright and license

Code and documentation copyright 2018-2022 Mannheim University of Applied Sciences. Code released under the Apache License, Version 2.0.
