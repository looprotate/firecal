//! A polygon on the globe as an area: the cells whose centers it contains.
//!
//! Longitude jumps of more than half the globe between two vertices are read as crossing the antimeridian, so
//! a polygon from 170 east to 170 west is 20 degrees wide, not 340. A vertex may sit on a pole. A polygon that
//! goes all the way around a pole is refused: it has no longitude at which to cut it.

#![cfg_attr(
    test,
    expect(
        clippy::suboptimal_flops,
        reason = "only with std, which tests have: fused multiply-add rounds once, and wasm must round like native"
    )
)]

use crate::area::{column_center, columns_between, row_center, rows_between};
use crate::{Area, QueryError};
use core::ops::Range;
use firecal_types::GRID_COLUMNS;

const MIN_VERTICES: usize = 3;
const EMPTY_AREA_DEGREES_SQUARED: f64 = 1e-12;

/// A corner of a polygon, in degrees.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
    /// Latitude, -90 to 90.
    pub latitude: f64,
    /// Longitude, -180 to 180.
    pub longitude: f64,
}

/// A simple polygon with at least three corners, as an [`Area`].
///
/// It selects the cells whose center is inside. A cell whose center is exactly on an edge belongs to the polygon on one side only, so two polygons that
/// share an edge never both count it.
#[derive(Debug, Clone, Copy)]
pub struct PolygonArea<'a> {
    unwrapped: &'a [Vertex],
    rows: (u32, u32),
    columns: (u32, u32),
}

impl<'a> PolygonArea<'a> {
    /// The polygon with corners `vertices`, in either winding. A last corner equal to the first only closes
    /// the ring. `scratch` holds the corners with their longitudes continued across the antimeridian, and
    /// must be at least as long as `vertices`; the polygon borrows it.
    ///
    /// # Errors
    /// [`QueryError::LatitudeOutOfRange`] and [`QueryError::LongitudeOutOfRange`] for a corner off the globe;
    /// [`QueryError::EmptyArea`] for fewer than three corners or no area; [`QueryError::SelfIntersecting`] when
    /// edges cross or touch; [`QueryError::EnclosesPole`]; [`QueryError::OutputLengthMismatch`] when `scratch`
    /// is too short.
    pub fn new(vertices: &[Vertex], scratch: &'a mut [Vertex]) -> Result<Self, QueryError> {
        let corners = without_closing_corner(vertices);
        if corners.len() < MIN_VERTICES {
            return Err(QueryError::EmptyArea);
        }
        let unwrapped = scratch
            .get_mut(..corners.len())
            .ok_or(QueryError::OutputLengthMismatch)?;
        check_on_globe(corners)?;
        continue_across_antimeridian(corners, unwrapped)?;
        let unwrapped: &'a [Vertex] = unwrapped;
        if has_crossing_edges(unwrapped) {
            return Err(QueryError::SelfIntersecting);
        }
        if twice_area(unwrapped).abs() < EMPTY_AREA_DEGREES_SQUARED {
            return Err(QueryError::EmptyArea);
        }
        let (south, north) = extent(unwrapped, |corner| corner.latitude);
        let (west, east) = extent(unwrapped, |corner| corner.longitude);
        let rows = rows_between(south, north);
        let columns = if west < -180.0 || east > 180.0 {
            0..GRID_COLUMNS
        } else {
            columns_between(west, east)
        };
        Ok(Self {
            unwrapped,
            rows: (rows.start, rows.end),
            columns: (columns.start, columns.end),
        })
    }
}

impl Area for PolygonArea<'_> {
    fn rows(&self) -> Range<u32> {
        self.rows.0..self.rows.1
    }

    fn columns(&self) -> Range<u32> {
        self.columns.0..self.columns.1
    }

    fn selects(&self, row: u32, col: u32) -> bool {
        let (latitude, longitude) = (row_center(row), column_center(col));
        [-360.0, 0.0, 360.0]
            .iter()
            .any(|shift| contains(self.unwrapped, latitude, longitude + shift))
    }
}

fn without_closing_corner(vertices: &[Vertex]) -> &[Vertex] {
    match (vertices.first(), vertices.last()) {
        (Some(first), Some(last)) if vertices.len() > 1 && first == last => {
            vertices.get(..vertices.len() - 1).unwrap_or(vertices)
        }
        _ => vertices,
    }
}

fn check_on_globe(corners: &[Vertex]) -> Result<(), QueryError> {
    for corner in corners {
        if !(-90.0..=90.0).contains(&corner.latitude) {
            return Err(QueryError::LatitudeOutOfRange);
        }
        if !(-180.0..=180.0).contains(&corner.longitude) {
            return Err(QueryError::LongitudeOutOfRange);
        }
    }
    Ok(())
}

