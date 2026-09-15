//! 2D profile geometry behind Petunia abstractions (`geo`, P0-03).
//!
//! The draw-profile tool and planar checks work on plain `[[f64; 2]]` rings.
//! `geo` types never leak through this boundary: predicates, area, winding
//! and intersection tests take and return Petunia-owned data.

use geo::{Area, Contains, Intersects, LineString, Polygon};

/// Profile validation failure.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum ProfileGeoError {
    /// Fewer than 3 distinct points.
    #[error("profile needs at least 3 points")]
    TooFewPoints,
    /// A coordinate is `NaN` or infinite.
    #[error("profile contains non-finite coordinates")]
    NonFinite,
}

/// Checks points are finite and returns them as a closed `LineString`.
fn ring_of(points: &[[f64; 2]]) -> Result<LineString<f64>, ProfileGeoError> {
    if points.len() < 3 {
        return Err(ProfileGeoError::TooFewPoints);
    }
    if points
        .iter()
        .any(|p| !p[0].is_finite() || !p[1].is_finite())
    {
        return Err(ProfileGeoError::NonFinite);
    }
    let mut coords: Vec<[f64; 2]> = points.to_vec();
    if coords.first() != coords.last() {
        let first = coords[0];
        coords.push(first);
    }
    Ok(LineString::from(coords))
}

/// Unsigned area of a profile ring.
pub fn profile_area(points: &[[f64; 2]]) -> Result<f64, ProfileGeoError> {
    let ring = ring_of(points)?;
    Ok(Polygon::new(ring, vec![]).unsigned_area())
}

/// Signed area: positive means counter-clockwise winding.
pub fn profile_signed_area(points: &[[f64; 2]]) -> Result<f64, ProfileGeoError> {
    let ring = ring_of(points)?;
    Ok(Polygon::new(ring, vec![]).signed_area())
}

/// Whether the ring is explicitly counter-clockwise.
pub fn is_counter_clockwise(points: &[[f64; 2]]) -> Result<bool, ProfileGeoError> {
    Ok(profile_signed_area(points)? > 0.0)
}

/// Whether `point` lies strictly inside the profile ring.
pub fn profile_contains(points: &[[f64; 2]], point: [f64; 2]) -> Result<bool, ProfileGeoError> {
    let ring = ring_of(points)?;
    let polygon = Polygon::new(ring, vec![]);
    Ok(polygon.contains(&geo::Point::new(point[0], point[1])))
}

/// Whether two profile rings intersect (touching counts).
pub fn profiles_intersect(a: &[[f64; 2]], b: &[[f64; 2]]) -> Result<bool, ProfileGeoError> {
    let ra = ring_of(a)?;
    let rb = ring_of(b)?;
    Ok(Polygon::new(ra, vec![]).intersects(&Polygon::new(rb, vec![])))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SQUARE: [[f64; 2]; 4] = [[0.0, 0.0], [2.0, 0.0], [2.0, 2.0], [0.0, 2.0]];
    const TRIANGLE: [[f64; 2]; 3] = [[0.0, 0.0], [4.0, 0.0], [0.0, 3.0]];

    #[test]
    fn square_area_and_winding() {
        assert_eq!(profile_area(&SQUARE).unwrap(), 4.0);
        assert!(is_counter_clockwise(&SQUARE).unwrap());
        let mut clockwise = SQUARE;
        clockwise.reverse();
        assert!(!is_counter_clockwise(&clockwise).unwrap());
    }

    #[test]
    fn triangle_area() {
        assert_eq!(profile_area(&TRIANGLE).unwrap(), 6.0);
    }

    #[test]
    fn containment_and_intersection() {
        assert!(profile_contains(&SQUARE, [1.0, 1.0]).unwrap());
        assert!(!profile_contains(&SQUARE, [5.0, 5.0]).unwrap());
        let overlapping = [[1.0, 1.0], [3.0, 1.0], [3.0, 3.0], [1.0, 3.0]];
        let far = [[10.0, 10.0], [12.0, 10.0], [12.0, 12.0], [10.0, 12.0]];
        assert!(profiles_intersect(&SQUARE, &overlapping).unwrap());
        assert!(!profiles_intersect(&SQUARE, &far).unwrap());
    }

    #[test]
    fn degenerate_inputs_are_rejected() {
        assert_eq!(
            profile_area(&[[0.0, 0.0]]),
            Err(ProfileGeoError::TooFewPoints)
        );
        assert_eq!(
            profile_area(&[[0.0, 0.0], [f64::NAN, 0.0], [0.0, 1.0]]),
            Err(ProfileGeoError::NonFinite)
        );
    }
}
