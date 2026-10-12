//! The exact shape of a generated part, from its `truth.json`.
//!
//! Every generated part is solids of revolution, prisms, or both. A solid of
//! revolution is a profile of segments and arcs in the half-plane through an axis,
//! turned a full turn. The nearest point of such a surface to `p` lies in the
//! half-plane through `p`, so distance to it is distance to the profile in that
//! plane: exact, and cheap. A prism is a flat profile of segments and arcs, an
//! outside and holes, swept straight along an axis. The nearest point of one of its
//! walls lies over the nearest point of the profile, and of a cap straight below or
//! above `p` when `p` lies over the profile: exact again. An index of each profile's
//! pieces keeps both cheap with hundreds of faces or holes.
//!
//! A freeform part is B-spline walls and floors trimmed by levels, and flat caps:
//! see [`freeform`].

mod freeform;

use std::error::Error;
use std::f64::consts::{FRAC_PI_2, PI, TAU};
use std::path::Path;

use glam::{DVec2, DVec3};
use serde::Deserialize;

/// Every solid in a generated part, where the file puts it.
#[derive(Debug, Deserialize)]
pub struct Truth {
    solids: Vec<Solid>,
    #[serde(default)]
    prisms: Vec<Prism>,
    freeform: Option<freeform::Freeform>,
}

/// A solid of revolution as `truth.json` gives it.
#[derive(Deserialize)]
struct SolidFile {
    origin: [f64; 3],
    axis: [f64; 3],
    profile: Vec<Element>,
}

/// A solid of revolution, ready to measure: the pieces of its profile off the axis,
/// which are its surfaces, indexed.
#[derive(Debug, Deserialize)]
#[serde(from = "SolidFile")]
struct Solid {
    origin: DVec3,
    axis: DVec3,
    pieces: Vec<Element>,
    index: Index,
}

/// A piece of profile, millimetres: in `[z, r]` for a solid of revolution, along
/// the axis and out from it; in `[x, y]` for a prism, in the plane of its profile.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Element {
    Segment { from: [f64; 2], to: [f64; 2] },
    /// `center + radius * (cos t, sin t)`, `t` from `start` through `start + sweep`.
    Arc {
        center: [f64; 2],
        radius: f64,
        start: f64,
        sweep: f64,
    },
}

/// A prism as `truth.json` gives it: a flat profile in the plane through `origin`
/// square to `axis`, its `[x, y]` along `x` and `axis × x`, swept along `axis` for
/// `height`. The profile is closed loops of pieces, an outside and holes.
#[derive(Deserialize)]
struct PrismFile {
    origin: [f64; 3],
    axis: [f64; 3],
    x: [f64; 3],
    height: f64,
    loops: Vec<Vec<Element>>,
}

/// A prism, ready to measure: its frame, and every piece of every loop, indexed.
#[derive(Debug, Deserialize)]
#[serde(from = "PrismFile")]
struct Prism {
    origin: DVec3,
    axis: DVec3,
    x: DVec3,
    y: DVec3,
    height: f64,
    pieces: Vec<Element>,
    index: Index,
}

