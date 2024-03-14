

This is an example showing how `rnetmatch` can be used with `st_join`,
from issue [\#35](https://github.com/nptscot/rnetmatch/issues/35).

``` r
library(sf)
```

    Linking to GEOS 3.12.1, GDAL 3.8.4, PROJ 9.3.1; sf_use_s2() is TRUE

``` r
library(rnetmatch)
crop_box <- st_bbox(c("xmin" = 427200, xmax = 427500, ymin = 433550, ymax = 433700))

rnet_y <- "https://raw.githubusercontent.com/nptscot/networkmerge/main/data/rnet_armley.geojson" |> 
  read_sf() |>
  # st_geometry() |> 
  st_transform(27700) |> 
  st_crop(crop_box)
```

    Warning: attribute variables are assumed to be spatially constant throughout
    all geometries

``` r
rnet_x <- "https://raw.githubusercontent.com/nptscot/networkmerge/main/data/rnet_armley_line.geojson" |>
  read_sf() |> 
  st_crop(crop_box)

# create matches
matches <- rnetmatch::rnet_match(rnet_x, rnet_y, 10, 10)

# function with signature to work with st_join
match_keys <- function(.x = NULL, .y = NULL, matches) {
  dplyr::left_join(tibble::tibble(i = 1:max(matches$i)), matches) |> 
    dplyr::group_by(i) |> 
    dplyr::summarise(j = list(c(j))) |> 
    tibble::deframe() |> 
    lapply(\(.x) {
      if (all(is.na(.x))) {
        integer()
      } else {
        .x
      }
    })
}

x_y_joined_rnetmatch_10_10 <- st_join(
  rnet_x,
  rnet_y,
  join = match_keys,
  # pass matches via dots 
  matches = matches
)
```

    Joining with `by = join_by(i)`
