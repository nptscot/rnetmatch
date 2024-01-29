use crate::structs::TarLine;
use rstar::primitives::{CachedEnvelope, GeomWithData};

/// Create an RTree from LineStringArray<i64>
///
/// Creates an R* Tree using cached envelopes for each Line in a LineString.
/// In addition to the envelope, it stores the slope, and the index of the LineString.
pub fn create_source_rtree(
    x: impl Iterator<Item = geo_types::LineString>,
) -> rstar::RTree<GeomWithData<CachedEnvelope<geo_types::Line>, (usize, f64)>> {
    let to_insert = x
        .enumerate()
        .flat_map(|(i, xi)| {
            let components = xi
                .lines()
                .map(|li| {
                    let slope = li.slope();
                    let env = CachedEnvelope::new(li);
                    GeomWithData::new(env, (i, slope))
                })
                .collect::<Vec<GeomWithData<_, _>>>();
            components
        })
        .collect::<Vec<_>>();

    rstar::RTree::bulk_load(to_insert)
}

pub fn create_target_rtree(
    y: impl Iterator<Item = geo_types::LineString>,
    dist: f64,
) -> rstar::RTree<GeomWithData<CachedEnvelope<TarLine>, (usize, f64)>> {
    let to_insert = y
        .enumerate()
        .flat_map(|(i, yi)| {
            let components = yi
                .lines()
                .map(|li| {
                    let tl = TarLine(li, dist);
                    let slope = li.slope();
                    let env = CachedEnvelope::new(tl);
                    GeomWithData::new(env, (i, slope))
                })
                .collect::<Vec<GeomWithData<_, _>>>();
            components
        })
        .collect::<Vec<_>>();

    rstar::RTree::bulk_load(to_insert)
}
