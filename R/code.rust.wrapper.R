#' Returns a new CircCode object.
#'
#' This function creates a [CircCode] object. The 'code.or.seq' parameter
#' can either be a sequence or set of words. Please note that if code.or.seq is a set of words the
#' 'tuple.length' parameter will be ignored. Likewise, the 'code.or.seq' parameter can also be a
#' comma- or space-separated string.
#'
#' @seealso [CircCode]
#'
#' @param code.or.seq a string vector or a sequence.
#' @param tuple.length if the used as tuple length if code.or.seq is a sequence.
#' @param id (optional) [string] the id/name  of the code
#'
#' @return  [CircCode] object.
#'
#' @examples
#' code <- new.circ.code(c("110", "10", "100"), id = "#1")
#' code <- new.circ.code("110100", 3, id = "#2")
#' code <- new.circ.code("110 100, 10")
#'
#' @export
new.circ.code <- function(code.or.seq, tuple.length = NULL, id = NULL) {
  code <- (function () {
    if (length(code.or.seq) == 1) {
      if (!is.null(tuple.length)) {
        return(CircCode$new_from_seq(code.or.seq, tuple.length))
      }

      code.or.seq <- unlist(stringr::str_split(code.or.seq, "[, ]+"))
    }

    return(CircCode$new_from_vec(code.or.seq))
  })();

  if (!is.null(id)) {
    code$set_id(id);
  }

  return(code);
}
