# Demonstration of matching route networks


``` r
library(stplanr)
library(sf)
```

    Linking to GEOS 3.11.1, GDAL 3.6.4, PROJ 9.1.1; sf_use_s2() is TRUE

``` r
rnet_y = route_network_small["flow"]
rnet_y$id = 1:nrow(rnet_y)
# The target object
rnet_x = rnet_subset(osm_net_example[1], rnet_y)
```

    Warning: attribute variables are assumed to be spatially constant throughout
    all geometries

    Warning in st_cast.sf(sf::st_cast(x, "MULTILINESTRING"), "LINESTRING"):
    repeating attributes for all sub-geometries for which they may not be constant

    Joining with `by = join_by(osm_id)`

``` r
# Get start and end points for each linestring in rnet_x:
rnet_x_s = lwgeom::st_startpoint(rnet_x)
rnet_x_e = lwgeom::st_endpoint(rnet_x)
plot(rnet_x$geometry, lwd = 9, col = "lightgrey")
plot(rnet_y["flow"], add = TRUE, lwd = 5)
plot(rnet_x_s, add = TRUE, pch = 3)
plot(rnet_x_e, add = TRUE, pch = 3)
```

![](demo_files/figure-commonmark/unnamed-chunk-1-1.png)

The stplanr solution was as follows:

``` r
rnet_y$quietness <- rnorm(nrow(rnet_y))
funs <- list(flow = sum, quietness = mean)
rnet_merged <- rnet_merge(rnet_x[1], rnet_y[c("flow", "quietness")],
  dist = 9, segment_length = 20, funs = funs
)
```

    Warning in line_segment_rsgeo(l, n_segments = n_segments): The CRS of the input object is latlon.
    This may cause problems with the rsgeo implementation of line_segment().

    Joining with `by = join_by(osm_id)`

``` r
plot(rnet_y$geometry, lwd = 5, col = "lightgrey")
plot(rnet_merged["flow"], add = TRUE, lwd = 2)
```

![](demo_files/figure-commonmark/unnamed-chunk-2-1.png)

Note that this leaves gaps in the network. Also, the join syntax is a
bit clunky.

Let’s try doing the same thing but with `geos`. First we’ll reproject
the data.

``` r
rnet_x_projected = st_transform(rnet_x, "EPSG:27700")
rnet_y_projected = st_transform(rnet_y, "EPSG:27700")
```

Then we’ll convert to `geos` objects.

``` r
rnet_x_geos = geos::as_geos_geometry(rnet_x_projected)
rnet_y_geos = geos::as_geos_geometry(rnet_y_projected)
```

We’ll create a buffer around `rnet_x`:

``` r
params = geos::geos_buffer_params(end_cap_style = "flat")
rnet_x_buffer = geos::geos_buffer(rnet_x_geos, 9, params = params)
plot(rnet_x_buffer)
plot(rnet_y_geos, add = TRUE, col = "red", lwd = 2)
```

![](demo_files/figure-commonmark/unnamed-chunk-5-1.png)

Now let’s ‘chop’ the source geometry into segments that fit within the
buffer:

``` r
rnet_xbl = geos::geos_boundary(rnet_x_buffer)
rnet_xblb = geos::geos_buffer(rnet_xbl, 3, params = params)
rnet_xlbc = geos::geos_make_collection(rnet_xblb)
rnet_xlbcu = geos::geos_unary_union(rnet_xlbc)
rnet_y_chopped = geos::geos_intersection(
  rnet_y_geos,
  rnet_xlbcu
)
plot(rnet_xlbcu, col = "lightgrey")
```

![](demo_files/figure-commonmark/unnamed-chunk-6-1.png)

``` r
plot(rnet_xlbcu, col = "lightgrey")
plot(rnet_y_chopped, add = TRUE, col = "red", lwd = 2)
```

![](demo_files/figure-commonmark/unnamed-chunk-7-1.png)

The red bits are the parts of the source geometry `rnet_y` that we
*don’t* want. Let’s get the bits that we *do* want:

