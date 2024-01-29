use geo::BoundingRect;
use std::collections::BTreeMap;

mod overlap;
pub use crate::overlap::*;

mod structs;
pub use crate::structs::*;

mod trees;
pub use crate::trees::*;

pub fn find_candidates(
    x: impl Iterator<Item = geo_types::LineString>,
    y: impl Iterator<Item = geo_types::LineString>,
    dist: f64,
    slope_tolerance: f64,
    crs_type: CrsType,
) -> BTreeMap<i32, Vec<(i32, f64)>> {
    let mut matches: BTreeMap<i32, Vec<(i32, f64)>> = BTreeMap::new();
    let source_tree = create_source_rtree(x);
    let target_tree = create_target_rtree(y, dist);
    let candidates = source_tree.intersection_candidates_with_other_tree(&target_tree);

    candidates.for_each(|(cx, cy)| {
        let xbb = cx.geom().bounding_rect();
        let ybb = cy.geom().0.bounding_rect();

        // extract cached slopes and index positions
        let (i, x_slope) = cx.data;
        let (j, y_slope) = cy.data;

        // compare slopes:
        // TODO handle vertical lines
        // TODO use degrees or radians instead of calculated slope
        let is_tolerant = (x_slope - y_slope).abs() < slope_tolerance;

        // if the slopes are within tolerance then we check for overlap
        if is_tolerant {
            let x_overlap = overlap_range(x_range(&xbb), x_range(&ybb));
            let y_overlap = overlap_range(y_range(&xbb), y_range(&ybb));

            // if theres overlap then we do a distance based check
            // following, check that they're within distance tolerance,
            // if so, calculate the shared length
            if x_overlap.is_some() || y_overlap.is_some() {
                // calculate the distance from the line segment
                // if its within our threshold we include it;
                let d = cy.geom().dist_by_crs(&cx.geom(), &crs_type);
                // if distance is less than or equal to tolerance, add the key
                if d <= dist {
                    let shared_len = solve_segment_length(x_overlap, y_overlap, &xbb);
                    // TODO remove addition of 1 for R indexing
                    // add 1 for R indexing
                    // ensures that no duplicates are inserted. Creates a new empty vector is needed
                    let entry = matches.entry((i + 1) as i32).or_insert_with(Vec::new);
                    let j_plus_one = (j + 1) as i32;

                    if let Some(tuple) = entry.iter_mut().find(|(x, _)| *x == j_plus_one) {
                        tuple.1 += shared_len;
                    } else {
                        entry.extend(std::iter::once((j_plus_one, shared_len)));
                    }
                }
            }
        }
    });
    matches
}

pub fn find_candidates_one_tree(
    x: impl Iterator<Item = geo_types::LineString>,
    y: impl Iterator<Item = geo_types::LineString>,
    distance_tolerance: f64,
    slope_tolerance: f64,
    crs_type: CrsType,
) -> BTreeMap<i32, Vec<(i32, f64)>> {
    let mut matches: BTreeMap<i32, Vec<(i32, f64)>> = BTreeMap::new();
    let source_tree = create_source_rtree(x);

    let _ = y.enumerate().for_each(|(j, lns)| {
        lns.lines().for_each(|li| {
            let t = TarLine(li, distance_tolerance);
            let envelope = t.envelope();
            let ybb = li.bounding_rect();
            let candidates = source_tree.locate_in_envelope_intersecting(&envelope);

            candidates.for_each(|cx| {
                let xbb = cx.geom().bounding_rect();
                let (i, x_slope) = cx.data;
                let y_slope = li.slope();
                // compare slopes:
                let is_tolerant = (x_slope - y_slope).abs() < slope_tolerance;

                // if the slopes are within tolerance then we check for overlap
                if is_tolerant {
                    let x_overlap = overlap_range(x_range(&xbb), x_range(&ybb));
                    let y_overlap = overlap_range(y_range(&xbb), y_range(&ybb));

                    // if theres overlap then we do a distance based check
                    // following, check that they're within distance tolerance,
                    // if so, calculate the shared length
                    if x_overlap.is_some() || y_overlap.is_some() {
                        // calculate the distance from the line segment
                        // if its within our threshold we include it;
                        let d = t.dist_by_crs(&cx.geom(), &crs_type);
                        // if distance is less than or equal to tolerance, add the key
                        if d <= distance_tolerance {
                            // solve_segment_length() will always calculate in R2 space
                            let shared_len = solve_segment_length(x_overlap, y_overlap, &xbb);
                            // add 1 for R indexing
                            // ensures that no duplicates are inserted. Creates a new empty vector is needed
                            let entry = matches.entry((i + 1) as i32).or_insert_with(Vec::new);
                            let j_plus_one = (j + 1) as i32;

                            if let Some(tuple) = entry.iter_mut().find(|(x, _)| *x == j_plus_one) {
                                tuple.1 += shared_len;
                            } else {
                                entry.extend(std::iter::once((j_plus_one, shared_len)));
                            }
                        }
                    }
                }
            })
        })
    });
    matches
}