/// The change of longitude from `from` to `to` going the short way round, -180 to 180.
fn short_way(from: f64, to: f64) -> f64 {
    let change = to - from;
    if change > 180.0 {
        change - 360.0
    } else if change < -180.0 {
        change + 360.0
    } else {
        change
    }
}

/// Writes the corners with each longitude continued from the one before, so an edge over the antimeridian is
/// short. Fails when the ring does not close, which means it goes around a pole.
fn continue_across_antimeridian(corners: &[Vertex], out: &mut [Vertex]) -> Result<(), QueryError> {
    let mut longitude = corners.first().map_or(0.0, |corner| corner.longitude);
    let mut previous = longitude;
    for (corner, slot) in corners.iter().zip(out.iter_mut()) {
        longitude += short_way(previous, corner.longitude);
        previous = corner.longitude;
        *slot = Vertex {
            latitude: corner.latitude,
            longitude,
        };
    }
    let first = corners.first().map_or(0.0, |corner| corner.longitude);
    // Back at the first corner the continued longitude is the start again, or a whole turn away.
    if (longitude + short_way(previous, first) - first).abs() > 180.0 {
        return Err(QueryError::EnclosesPole);
    }
    Ok(())
}

/// Twice the signed area by the shoelace formula, in square degrees.
fn twice_area(corners: &[Vertex]) -> f64 {
    let mut sum = 0.0;
    for (at, corner) in corners.iter().enumerate() {
        let next = corners.get((at + 1) % corners.len()).unwrap_or(corner);
        sum += corner.longitude * next.latitude - next.longitude * corner.latitude;
    }
    sum
}

fn extent(corners: &[Vertex], of: impl Fn(&Vertex) -> f64) -> (f64, f64) {
    corners
        .iter()
        .map(of)
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), number| {
            (low.min(number), high.max(number))
        })
}

/// Which side of the line from `a` to `b` the point `c` is: positive left, negative right, zero on it.
fn turn(a: &Vertex, b: &Vertex, c: &Vertex) -> f64 {
    (b.longitude - a.longitude) * (c.latitude - a.latitude)
        - (b.latitude - a.latitude) * (c.longitude - a.longitude)
}

/// Whether `c`, known to be on the line through `a` and `b`, is between them.
fn between(a: &Vertex, b: &Vertex, c: &Vertex) -> bool {
    c.longitude >= a.longitude.min(b.longitude)
        && c.longitude <= a.longitude.max(b.longitude)
        && c.latitude >= a.latitude.min(b.latitude)
        && c.latitude <= a.latitude.max(b.latitude)
}

/// Whether the segments `a`-`b` and `c`-`d` share a point, an end included.
fn segments_meet(a: &Vertex, b: &Vertex, c: &Vertex, d: &Vertex) -> bool {
    let (abc, abd, cda, cdb) = (turn(a, b, c), turn(a, b, d), turn(c, d, a), turn(c, d, b));
    if abc * abd < 0.0 && cda * cdb < 0.0 {
        return true;
    }
    (abc == 0.0 && between(a, b, c))
        || (abd == 0.0 && between(a, b, d))
        || (cda == 0.0 && between(c, d, a))
        || (cdb == 0.0 && between(c, d, b))
}

/// Whether any two edges that are not neighbours meet. Neighbours share a corner by design.
fn has_crossing_edges(corners: &[Vertex]) -> bool {
    let count = corners.len();
    for first in 0..count {
        for second in first + 2..count {
            if first == 0 && second == count - 1 {
                continue;
            }
            let ends = (
                corners.get(first),
                corners.get((first + 1) % count),
                corners.get(second),
                corners.get((second + 1) % count),
            );
            if let (Some(a), Some(b), Some(c), Some(d)) = ends
                && segments_meet(a, b, c, d)
            {
                return true;
            }
        }
    }
    false
}

/// Whether the point is inside the polygon: a ray to the east crosses an odd number of edges. An edge counts
/// when it spans the point's latitude with one end strictly north of it, so a corner on the ray is counted once.
fn contains(corners: &[Vertex], latitude: f64, longitude: f64) -> bool {
    let mut inside = false;
    for (at, a) in corners.iter().enumerate() {
        let Some(b) = corners.get((at + 1) % corners.len()) else {
            continue;
        };
        if (a.latitude > latitude) != (b.latitude > latitude) {
            let crossing = a.longitude
                + (latitude - a.latitude) * (b.longitude - a.longitude) / (b.latitude - a.latitude);
            if longitude < crossing {
                inside = !inside;
            }
        }
    }
    inside
}

#[cfg(test)]
mod tests;
