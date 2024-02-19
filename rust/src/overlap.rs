use geo_types::{Line, Point, Rect};
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


// When x range is known but y range is not, we need to solve for start and end points 
// of the line segment
pub fn solve_no_y_overlap(x_overlap: Range<f64>, x: &Line, slope: &f64) -> (Point, Point) {
    let (known_x, known_y) = x.points().0.x_y();
    let b = known_y - (slope * known_x); // Corrected calculation of b

    let y1 = (slope * x_overlap.start) + b;
    let y2 = (slope * x_overlap.end) + b;
    let p1 = Point::new(x_overlap.start, y1);
    let p2 = Point::new(x_overlap.end, y2);
    (p1, p2)  
}

pub fn solve_no_x_overlap(y_overlap: Range<f64>, x: &Line, slope: &f64) -> (Point, Point) {
    let (known_x, known_y) = x.points().0.x_y();
    let b = known_y - (slope * known_x); // Corrected calculation of b

    // create bindings to x vars that will be set in if statement
    let x1;
    let x2;
    
    // handle undefined slope
    if slope.is_infinite() || slope.is_nan() {
        // Assign a constant value to x1 and x2
        x1 = known_x;
        x2 = known_x;
    } else {
        x1 = (y_overlap.start - b) / slope;
        x2 = (y_overlap.end - b) / slope;
    }
    let p1 = Point::new(x1, y_overlap.start);
    let p2 = Point::new(x2, y_overlap.end);
    (p1, p2)  
}

pub fn solve_known_overlaps(
    x_overlap: Range<f64>, 
    y_overlap: Range<f64>, 
    xbb : &Rect) -> (Point, Point) {

        let dy = solve_dy(y_overlap.clone());
        // this is the width of the overlapping bbox
        // let base_w = x_overlap.end - x_overlap.start;
        // this is the heeight of the bbox around xi itself
        // _not_ the bbox of the overlapping area
        let (base_w, base_h) = wh(&xbb);
        // this is the length of the line from the side of the bbox
        // to the end of the line segment
        let dx = solve_dx(dy, base_w, base_h);

        let x1 = x_overlap.end - dx;
        let p1 = Point::new(x1, y_overlap.start);
        let p2 = Point::new(x_overlap.end, y_overlap.end);
        (p1, p2)
}


// get height and width from a Line
// do this by passing in the bounding rectangle
// (width, height)
pub fn wh(x: &Rect) -> (f64, f64) {
    let (x1, y1) = x.min().x_y();
    let (x2, y2) = x.max().x_y();
    (x2 - x1, y2 - y1)
}

// Solve for dy:
// This is the height of the range of Y values
pub fn solve_dy(y_range: Range<f64>) -> f64 {
    y_range.end - y_range.start
}

// base_w is dx2 (the )
fn solve_dx(dy: f64, base_w: f64, base_h: f64) -> f64 {
    dy * base_w / base_h
}

fn solve_h(dx: f64, dy: f64) -> f64 {
    (dx.powi(2) + dy.powi(2)).sqrt()
}
