//! `graham_scan` and `jarvis_march` on inputs below and at three points.

use super::{graham_scan, jarvis_march};
use crate::computational_geometry::types::Point2D;

#[test]
fn short_inputs_come_back_unchanged_and_squares_lose_their_centre() {
    let pts = [Point2D::new(1.0, 2.0), Point2D::new(-1.0, 0.5)];
    for k in 0..=2 {
        assert_eq!(graham_scan(&pts[..k]), pts[..k].to_vec());
        assert_eq!(jarvis_march(&pts[..k]), pts[..k].to_vec());
    }
    let square = [
        Point2D::new(0.0, 0.0),
        Point2D::new(2.0, 0.0),
        Point2D::new(1.0, 1.0),
        Point2D::new(2.0, 2.0),
        Point2D::new(0.0, 2.0),
    ];
    for hull in [graham_scan(&square), jarvis_march(&square)] {
        assert_eq!(hull.len(), 4, "{hull:?}");
        assert!(!hull.contains(&Point2D::new(1.0, 1.0)));
    }
}

#[test]
fn convex_hull_2d_of_short_inputs_and_of_a_square_with_its_centre() {
    use crate::computational_geometry::types::ConvexHull2D;
    let pts = [(1.0, 2.0), (-1.0, 0.5)];
    for k in 0..=2 {
        let hull = ConvexHull2D::compute(pts[..k].to_vec());
        assert_eq!(hull.hull, (0..k).collect::<Vec<usize>>());
    }
    let square = vec![(0.0, 0.0), (2.0, 0.0), (1.0, 1.0), (2.0, 2.0), (0.0, 2.0)];
    let hull = ConvexHull2D::compute(square).hull;
    assert_eq!(hull.len(), 4, "{hull:?}");
    assert!(!hull.contains(&2));
}
