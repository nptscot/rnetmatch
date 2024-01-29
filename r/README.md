

For local development run this in the `r` directory:

``` r
devtools::load_all()
```

    ℹ Loading rnetmatch

``` r
library(dplyr)
```


    Attaching package: 'dplyr'

    The following objects are masked from 'package:stats':

        filter, lag

    The following objects are masked from 'package:base':

        intersect, setdiff, setequal, union

Basic test

``` r
x = sf::read_sf("data-raw/geojson/princes_street_minimal_x_1.geojson")
x = x |> sf::st_transform("EPSG:27700")
y = sf::read_sf("data-raw/geojson/princes_street_minimal.geojson")
y = y |> sf::st_transform("EPSG:27700")
```

``` r
bind_rows(
  x |>
    transmute(value = "x"),
  y |>
    transmute(value = as.character(value))
) |>
  plot(lwd = 5)
```

<img src="README_files/figure-commonmark/fig-x-and-y-1.png"
id="fig-x-and-y" />

``` r
matched_df = rnet_match(x, y, dist_tolerance = 20, slope_tolerance = 0.1, trees = "xy")
y_to_match = y |>
  transmute(j = 1:n(), value) |>
  sf::st_drop_geometry() 
matched_df_y = left_join(matched_df, y_to_match)
```

    Joining with `by = join_by(j)`

``` r
x$length_x = sf::st_length(x) |>
  # Drop units:
  units::drop_units()
x_to_match = x |>
  transmute(i = 1:n(), length_x) |>
  sf::st_drop_geometry()
matched_df_xy = left_join(matched_df_y, x_to_match)
```

    Joining with `by = join_by(i)`

``` r
x_formula = matched_df_xy |>
  mutate(value_formula = value * shared_len / length_x)
x_formula
```

      i j shared_len value length_x value_formula
    1 1 4 66.1969445     4 71.68314   3.693863952
    2 1 1  0.5893015     1 71.68314   0.008220921
    3 2 2 17.3321012     2 64.16447   0.540239849
    4 2 1 57.0626066     1 64.16447   0.889317851
    5 2 3 12.5390962     3 64.16447   0.586263551
    6 3 3 69.7763265     3 77.58872   2.697930711
    7 3 2 67.8390165     2 77.58872   1.748682541

``` r
x_with_values = x_formula |>
  group_by(id = i) |>
  summarise(value = sum(value_formula))
x_joined = left_join(x, x_with_values)
```

    Joining with `by = join_by(id)`

``` r
y |>
  select(value) |>
  plot(lwd = 5)
x_joined |>
  select(value) |>
  plot(lwd = 5)
```

<img src="README_files/figure-commonmark/fig-x_joined-1.png"
id="fig-x_joined-1" />

<img src="README_files/figure-commonmark/fig-x_joined-2.png"
id="fig-x_joined-2" />
