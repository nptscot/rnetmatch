use geo_types::Rect;
use std::ops::Range;

// TODO for handling geographic CRS
// calculate the distance from the top left to the bottom left corners
pub fn x_range(rect: &Rect) -> Range<f64> {
    rect.min().x..rect.max().x
}

// TODO for handling geographic CRS
// calculate the distance from the top left to the top right corners
pub fn y_range(rect: &Rect) -> Range<f64> {
    rect.min().y..rect.max().y
}

pub fn overlap_range(r1: Range<f64>, r2: Range<f64>) -> Option<Range<f64>> {
    if r1.end < r2.start || r2.end < r1.start {
        None
    } else {
        Some(r1.start.max(r2.start)..r1.end.min(r2.end))
    }
}

// Given the overlap in the domain and range
// we can calculate the segment lenth of the line that is provided
// we use the bounding box as a &Rect to determine the width or height
// of the triangle
// TODO: solve_segment_length does not handle geographic CRS yet 
// TO support geographic CRS overlap_range() needs to be able to calculate haversine distance for the overlap
// this will probably need to be handle by x_range and y_range?
pub fn solve_segment_length(
    x_overlap: Option<Range<f64>>,
    y_overlap: Option<Range<f64>>,
    bbox: &Rect,
) -> f64 {
    if x_overlap.is_some() && y_overlap.is_some() {
        let (base_w, base_h) = wh(&bbox);
        let dy = solve_dy(y_overlap.unwrap());
        let dx = solve_dx(dy, base_w, base_h);
        solve_h(dx, dy)
    } else if x_overlap.is_some() {
        let x_over = x_overlap.unwrap();
        x_over.end - x_over.start
    } else if y_overlap.is_some() {
        let y_over = y_overlap.unwrap();
        y_over.end - y_over.start
    } else {
        unreachable!() // this should never happen
    }
}

// get height and width from a Line
// do this by passing in the bounding rectangle
// (width, height)
fn wh(x: &Rect) -> (f64, f64) {
    let (x1, y1) = x.min().x_y();
    let (x2, y2) = x.max().x_y();
    (x2 - x1, y2 - y1)
}

// Solve for dy:
// This is the height of the range of Y values
fn solve_dy(y_range: Range<f64>) -> f64 {
    y_range.end - y_range.start
}

// base_w is dx2 (the )
fn solve_dx(dy: f64, base_w: f64, base_h: f64) -> f64 {
    dy * base_w / base_h
}

fn solve_h(dx: f64, dy: f64) -> f64 {
    (dx.powi(2) + dy.powi(2)).sqrt()
}
