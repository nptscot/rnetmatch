rnet_join_geos = function(
    rnet_x,
    rnet_y,
    distance = 9,
    dist_chop = 0.1,
    rnet_y_df = NULL,
    rnet_x_df = NULL) {
  if (is(rnet_x, "sf") && is(rnet_y, "sf")) {
    rnet_x_geos = geos::as_geos_geometry(rnet_x)
    rnet_y_geos = geos::as_geos_geometry(rnet_y)
    # stopifnot(
    #   sf::st_is_longlat(rnet_x) == FALSE,
    #   sf::st_is_longlat(rnet_y) == FALSE
    # )
  } else if (is(rnet_x, "geos_geometry") && is(rnet_y, "geos_geometry")) {
    rnet_x_geos = rnet_x
    rnet_y_geos = rnet_y
  } else {
    stop("rnet_x and rnet_y must be sf objects or geos_geometry objects")
  }
  rnet_ycj = rnet_match_geos(
    rnet_x_geos,
    rnet_y_geos,
    distance = distance,
    dist_chop = dist_chop
  )
  if (is.null(rnet_y_df)) {
    rnet_y_df = sf::st_drop_geometry(rnet_y)
  }
  if (is.null(rnet_x_df)) {
    rnet_x_df = sf::st_drop_geometry(rnet_x)
  }
  rnet_y_df_expanded = rnet_y_df[rnet_ycj$x, ]
  rnet_y_df_expanded$id_x = rnet_x_df[[1]][rnet_ycj$y]
  # Replace 'id_x' with the name of the first column in rnet_x_df:
  nx = which(names(rnet_y_df_expanded) == "id_x")
  new_name = names(rnet_x_df)[1]
  names(rnet_y_df_expanded)[nx] = new_name
  rnet_y_df_expanded
}

rnet_match_geos = function(
    rnet_x_geos,
    rnet_y_geos,
    distance = 9,
    dist_chop = 0.1) {
  params = geos::geos_buffer_params(end_cap_style = "flat")
  rnet_x_buffer = geos::geos_buffer(rnet_x_geos, distance, params = params)
  rnet_xbl = geos::geos_boundary(rnet_x_buffer)
  rnet_xblb = geos::geos_buffer(rnet_xbl, dist_chop, params = params)
  rnet_xlbc = geos::geos_make_collection(rnet_xblb)
  rnet_xlbcu = geos::geos_unary_union(rnet_xlbc)
  rnet_y_chopped = geos::geos_difference(
    rnet_y_geos,
    rnet_xlbcu
  )
  rnet_ycj = geos::geos_inner_join_keys(
    rnet_y_chopped,
    rnet_x_buffer
  )
  rnet_ycj
}