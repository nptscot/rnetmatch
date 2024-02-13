#' Weighted aggregation of matched networks
#'
#' @param x data.frame
#' @param y sf object
#' @param matches result of rnet_match()
#' @param ... unquoted variable names in y
#' @param y_len default `sf::st_length(y)` a numeric vector the same length as y containing the length of each linestring in y
#' @export
#' @rdname aggregate
rnet_aggregate_extensive <- function(
    x, y, matches, ...,
    # we automatically calculate the length of y
    # if sf::st_length() doesn't work it must be supplied
    y_len = sf::st_length(y)
) {
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
      wt = shared_len / as.numeric(y_len[j])
    ) |>
    dplyr::group_by(i) |>
    dplyr::summarise(dplyr::across(
      -all_of(c("j", "shared_len", "wt")),
      ~sum(.x * wt, na.rm = TRUE)
    ))
}


#' @export
#' @rdname aggregate
rnet_aggregate_intensive <- function(
    x, y, matches, ...,
    # we automatically calculate the length of y
    # if sf::st_length() doesn't work it must be supplied
    x_len = sf::st_length(x)
) {
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
      wt = shared_len / as.numeric(x_len[i])
    ) |>
    dplyr::group_by(i) |>
    dplyr::summarise(dplyr::across(
      -all_of(c("j", "shared_len", "wt")),
      ~ weighted.mean(.x, wt, na.rm = TRUE)
      # ~sum(.x * wt, na.rm = TRUE)
    ))
}

