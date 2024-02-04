#' Weighted aggregation of matched networks
#'
#' @param x data.frame
#' @param y sf object
#' @param matches result of rnet_match()
#' @param ... unquoted variable names in y
#' @param y_len default `sf::st_length(y)` a numeric vector the same length as y containing the length of each linestring in y
#' @export
#' @examples
#' library(sf)
#' x <- read_sf("data-raw/geojson/princes_street_minimal_x_1.geojson") |>
#'   sf::st_transform(27700)
#' y <- read_sf("data-raw/geojson/princes_street_minimal.geojson") |>
#'   sf::st_transform(27700)
#' matches <- rnetmatch::rnet_match(x, y, dist_tolerance = 10, angle_tolerance = 5)
#' y_aggregated <- rnet_aggregate(x, y, matches, value)
#' y_joined <- cbind(x, y_aggregated)
#' plot(y["value"], lwd = 5)
#' plot(y_joined["value"], lwd = 5)
#' y_y_joined <- dplyr::bind_rows(y["value"], y_joined["value"])
#' plot(y_y_joined, lwd = 5)
rnet_aggregate <- function(
    x, y, matches, ...,
    # we automatically calculate the length of y
    # if sf::st_length() doesn't work it must be supplied
    y_len = as.numeric(sf::st_length(y))) {
  # TODO object validation of x, y, and matches

  # capture variables
  vars <- rlang::ensyms(...)
  # get var-names
  var_names <- vapply(vars, rlang::as_string, character(1))
  # TODO validate variables are in y before subsetting
  # extract j index
  j <- matches$j
  # subset vars by j to get ij pairs
  ij <- rlang::set_names(lapply(var_names, \(.x) y[[.x]][j]), var_names)
  # combine into 1 df
  dplyr::bind_cols(matches, ij) |>
    dplyr::mutate(
      wt = shared_len / y_len[j]
    ) |>
    dplyr::group_by(i) |>
    dplyr::summarise(dplyr::across(
      -all_of(c("j", "shared_len", "wt")),
      ~ sum(.x * wt, na.rm = TRUE)
    ))
}


# library(sf)
# library(dplyr)
#
# x <- read_sf("data-raw/geojson/intersection_example_simple.geojson") |>
#   sf::st_transform(27700)
# y <- read_sf("data-raw/geojson/intersection_example_complex.geojson") |>
#   sf::st_transform(27700)
#
# matches <- rnetmatch::rnet_match(x, y, 10, 5) |>
#   as_tibble()
#
# rnet_aggregate(
#   x, y, matches,
#   # columns in j
#   all_fastest_bicycle,
#   Quietness,
#   commute_quietest_bicycle_go_dutch
# )
#
#