impl Truth {
    /// The truth beside `part` (`<stem>.truth.json`), if it has one.
    pub fn beside(part: &Path) -> Result<Option<Self>, Box<dyn Error>> {
        let path = part.with_extension("truth.json");
        match std::fs::read_to_string(&path) {
            Ok(text) => Ok(Some(serde_json::from_str(&text)?)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// How far `p` is from the nearest surface of the part.
    pub fn distance(&self, p: DVec3) -> f64 {
        let solids = self
            .solids
            .iter()
            .map(|s| s.index.nearest(s.half_plane(p), &s.pieces));
        let prisms = self.prisms.iter().map(|prism| prism.distance(p));
        let freeform = self.freeform.iter().map(|f| f.distance(p));
        solids.chain(prisms).chain(freeform).fold(f64::INFINITY, f64::min)
    }

    /// Points spread over every surface of the part. On a solid of revolution,
    /// `along` steps down each piece of profile, fewer on a profile of more than 16
    /// pieces, and `around` steps round the axis. On a prism, points about a
    /// `around`th of its widest side apart. On a freeform part, every vertex of its
    /// fine mesh, and its caps as prisms.
    pub fn samples(&self, along: usize, around: usize) -> Vec<DVec3> {
        let mut out = Vec::new();
        for s in &self.solids {
            let (o, a) = (s.origin, s.axis);
            let x = a.any_orthonormal_vector();
            let y = a.cross(x);
            let along = (along * 16 / s.pieces.len().max(16)).max(4);
            for e in &s.pieces {
                for i in 0..=along {
                    let zr = e.at(i as f64 / along as f64);
                    let turns = if zr.y < 1e-12 { 1 } else { around };
                    for j in 0..turns {
                        let t = TAU * j as f64 / turns as f64;
                        out.push(o + zr.x * a + zr.y * (t.cos() * x + t.sin() * y));
                    }
                }
            }
        }
        for prism in &self.prisms {
            prism.samples(around, &mut out);
        }
        if let Some(f) = &self.freeform {
            f.samples(around, &mut out);
        }
        out
    }
}

impl From<SolidFile> for Solid {
    fn from(f: SolidFile) -> Self {
        let pieces: Vec<Element> = f.profile.into_iter().filter(|e| !e.on_axis()).collect();
        Solid {
            origin: DVec3::from(f.origin),
            axis: DVec3::from(f.axis).normalize(),
            index: Index::new(&pieces),
            pieces,
        }
    }
}

impl Solid {
    /// `p` as `(z, r)` about this solid's axis.
    fn half_plane(&self, p: DVec3) -> DVec2 {
        let d = p - self.origin;
        let z = d.dot(self.axis);
        DVec2::new(z, (d - z * self.axis).length())
    }
}

impl From<PrismFile> for Prism {
    fn from(f: PrismFile) -> Self {
        let axis = DVec3::from(f.axis).normalize();
        let x = DVec3::from(f.x).normalize();
        let pieces: Vec<Element> = f.loops.into_iter().flatten().collect();
        Prism {
            origin: DVec3::from(f.origin),
            axis,
            x,
            y: axis.cross(x),
            height: f.height,
            index: Index::new(&pieces),
            pieces,
        }
    }
}

impl Prism {
    /// `p` as where it lies over the profile's plane, and how far along the axis.
    fn local(&self, p: DVec3) -> (DVec2, f64) {
        let d = p - self.origin;
        (DVec2::new(d.dot(self.x), d.dot(self.y)), d.dot(self.axis))
    }

    fn world(&self, q: DVec2, w: f64) -> DVec3 {
        self.origin + q.x * self.x + q.y * self.y + w * self.axis
    }

    /// How far `p` is from the prism's surface: from the nearest wall, or, over the
    /// profile, from the nearer cap.
    fn distance(&self, p: DVec3) -> f64 {
        let (q, w) = self.local(p);
        let wall = self
            .index
            .nearest(q, &self.pieces)
            .hypot(w - w.clamp(0.0, self.height));
        if self.index.inside(q, &self.pieces) {
            wall.min(w.abs()).min((w - self.height).abs())
        } else {
            wall
        }
    }

    /// Points about an `around`th of the prism's widest side apart: up every wall, at
    /// least eight along each piece, and over both caps.
    fn samples(&self, around: usize, out: &mut Vec<DVec3>) {
        let (lo, hi) = (self.index.min, self.index.max);
        let spacing = (hi - lo).max_element().max(self.height) / around as f64;
        let up = ((self.height / spacing).ceil() as usize).max(2);
        for piece in &self.pieces {
            let along = ((piece.length() / spacing).ceil() as usize).max(8);
            for i in 0..=along {
                let q = piece.at(i as f64 / along as f64);
                for j in 0..=up {
                    out.push(self.world(q, self.height * j as f64 / up as f64));
                }
            }
        }
        let n = ((hi - lo) / spacing).ceil();
        for i in 0..n.x as usize {
            for j in 0..n.y as usize {
                let q = lo + spacing * DVec2::new(i as f64 + 0.5, j as f64 + 0.5);
                if self.index.inside(q, &self.pieces) {
                    out.push(self.world(q, 0.0));
                    out.push(self.world(q, self.height));
                }
            }
        }
    }
}

/// The pieces of a profile by where they lie, so the nearest to a point, and those
/// a ray from it crosses, are found without trying them all.
#[derive(Debug)]
struct Index {
    /// The corners of a box holding every piece.
    min: DVec2,
    max: DVec2,
    /// A square grid over the box, `nx` cells across and `ny` up.
    cell: f64,
    nx: usize,
    ny: usize,
    /// For each cell, row by row, the pieces whose bounding box meets it.
    cells: Vec<Vec<usize>>,
    /// For each row, the pieces whose bounding box meets it.
    rows: Vec<Vec<usize>>,
}

impl Index {
    fn new(pieces: &[Element]) -> Self {
        let boxes: Vec<(DVec2, DVec2)> = pieces.iter().map(Element::bounds).collect();
        let min = boxes.iter().map(|b| b.0).fold(DVec2::INFINITY, DVec2::min);
        let max = boxes.iter().map(|b| b.1).fold(DVec2::NEG_INFINITY, DVec2::max);
        let size = (max - min).max(DVec2::splat(1e-9));
        // About a piece to a cell, and at most 1024 cells a side.
        let cell = (size.x * size.y / pieces.len().max(1) as f64)
            .sqrt()
            .max(size.max_element() / 1024.0);
        let mut index = Index {
            min,
            max,
            cell,
            nx: ((size.x / cell).ceil() as usize).max(1),
            ny: ((size.y / cell).ceil() as usize).max(1),
            cells: Vec::new(),
            rows: Vec::new(),
        };
        index.cells = vec![Vec::new(); index.nx * index.ny];
        index.rows = vec![Vec::new(); index.ny];
        for (k, (lo, hi)) in boxes.iter().enumerate() {
            let ((i0, j0), (i1, j1)) = (index.cell_of(*lo), index.cell_of(*hi));
            for j in j0..=j1 {
                index.rows[j].push(k);
                for i in i0..=i1 {
                    index.cells[j * index.nx + i].push(k);
                }
            }
        }
        index
    }

    /// The cell holding `q`, or the cell nearest it.
    fn cell_of(&self, q: DVec2) -> (usize, usize) {
        let c = ((q - self.min) / self.cell).floor();
        (
            c.x.clamp(0.0, (self.nx - 1) as f64) as usize,
            c.y.clamp(0.0, (self.ny - 1) as f64) as usize,
        )
    }

    /// How far `q` is from the nearest piece. The cells are searched in rings out
    /// from the one nearest `q`. Every cell outside ring `k` is more than `k` cells'
    /// width from `q`, so once the nearest piece found is no farther than that,
    /// nothing outside can be nearer.
    fn nearest(&self, q: DVec2, pieces: &[Element]) -> f64 {
        let (ci, cj) = self.cell_of(q);
        let (ci, cj) = (ci as isize, cj as isize);
        let (nx, ny) = (self.nx as isize, self.ny as isize);
        let mut best = f64::INFINITY;
        for k in 0..=nx.max(ny) {
            for j in (cj - k).max(0)..=(cj + k).min(ny - 1) {
                let edge = (j - cj).abs() == k;
                let mut i = (ci - k).max(0);
                while i <= (ci + k).min(nx - 1) {
                    for &p in &self.cells[(j * nx + i) as usize] {
                        best = best.min(pieces[p].distance(q));
                    }
                    // Inside the ring's top and bottom rows, only its two sides.
                    i = if edge || i == ci + k { i + 1 } else { ci + k };
                }
            }
            if best <= k as f64 * self.cell {
                break;
            }
        }
        best
    }

    /// Whether `q` lies inside the profile: whether a ray from it along +x crosses
    /// the profile's loops an odd number of times.
    fn inside(&self, q: DVec2, pieces: &[Element]) -> bool {
        let row = ((q.y - self.min.y) / self.cell).floor();
        if !(row >= 0.0 && row < self.ny as f64) {
            return false;
        }
        let crossings: usize = self.rows[row as usize].iter().map(|&p| pieces[p].crossings(q)).sum();
        crossings % 2 == 1
    }
}

/// The point at angle `t` on the unit circle, exact at every quarter turn, so arcs
/// that start or end there meet their neighbours, and themselves, exactly.
fn unit(t: f64) -> DVec2 {
    let quarters = t / FRAC_PI_2;
    if (quarters - quarters.round()).abs() < 1e-12 {
        [DVec2::X, DVec2::Y, DVec2::NEG_X, DVec2::NEG_Y][quarters.round().rem_euclid(4.0) as usize]
    } else {
        DVec2::from_angle(t)
    }
}

impl Element {
    fn on_axis(&self) -> bool {
        matches!(self, Element::Segment { from, to } if from[1].abs() < 1e-12 && to[1].abs() < 1e-12)
    }

    fn at(&self, t: f64) -> DVec2 {
        match *self {
            Element::Segment { from, to } => DVec2::from(from).lerp(DVec2::from(to), t),
            Element::Arc {
                center,
                radius,
                start,
                sweep,
            } => DVec2::from(center) + radius * unit(start + sweep * t),
        }
    }

    fn length(&self) -> f64 {
        match *self {
            Element::Segment { from, to } => DVec2::from(from).distance(DVec2::from(to)),
            Element::Arc { radius, sweep, .. } => radius * sweep.abs(),
        }
    }

    /// A box holding the piece: an arc's whole circle's, which is enough for an
    /// index.
    fn bounds(&self) -> (DVec2, DVec2) {
        match *self {
            Element::Segment { from, to } => {
                let (a, b) = (DVec2::from(from), DVec2::from(to));
                (a.min(b), a.max(b))
            }
            Element::Arc { center, radius, .. } => {
                let c = DVec2::from(center);
                (c - radius, c + radius)
            }
        }
    }

    /// How many times the piece crosses the ray from `q` along +x. A crossing counts
    /// where one end of the crossing piece lies above `q` and the other does not, so
    /// a ray through the joint of two pieces is counted once, or not at all, as it
    /// should be.
    fn crossings(&self, q: DVec2) -> usize {
        match *self {
            Element::Segment { from, to } => {
                let (a, b) = (DVec2::from(from), DVec2::from(to));
                if (a.y > q.y) == (b.y > q.y) {
                    return 0;
                }
                let x = a.x + (q.y - a.y) / (b.y - a.y) * (b.x - a.x);
                usize::from(x > q.x)
            }
            Element::Arc {
                center,
                radius,
                start,
                sweep,
            } => {
                // Cut where the circle turns, at its top and bottom, into pieces that
                // each cross a level once at most, on one side of the centre. A turn
                // or less has at most two such cuts.
                let (lo, hi) = (start.min(start + sweep), start.max(start + sweep));
                let (mut cuts, mut n) = ([lo; 5], 1);
                let mut t = ((lo - FRAC_PI_2) / PI).floor() * PI + FRAC_PI_2;
                while t < hi && n < 4 {
                    if t > lo {
                        cuts[n] = t;
                        n += 1;
                    }
                    t += PI;
                }
                cuts[n] = hi;
                let c = DVec2::from(center);
                cuts[..=n]
                    .windows(2)
                    .filter(|w| {
                        let (a, b) = (c + radius * unit(w[0]), c + radius * unit(w[1]));
                        if (a.y > q.y) == (b.y > q.y) {
                            return false;
                        }
                        let side = ((w[0] + w[1]) / 2.0).cos().signum();
                        let x = c.x + side * (radius * radius - (q.y - c.y).powi(2)).max(0.0).sqrt();
                        x > q.x
                    })
                    .count()
            }
        }
    }

    fn distance(&self, q: DVec2) -> f64 {
        match *self {
            Element::Segment { from, to } => {
                let (a, b) = (DVec2::from(from), DVec2::from(to));
                let ab = b - a;
                let t = ((q - a).dot(ab) / ab.length_squared()).clamp(0.0, 1.0);
                (a + t * ab).distance(q)
            }
            Element::Arc {
                center,
                radius,
                start,
                sweep,
            } => {
                let c = DVec2::from(center);
                let d = q - c;
                let angle = d.y.atan2(d.x);
                // How far round from the start, the way the arc runs.
                let gone = if sweep >= 0.0 {
                    (angle - start).rem_euclid(TAU)
                } else {
                    (start - angle).rem_euclid(TAU)
                };
                if gone <= sweep.abs() {
                    (d.length() - radius).abs()
                } else {
                    self.at(0.0).distance(q).min(self.at(1.0).distance(q))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A plate 20 mm square and 2 thick, its corners rounded 2 mm, with a hole 6 mm
    /// across in the middle.
    fn plate() -> Truth {
        serde_json::from_str(
            r#"{"solids": [], "prisms": [{"origin": [0, 0, 0], "axis": [0, 0, 1], "x": [1, 0, 0], "height": 2,
            "loops": [[
                {"segment": {"from": [2, 0], "to": [18, 0]}},
                {"arc": {"center": [18, 2], "radius": 2, "start": -1.5707963267948966, "sweep": 1.5707963267948966}},
                {"segment": {"from": [20, 2], "to": [20, 18]}},
                {"arc": {"center": [18, 18], "radius": 2, "start": 0, "sweep": 1.5707963267948966}},
                {"segment": {"from": [18, 20], "to": [2, 20]}},
                {"arc": {"center": [2, 18], "radius": 2, "start": 1.5707963267948966, "sweep": 1.5707963267948966}},
                {"segment": {"from": [0, 18], "to": [0, 2]}},
                {"arc": {"center": [2, 2], "radius": 2, "start": 3.141592653589793, "sweep": 1.5707963267948966}}
            ], [
                {"arc": {"center": [10, 10], "radius": 3, "start": 0, "sweep": -6.283185307179586}}
            ]]}]}"#,
        )
        .unwrap()
    }

    #[test]
    fn distance_to_a_plate_with_a_hole() {
        let t = plate();
        let d = |x, y, z| t.distance(DVec3::new(x, y, z));
        // On the top, over the material; above it; below the bottom.
        assert_eq!(d(5.0, 5.0, 2.0), 0.0);
        assert!((d(5.0, 5.0, 3.0) - 1.0).abs() < 1e-12);
        assert!((d(5.0, 5.0, -0.5) - 0.5).abs() < 1e-12);
        // Inside the material, nearer a cap than a wall.
        assert!((d(5.0, 5.0, 0.4) - 0.4).abs() < 1e-12);
        // In the hole, level with the top, and on the seam's level: nearest the wall.
        assert!((d(10.0, 10.0, 2.0) - 3.0).abs() < 1e-12);
        assert!((d(11.0, 10.0, 1.0) - 2.0).abs() < 1e-12);
        assert!((d(7.5, 10.0, 0.0) - 0.5).abs() < 1e-12);
        // Above the hole: to its rim.
        assert!((d(10.0, 10.0, 6.0) - 5.0).abs() < 1e-12);
        // Off a rounded corner, and off a side.
        let corner = DVec3::new(-1.0, -1.0, 1.0);
        assert!((t.distance(corner) - (DVec2::splat(3.0).length() - 2.0)).abs() < 1e-12);
        assert!((d(25.0, 10.0, 1.0) - 5.0).abs() < 1e-12);
        // Far off, past the index.
        assert!((d(1000.0, 10.0, 1.0) - 980.0).abs() < 1e-9);
    }

    #[test]
    fn samples_lie_on_the_plate() {
        let t = plate();
        let samples = t.samples(64, 64);
        assert!(samples.len() > 1000);
        for p in samples {
            assert!(t.distance(p) < 1e-9, "{p} is {} off", t.distance(p));
        }
    }

    #[test]
    fn inside_through_joints_and_the_seam() {
        let t = plate();
        let prism = &t.prisms[0];
        let inside = |x, y| prism.index.inside(DVec2::new(x, y), &prism.pieces);
        // Level with the hole's centre and its seam, either side of it and in it.
        assert!(inside(1.0, 10.0));
        assert!(!inside(10.0, 10.0));
        assert!(inside(15.0, 10.0));
        // Level with the corner arcs' joints, and outside.
        assert!(inside(0.5, 2.0));
        assert!(inside(19.5, 18.0));
        assert!(!inside(-0.5, 2.0));
        assert!(!inside(0.1, 0.1));
        assert!(!inside(25.0, 10.0));
    }
}
