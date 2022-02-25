#' The i-th C3 code.
#'
#' @param i Integer 1 <= i <= 216. The number of the C3 code
#' @return A C3 code
#' @export
c3_code = function(i) gcatbase::code(all_c3_codes[[i]], id = paste0("maximum self-complementary C³ ", i))

#' Table for mapping of code numbers to equivalence classes.
#'
#' @return First column: C3 code number, second row: equivalence class.
c3_equivmatrix = {
  equiv = c(1, 1, 2, 3, 2, 3, 4, 5, 4, 5, 6, 7, 8, 9, 2, 3, 10, 11, 11, 10, 6, 1, 8, 12, 13, 9, 7, 9, 1, 7, 6, 12, 8, 10, 13, 9, 8, 7, 6, 10, 11, 3, 2, 11, 5, 4, 13, 12, 12, 13, 4, 5, 5, 4, 5, 13, 12, 4, 13, 12, 3, 2, 11, 7, 8, 9, 11, 6, 10, 9, 1, 7, 12, 6, 10, 13, 8, 6, 1, 10, 8, 9, 12, 7, 13, 3, 8, 7, 10, 2, 6, 9, 11, 11, 4, 5, 4, 5, 1, 1, 2, 3, 2, 3, 14, 14, 15, 16, 17, 16, 18, 15, 19, 17, 19, 18, 20, 20, 18, 15, 17, 16, 14, 14, 16, 18, 15, 17, 19, 20, 20, 19, 20, 19, 20, 19, 17, 18, 15, 16, 14, 16, 14, 17, 18, 15, 14, 15, 16, 14, 18, 16, 17, 17, 19, 15, 20, 19, 18, 20, 21, 21, 22, 23, 22, 23, 24, 21, 25, 25, 24, 26, 27, 24, 26, 27, 25, 24, 21, 25, 26, 27, 27, 26, 22, 23, 23, 22, 23, 22, 23, 27, 27, 22, 26, 26, 21, 25, 25, 24, 24, 26, 27, 24, 26, 27, 21, 24, 25, 25, 21, 22, 23, 21, 22, 23)
  cbind(1:216, equiv)
}

#' Equivalence class for a C3 code number.
#'
#' @param cid Integer 0 < i < 217. The number of the C3 code.
#'
#' @return Its equivalence class.
#' @export
c3_equiv_class = function(cid) c3_equivmatrix[cid, 2]


#' All code numbers for given equivalence class
#'
#' @param eid Equivalence class.
#'
#' @return List of C3 code numbers.
#' @export
c3_in_class = function(eid) {
  c3_equivmatrix[c3_equivmatrix[, 2] == eid, 1]
}