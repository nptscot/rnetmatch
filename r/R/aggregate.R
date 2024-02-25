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
#' xfp <- system.file("extdata/princes_street_minimal_x_1.geojson", package = "rnetmatch")
#' yfp <- system.file("extdata/princes_street_minimal.geojson", package = "rnetmatch")
#'
#' x <- st_transform(st_read(xfp, quiet = TRUE), 27700)[3, ]
#' y <- st_transform(st_read(yfp, quiet = TRUE), 27700)[2:3, ]
#'
#' matches <- rnet_match(x, y, dist_tolerance = 10, angle_tolerance = 5)
 #' rnet_aggregate_intensive(x, y, matches, value)
#' rnet_aggregate_extensive(x, y, matches, value)
#' @rdname aggregate
rnet_aggregate_extensive <- function(
    x, y, matches, ...,
    # we automatically calculate the length of y
    # if sf::st_length() doesn't work it must be supplied
    y_len = as.numeric(sf::st_length(y))
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
      ~ sum(.x * wt, na.rm = TRUE)
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


#' Aggregate Network Matches
#' @param source data.frame that contains attributes that will be attributed to the target dataset
#' @param matches results from `rnet_match()`. Contain `i`, `j`, and `shared_len`.
#' @param extensive_vars character vector of variables names in `source` that will be aggregated to the target as an extensive variable. See Details.
#' @param intensive_vars character vector of variable names in `source` that will be aggregated to hte target as intensive variables. See Details.
#' @param source_len default `sf::st_length(source)`. The length of linestrings in the source data set. Used for weighted extensive variables.
#' @param target_len the lengths of each line in the target dataset. Used for intensive variables.
#' @export
rnet_aggregate <- function(
    source,
    matches,
    extensive_vars = NULL,
    intensive_vars = NULL,
    categorical_vars = NULL,
    source_len = sf::st_length(source),
    target_len = NULL
) {


  # target_len cannot be missing when intensive_vars is not null
  # source_len cannot be missing when extensive_vars is not null

  # create column names with suffixes to be used
  ext_nms <- paste0(extensive_vars, "_ext")
  int_nms <- paste0(intensive_vars, "_int")

  # fetch neighbor index
  j <- matches$j

  # get the neighboring values for each of the intensive and extensive vars
  # we give them names of the columns with a suffix based on whether or
  # not it is an extensive or intensive variable
  ij <- rlang::set_names(
    lapply(
      c(extensive_vars, intensive_vars, categorical_vars),
      function(.x) source[[.x]][j]
    ),
    c(ext_nms, int_nms, categorical_vars)
  )

  # combine the `ij` values to the original matches
  wts <- dplyr::bind_cols(matches, ij) |>
    dplyr::mutate(
      # calculate weight for intensive variables
      wt_int = shared_len / as.numeric(target_len[i]),
      # calulate weight for extensive variables
      wt_ext = shared_len / as.numeric(source_len[j])
    )

  # calculate the proportions of categorical variables
  # for matched lines
  # FIXME do we apply weighting like Tobler?
  # https://pysal.org/tobler/generated/tobler.area_weighted.area_interpolate.html#tobler.area_weighted.area_interpolate
  cat_res <- wts |>
    dplyr::reframe(
      dplyr::across(dplyr::all_of(categorical_vars), ~ calc_props(i, .x))
    ) |>
    tidyr::unnest(dplyr::all_of(categorical_vars))

  numeric_res <- wts |>
    dplyr::group_by(i) |>
    dplyr::summarise(
      # handle extensive variables
      dplyr::across(
        dplyr::all_of(ext_nms),
        ~ sum(.x, wt_ext, na.rm = TRUE)
      ),
      # handle intensive variables
      dplyr::across(
        dplyr::all_of(int_nms),
        ~ weighted.mean(.x, wt_int, na.rm = TRUE)
      )
    )

  dplyr::bind_cols(numeric_res, cat_res)
}


calc_props <- function(
    i,
    var,
    name_repair = "unique",
    call = rlang::caller_env()
) {
  # calculate proportions quickly
  prp <- proportions(
    unclass(collapse::qtab(i, var)),
    margin = 1L
  )

  # convert to a dataframe
  res <- as.data.frame(prp)

  # create raw names vector
  raw_names <- paste0(
    deparse(substitute(var)),
    "_",
    colnames(prp)
  )

  # set them to "clean" vctrs names
  colnames(res) <- vctrs::vec_as_names(
    raw_names,
    repair = name_repair,
    call = call
  )
  # return
  res
}
