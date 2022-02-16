use extendr_api::prelude::*;

extern crate rust_gcatcirc_lib;

mod lib_utils;
use lib_utils::new_code_from_vec;

mod graph;
use graph::*;
/// Checks whether the set of words is a code or not
///
/// This function returns true if a set of words is by
/// definition a code. A code \emph{X} is a set of words so that
/// any sequence has at most one decomposition in words of \emph{X}
///
/// @param tuples A gcatbase::gcat.code object
///
/// @return A Boolean. If true the code is a code
///
/// @examples
/// code <- gcatbase::code(c("ACG", "CGG", "AC"))
/// is_code(code)
///
/// @export
#[extendr]
pub fn is_code(tuples: Vec<String>) -> bool {
    let code = new_code_from_vec(tuples);
    return code.is_code();
}

/// If a set of words is not a code it returns all ambiguous sequences.
///
/// This function returns all ambiguous sequences
/// which only exist if a set of words \emph{X} is by
/// definition not a code. Such a sequence can be decomposed in
/// at least two disjoint sets of words of \emph{X}.
///
/// @param tuples A gcatbase::gcat.code object
///
/// @return A String vector with all ambiguous sequences.
///
/// @seealso \link{is_code}
///
/// @examples
/// code <- gcatbase::code(c("ACG", "CGG", "AC"))
/// all_ambiguous_sequences(code)
///
/// @export
#[extendr]
fn all_ambiguous_sequences(tuples: Vec<String>) -> Vec<String> {
    let code = new_code_from_vec(tuples);
    return code.all_ambiguous_sequences().1;
}

/// Check if a code is circular.
///
/// This function checks if a code is circular. Circular codes are sets of
/// tuples \emph{X} of different tuple length where
/// every concatenation of words \emph{w} in \emph{X} written on a circle
/// has only a single decomposition into words from \emph{X}.\cr
/// For more info on this subject read:\cr
/// \link{https://www.ncbi.nlm.nih.gov/pmc/articles/PMC5492142/},\cr
/// \link{http://dpt-info.u-strasbg.fr/~c.michel/Circular_Codes.pdf},\cr
/// \emph{2007 Christian MICHEL. CIRCULAR CODES IN GENES}
///
/// @param tuples A gcatbase::gcat.code object
///
/// @return Boolean value. True if the code is circular.
///
/// @examples
/// code <- gcatbase::code(c("ACG", "CGG", "AC"))
/// is_code_circular(code)
///
/// @export
#[extendr]
fn is_code_circular(tuples: Vec<String>) -> bool {
    let code = new_code_from_vec(tuples);
    return code.is_circular();
}

/// This function checks if a code is circular.
///
/// K circle codes are a less restrictive code from the family of circle codes. These codes only ensure that for every
/// concatenation of less than k tuples from \emph{X} written on a circle, there is only one partition in tuples from \emph{X}.\cr
/// For mor details see: \link{https://link.springer.com/article/10.1007/s11538-020-00770-7}
///
/// @param tuples A gcatbase::gcat.code object
///
/// @return Integer value, the exact k value of the k-circularity.
///
/// @examples
/// code <- gcatbase::code(c("ACG", "CGG", "AC"))
/// k <- get_exact_k_circular(code)
///
/// @seealso \link{is_code_circular}
///
/// @export
// [[Rcpp::export]]
#[extendr]
fn get_exact_k_circular(tuples: Vec<String>) -> u32 {
    let code = new_code_from_vec(tuples);
    return code.get_exact_k_circular();
}

/// This function checks if a code is K-Graph circular.
///
/// K-Graph circle codes are a less restrictive code from the family of circle codes. These codes only ensure that for every
/// concatenation of less than k tuples from \emph{X} written on a circle, there is only one partition in tuples from \emph{X}.\cr
/// For mor details see: \link{https://link.springer.com/article/10.1007/s11538-020-00770-7}
///
/// @param tuples A gcatbase::gcat.code object
///
/// @return Integer value, the exact k value of the k-circularity.
///
/// @examples
/// code <- gcatbase::code(c("ACG", "CGG", "AC"))
/// k <- get_exact_k_graph_circular(code)
///
/// @seealso \link{is_code_circular}
///
/// @export
// [[Rcpp::export]]
#[extendr]
fn get_exact_k_graph_circular(tuples: Vec<String>) -> u32 {
    let code = new_code_from_vec(tuples);
    return code.get_exact_k_circular();
}

