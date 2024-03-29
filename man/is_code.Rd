% Generated by roxygen2: do not edit by hand
% Please edit documentation in R/extendr-wrappers.R
\name{is_code}
\alias{is_code}
\title{Checks whether the set of words is a code or not}
\usage{
is_code(tuples)
}
\arguments{
\item{tuples}{A gcatbase::gcat.code object}
}
\value{
A Boolean. If true the code is a code
}
\description{
This function returns true if a set of words is by
definition a code. A code \emph{X} is a set of words so that
any sequence has at most one decomposition in words of \emph{X}
}
\examples{
code <- gcatbase::code(c("ACG", "CGG", "AC"))
is_code(code)

}