``` r
rnet_y_chopped = geos::geos_difference(
  rnet_y_geos,
  rnet_xlbcu
)
plot(rnet_xlbcu)
plot(rnet_x_buffer, add = TRUE, col = "lightgrey", border = NA)
plot(rnet_y_chopped, add = TRUE, col = "red", lwd = 2)
```

![](demo_files/figure-commonmark/unnamed-chunk-8-1.png)

For every ‘chopped’ linestring there is at least one matching linestring
in `rnet_y`. Let’s find them as follows:

``` r
rnet_ycj = geos::geos_inner_join_keys(
  rnet_y_chopped,
  rnet_y_geos
)
plot(rnet_ycj)
```

![](demo_files/figure-commonmark/unnamed-chunk-9-1.png)

That shows that we’re not quite ready to join the data: we need to
exclude the red bits that are outside the original buffer.

``` r
rnet_ycl = geos::geos_unnest(rnet_y_chopped, keep_multi = FALSE)
rnet_ycj = geos::geos_inner_join_keys(
  rnet_ycl,
  rnet_y_geos
)
plot(rnet_ycj)
```

![](demo_files/figure-commonmark/unnamed-chunk-11-1.png)

We can also join `rnet_y_chopped` and `rnet_ycl` to `rnet_x_buffer` to
get the buffer geometry:

``` r
rnet_ycj = geos::geos_inner_join_keys(
  rnet_y_chopped,
  rnet_x_buffer
)
nrow(rnet_ycj)
```

    [1] 19

``` r
length(rnet_y_chopped)
```

    [1] 8

``` r
length(unique(rnet_ycj$x))
```

    [1] 7

``` r
length(unique(rnet_ycj$y))
```

    [1] 15

``` r
plot(rnet_ycj)
```

![](demo_files/figure-commonmark/unnamed-chunk-12-1.png)