/// This function checks if a code is Cn-circular.
///
/// That all circular permutations of the code (of all tuples) are circular codes again.
/// In total, this function checks all 'n' circular permutations where 'n' is the greatest
/// common multiple of all tuple lengths used.
/// This is an extended property of circular codes.
///
/// @param tuples A gcatbase::gcat.code object
///
/// @return Boolean value. True if the code is Cn circular.
///
/// @examples
/// code <- gcatbase::code(c("ACG", "CGG", "AC"))
/// k <- is_code_cn_circular(code)
///
/// @seealso \link{is_code_circular}
///
/// @export
#[extendr]
fn is_code_cn_circular(tuples: Vec<String>) -> bool {
    let code = new_code_from_vec(tuples);
    return code.is_cn_circular();
}

/// Check if a code is comma free.
///
/// This function checks if a code is comma free.
/// Comma free codes are a more restrictive codes from the circular code family.
/// A comma free code \emph{X} is a code in which no concatenation of a
/// nonempty suffix of any word from \emph{X} and a nonempty prefix of any word from \emph{X} forms a word from \emph{X}.\cr
/// This is an extended property of the circular codes. See \link{is_code_circular} for more details.\cr
/// For more info on this subject read:\cr
/// \link{https://www.ncbi.nlm.nih.gov/pmc/articles/PMC5492142/},\cr
/// \link{http://dpt-info.u-strasbg.fr/~c.michel/Circular_Codes.pdf},\cr
/// \emph{2007 Christian MICHEL. CIRCULAR CODES IN GENES}
///
/// @param tuples A gcatbase::gcat.code object
///
/// @return Boolean value. True if the code is comma free.
///
/// @examples
/// code <- gcatbase::code(c("ACG", "CGG", "AC"))
/// is_code_comma_free(code)
///
/// @seealso \link{is_code_circular}
///
/// @export
#[extendr]
fn is_code_comma_free(tuples: Vec<String>) -> bool {
    let code = new_code_from_vec(tuples);
    return code.is_comma_free();
}

/// Check if a code is strong comma free.
///
/// This function checks if a code is strong comma free.
/// Strong comma free codes are a more restrictive codes from the circular code family.
/// A strong comma free code \emph{X} is a code in which no nonempty suffix of any word from \emph{X}
/// is a nonempty prefix of any word from \emph{X}.\cr
/// This is an extended property of the circular codes. See \link{is_code_comma_free} for more details.\cr
/// For more info on this subject read:\cr
/// \link{https://www.ncbi.nlm.nih.gov/pmc/articles/PMC5492142/},\cr
/// \link{http://dpt-info.u-strasbg.fr/~c.michel/Circular_Codes.pdf},\cr
/// \emph{2007 Christian MICHEL. CIRCULAR CODES IN GENES}
///
/// @param tuples A gcatbase::gcat.code object
///
/// @return Boolean value. True if the code is strong comma free.
///
/// @examples
/// code <- gcatbase::code(c("ACG", "CGG", "AC"))
/// is_code_strong_comma_free(code)
///
/// @seealso \link{is_code_circular}, \link{is_code_comma_free}
///
/// @export
#[extendr]
fn is_code_strong_comma_free(tuples: Vec<String>) -> bool {
    let code = new_code_from_vec(tuples);
    return code.is_strong_comma_free();
}


/// Shifts each tuple by `sh` positions
///
/// Under the concept shift is understood a circular permutation, i.e.
/// let \emph{X}={123, 332}, then c.shift(2) results in {312, 233}
///
/// @param tuples A gcatbase::gcat.code object
/// @param sh A integer, the shift index, i.e. the number of shifts.
///
/// @return Boolean value. True if the code is circular.
/// @examples
/// code <- gcatbase::code(c("ACG", "CGG", "AC"))
/// is_code_circular(code)
///
/// @export
#[extendr]
fn circular_shift(tuples: Vec<String>, sh: i32) -> Vec<String> {
    let mut code = new_code_from_vec(tuples);
    code.shift(sh);
    return code.get_code()
}



// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C rust_gcatcirc_lib.code in `entrypoint.c`.
extendr_module! {
    mod gcatcirc; // like R package name
    fn all_ambiguous_sequences;
    fn is_code;
    fn circular_shift;
    fn is_code_circular;
    fn is_code_comma_free;
    fn is_code_strong_comma_free;
    fn is_code_cn_circular;
    fn get_exact_k_circular;
    use graph;

}
