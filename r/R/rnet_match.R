#' Match two road networks
#'
#' @details
#'
#' `x` and `y` are cast as geoarrow arrays via `geoarrow::as_geoarrow_array()`.
#'
#' Spherical geometries are not supported at this moment. Using them will result
#' in inaccurate measurements.
#'
#' With a smaller number of features, building a spatial index on only the `x`
#' geometries will be faster than building a spatial index on both `x` and `y`.
#' However, at a larger number of features in `y`, building a spatial index
#' can result in significant performance enhancements.
#'
#' @param x the target of the join
#' @param y the features that will be joined to `x`
#' @param dist_tolerance the maximum distances that each line segment from `y` can be away from `x`
#' @param angle_tolerance the maximum difference in slope between line segments to be considered a match
#' @param trees whether to build a spatial index on `x` or `x` and `y`.
#' @export
rnet_match <- function(x, y, dist_tolerance, angle_tolerance, trees = c("xy", "x")) {

  trees <- match.arg(trees, several.ok = FALSE)

  # if x or y are sf objects extract geometry
  if (inherits(x, "sf")) {
    x <- sf::st_geometry(x)
  }

  if (inherits(y, "sf")) {
    y <- sf::st_geometry(y)
  }

  # TODO: handle other geometry types (geos & rsgeo)
  f <- switch(trees, x = rnet_match_one_tree, xy = rnet_match_two_trees)

  f(
    geoarrow::as_geoarrow_array(x),
    geoarrow::as_geoarrow_array(y),
    dist_tolerance,
    angle_tolerance,
    TRUE
  )
}

