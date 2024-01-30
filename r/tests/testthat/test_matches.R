# Aim: test that the matching works as expected

test_that("matching works", {
#   x = sf::read_sf(system.file("extdata", "y_negative.geojson", package = "rnetmatch"))
  x = sf::read_sf("https://github.com/nptscot/rnetmatch/raw/main/r/data-raw/geojson/x_negative.geojson")
  y = sf::read_sf("https://github.com/nptscot/rnetmatch/raw/main/r/data-raw/geojson/y_negative.geojson")
  res = rnet_match(x, y, dist_tolerance = 1, angle_tolerance = 30)
  expect_equal(round(res[1, 3]), 2)
})