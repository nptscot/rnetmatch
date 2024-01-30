use geo::{BoundingRect, EuclideanDistance, HaversineDistance};
use geo::{Line, Point};
use rstar::RTreeObject;
use rstar::AABB;


pub enum CrsType {
    Projected,
    Geographic,
}

/// Custom struct to be used to insert into RTree
/// Represents a component `Line` of a target `LineString`.
/// The tuple stores the `Line` struct and the distance buffer to be used.
/// It's [rstar::Envelope] method grows the [rstar::AABB] in x and y directions
/// by the distance.
pub struct TarLine(pub Line<f64>, pub f64);
impl TarLine {
    /// Create an AABB from the contained `Line`
    pub fn envelope(&self) -> AABB<Point> {
        let padding_dist = self.1;
        let bb = self.0.bounding_rect();
        let (ll_x, ll_y) = bb.min().x_y();
        let (ur_x, ur_y) = bb.max().x_y();
        let ll = Point::new(ll_x - padding_dist, ll_y - padding_dist);
        let ur = Point::new(ur_x + padding_dist, ur_y + padding_dist);
        AABB::from_corners(ll, ur)
    }

    /// Using geographic coordinate systems should be avoided with this algorithm.
    /// Measuring distance in geographic space between two lines finds the minimum
    /// distance between vertices whereas the euclidean distance between two lines
    /// considers all possible distances.
    /// Geographic distance may create false negatives.
    pub fn dist_by_crs(&self, other: &Line, crs: &CrsType) -> f64 {
        match crs {
            CrsType::Projected => self.0.euclidean_distance(other),
            CrsType::Geographic => {
                let x = self.0;
                x.start_point().haversine_distance(&other.start_point())
                    .min(
                        x.start_point().haversine_distance(&other.end_point())
                    )
                    .min(
                        x.end_point().haversine_distance(&other.start_point())
                    )
                    .min(
                        x.end_point().haversine_distance(&other.end_point())
                    )
            },

        }
    }
}

impl RTreeObject for TarLine {
    type Envelope = AABB<Point>;
    fn envelope(&self) -> Self::Envelope {
        self.envelope()
    }
}