``` r
rnet_y
```

    Simple feature collection with 8 features and 3 fields
    Geometry type: LINESTRING
    Dimension:     XY
    Bounding box:  xmin: -1.533105 ymin: 53.80045 xmax: -1.526994 ymax: 53.80365
    Geodetic CRS:  WGS 84
       flow                       geometry id  quietness
    1   128 LINESTRING (-1.530829 53.80...  1 -0.1358755
    2   162 LINESTRING (-1.531674 53.80...  2 -0.1421779
    3   244 LINESTRING (-1.531674 53.80...  3 -0.5568811
    5   466 LINESTRING (-1.53293 53.802...  4  1.1316703
    6   540 LINESTRING (-1.53293 53.802...  5 -1.5137988
    8   784 LINESTRING (-1.529524 53.80...  6  0.1040959
    9   912 LINESTRING (-1.528802 53.80...  7 -0.2704893
    10 1006 LINESTRING (-1.532978 53.80...  8 -0.6370883

``` r
rnet_yclj = geos::geos_inner_join_keys(
  rnet_ycl,
  rnet_x_buffer
)
nrow(rnet_yclj)
```

    [1] 50

``` r
length(rnet_ycl)
```

    [1] 25

``` r
plot(rnet_yclj)
```

![](demo_files/figure-commonmark/unnamed-chunk-12-2.png)

``` r
rnet_y
```

    Simple feature collection with 8 features and 3 fields
    Geometry type: LINESTRING
    Dimension:     XY
    Bounding box:  xmin: -1.533105 ymin: 53.80045 xmax: -1.526994 ymax: 53.80365
    Geodetic CRS:  WGS 84
       flow                       geometry id  quietness
    1   128 LINESTRING (-1.530829 53.80...  1 -0.1358755
    2   162 LINESTRING (-1.531674 53.80...  2 -0.1421779
    3   244 LINESTRING (-1.531674 53.80...  3 -0.5568811
    5   466 LINESTRING (-1.53293 53.802...  4  1.1316703
    6   540 LINESTRING (-1.53293 53.802...  5 -1.5137988
    8   784 LINESTRING (-1.529524 53.80...  6  0.1040959
    9   912 LINESTRING (-1.528802 53.80...  7 -0.2704893
    10 1006 LINESTRING (-1.532978 53.80...  8 -0.6370883

Now let’s join the dataframe versions:

``` r
rnet_y_df = sf::st_drop_geometry(rnet_y)
rnet_y_df_expanded = rnet_y_df[rnet_ycj$x, ]
rnet_x_df = sf::st_drop_geometry(rnet_x)
rnet_x_df_expanded = rnet_x_df[rnet_ycj$y, ]
rnet_y_df_expanded$osm_id = rnet_x_df_expanded$osm_id
nrow(rnet_y_df_expanded)
```

    [1] 19

``` r
nrow(rnet_x)
```

    [1] 16

``` r
rnet_x_agg = rnet_y_df_expanded |>
  dplyr::group_by(osm_id) |>
  dplyr::summarise(
    flow = sum(flow),
    quietness = mean(quietness)
  )
rnet_x_joined = dplyr::left_join(
    rnet_x,
    rnet_x_agg,
)
```

    Joining with `by = join_by(osm_id)`

``` r
plot(rnet_x_joined["flow"])
```

![](demo_files/figure-commonmark/unnamed-chunk-13-1.png)

Let’s compare the old and new joined flows:

``` r
# place plots side by side
par(mfrow = c(1, 2))
plot(rnet_y$geometry, lwd = 5, col = "lightgrey")
plot(rnet_merged["flow"], add = TRUE, lwd = 2)

plot(rnet_y$geometry, lwd = 5, col = "lightgrey")
plot(rnet_x_joined["flow"], add = TRUE, lwd = 2)
```

![](demo_files/figure-commonmark/unnamed-chunk-14-1.png)

``` r
par(mfrow = c(1, 1))
```

To package this up we used the following commands:

``` r
usethis::use_description()
usethis::use_package("geos")
```

We created a function that takes `rnet_x` and `rnet_y` and returns a
dataframe with the joined data:

``` r
rnet_x = rnet_x_projected
rnet_y = rnet_y_projected
distance = 9
dist_chop = 0.1
rnet_join = function(
    rnet_x,
    rnet_y,
    distance = 9,
    dist_chop = 0.1
    ) {
  # Check CRS is projected:
  stopifnot(
    sf::st_is_longlat(rnet_x) == FALSE,
    sf::st_is_longlat(rnet_y) == FALSE
  )
  rnet_x_geos = geos::as_geos_geometry(rnet_x)
  rnet_y_geos = geos::as_geos_geometry(rnet_y)
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
  rnet_y_df = sf::st_drop_geometry(rnet_y)
  rnet_y_df_expanded = rnet_y_df[rnet_ycj$x, ]
  rnet_y_df_expanded$id_x = rnet_x_df[[1]][rnet_ycj$y]
  # Replace 'id_x' with the name of the first column in rnet_x_df:
  nx = which(names(rnet_y_df_expanded) == "id_x")
  new_name = names(rnet_x_df)[1]
  names(rnet_y_df_expanded)[nx] = new_name
  rnet_y_df_expanded
}
# Test it:
rnet_join(rnet_x, rnet_y)
```

        flow id  quietness    osm_id
    1    128  1 -0.1358755 619241249
    1.1  128  1 -0.1358755  34359804
    1.2  128  1 -0.1358755  25024600
    1.3  128  1 -0.1358755  23120679
    2    162  2 -0.1421779 169555938
    2.1  162  2 -0.1421779 145796711
    2.2  162  2 -0.1421779  34359804
    2.3  162  2 -0.1421779   6072857
    3    244  3 -0.5568811   6072857
    3.1  244  3 -0.5568811 169591262
    5    466  4  1.1316703 145796711
    5.1  466  4  1.1316703 162489416
    5.2  466  4  1.1316703 162489422
    5.3  466  4  1.1316703  34423763
    5.4  466  4  1.1316703  34423636
    5.5  466  4  1.1316703  53014870
    5.6  466  4  1.1316703   6072857
    6    540  5 -1.5137988  38422455
    6.1  540  5 -1.5137988   6072857
    6.2  540  5 -1.5137988 169591262
    6.3  540  5 -1.5137988 169591263
    6.4  540  5 -1.5137988 440408598
    8    784  6  0.1040959  23120679
    9    912  7 -0.2704893  23120679
