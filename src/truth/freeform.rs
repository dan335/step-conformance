//! The exact shape of a freeform part: B-spline walls, each trimmed between two
//! levels, a B-spline floor within a wall's rim, and flat caps within the walls'
//! rims.
//!
//! A level is a plane `z = c`, or a floor: a B-spline surface that is a height over
//! `x, y`. Where a wall crosses a level is found exactly, by bisection, on each
//! column of the wall, so the faces end where the part does. The walls and floors
//! are then meshed finely, rows and columns split until each lies within [`SAG`] of
//! the chords between its neighbours, and distance to them is distance to that
//! mesh. Its triangles stand off their surfaces by about [`SAG`], a little more
//! across a cell's diagonal: the tests hold it to twice that.
//! A cap is flat, so it is measured exactly, as a prism of no height whose profile
//! is the rims of the walls that bound it.

use glam::{DVec2, DVec3};
use parry3d_f64::query::PointQuery;
use parry3d_f64::shape::TriMesh;
use serde::Deserialize;

use super::{Element, Index, Prism};

/// How far a triangle of the mesh may stand off its surface, in millimetres.
const SAG: f64 = 0.0005;

/// The longest a triangle's side may be, so that the mesh's vertices, which lie on
/// the surface, are samples close enough to find missing surface between them.
const LONGEST: f64 = 1.0;

/// A freeform part as `truth.json` gives it.
#[derive(Deserialize)]
pub(super) struct FreeformFile {
    surfaces: Vec<Surface>,
    #[serde(default)]
    graphs: Vec<Graph>,
    bands: Vec<Band>,
    #[serde(default)]
    floors: Vec<Floor>,
    #[serde(default)]
    caps: Vec<Cap>,
}

/// A freeform part, ready to measure: its walls and floors as one fine mesh, and its
/// caps.
#[derive(Debug, Deserialize)]
#[serde(try_from = "FreeformFile")]
pub(super) struct Freeform {
    mesh: TriMesh,
    caps: Vec<Prism>,
}

/// A tensor-product B-spline surface, millimetres: `points[i][j]` with `i` along u.
#[derive(Deserialize)]
struct Surface {
    degree: [usize; 2],
    u_knots: Vec<f64>,
    v_knots: Vec<f64>,
    points: Vec<Vec<[f64; 3]>>,
}

/// A surface that is a height over `x, y`: its u runs with `x` across `x`, and its v
/// with `y` across `y`.
#[derive(Deserialize)]
struct Graph {
    surface: usize,
    x: [f64; 2],
    y: [f64; 2],
}

/// Where a wall is trimmed.
#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Level {
    /// The plane `z = c`.
    Plane(f64),
    /// A graph's surface.
    Graph(usize),
}

/// A wall: a surface closed in u, between where it crosses `from` and `to` going up
/// in v.
#[derive(Deserialize)]
struct Band {
    surface: usize,
    from: Level,
    to: Level,
}

/// A graph's surface within the rim where a band crosses its `from` level, which
/// must be seen whole from `center`.
#[derive(Deserialize)]
struct Floor {
    graph: usize,
    within: usize,
    center: [f64; 2],
}

/// The plane `z` within the rims of bands, at their `from` or `to` level.
#[derive(Deserialize)]
struct Cap {
    z: f64,
    rims: Vec<Rim>,
}

#[derive(Deserialize)]
struct Rim {
    band: usize,
    end: End,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum End {
    From,
    To,
}

impl Freeform {
    /// How far `p` is from the part's walls, floors and caps.
    pub(super) fn distance(&self, p: DVec3) -> f64 {
        let mesh = (self.mesh.project_local_point(p, false).point - p).length();
        self.caps.iter().map(|cap| cap.distance(p)).fold(mesh, f64::min)
    }

    /// Points on every surface: the mesh's vertices, which lie on its walls and
    /// floors, and points over its caps about an `around`th of their widest side
    /// apart.
    pub(super) fn samples(&self, around: usize, out: &mut Vec<DVec3>) {
        out.extend(self.mesh.vertices());
        for cap in &self.caps {
            cap.samples(around, out);
        }
    }
}

impl TryFrom<FreeformFile> for Freeform {
    type Error = String;

    fn try_from(f: FreeformFile) -> Result<Self, String> {
        if let Some(s) = f.surfaces.iter().find(|s| s.degree.iter().any(|&d| d == 0 || d > 3)) {
            return Err(format!("a surface of degree {:?}: only 1 to 3 are measured", s.degree));
        }
        let mut grids = Vec::new();
        // Each band's columns, kept for the floors and caps its rims bound.
        let mut walls = Vec::new();
        for band in &f.bands {
            let column = |u: f64, rows: usize| f.column(band, u, rows);
            let (u0, u1) = f.surfaces[band.surface].u_range();
            let rows = rows(&column, u0, u1);
            let columns = columns(&|u| column(u, rows), u0, u1);
            grids.push(columns.clone());
            walls.push(columns);
        }
        for floor in &f.floors {
            let (rim, graph) = (&walls[floor.within], &f.graphs[floor.graph]);
            let center = DVec2::from(floor.center);
            check_seen_whole(rim, center)?;
            let band = &f.bands[floor.within];
            let (u0, u1) = f.surfaces[band.surface].u_range();
            let column = |u: f64, rows: usize| {
                let edge = f.column(band, u, 1)[0].truncate();
                (0..=rows)
                    .map(|j| {
                        let q = center.lerp(edge, j as f64 / rows as f64);
                        q.extend(f.height(graph, q))
                    })
                    .collect()
            };
            let rows = rows(&column, u0, u1);
            grids.push(columns(&|u| column(u, rows), u0, u1));
        }
        let caps = f
            .caps
            .iter()
            .map(|cap| {
                let pieces = cap
                    .rims
                    .iter()
                    .flat_map(|rim| {
                        let ring: Vec<DVec2> = walls[rim.band]
                            .iter()
                            .map(|c| match rim.end {
                                End::From => c[0],
                                End::To => c[c.len() - 1],
                            })
                            .map(|p| p.truncate())
                            .collect();
                        ring.windows(2)
                            .filter(|w| w[0] != w[1])
                            .map(|w| Element::Segment { from: w[0].into(), to: w[1].into() })
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>();
                Prism {
                    origin: DVec3::new(0.0, 0.0, cap.z),
                    axis: DVec3::Z,
                    x: DVec3::X,
                    y: DVec3::Y,
                    height: 0.0,
                    index: Index::new(&pieces),
                    pieces,
                }
            })
            .collect();
        let (mut vertices, mut triangles) = (Vec::new(), Vec::new());
        for grid in &grids {
            mesh_grid(grid, &mut vertices, &mut triangles);
        }
        let mesh = TriMesh::new(vertices, triangles).map_err(|e| format!("the freeform mesh: {e}"))?;
        Ok(Freeform { mesh, caps })
    }
}

impl FreeformFile {
    /// `rows + 1` points up `band` at `u`, evenly in v from where it crosses its
    /// `from` level to where it crosses its `to`.
    fn column(&self, band: &Band, u: f64, rows: usize) -> Vec<DVec3> {
        let s = &self.surfaces[band.surface];
        let (from, to) = (self.crossing(s, u, band.from), self.crossing(s, u, band.to));
        (0..=rows).map(|j| s.at(u, from + (to - from) * j as f64 / rows as f64)).collect()
    }

    /// The v at which `s` crosses `level` at `u`, by bisection: the surface must lie
    /// below the level at the start of v and above it at the end.
    fn crossing(&self, s: &Surface, u: f64, level: Level) -> f64 {
        let above = |v: f64| {
            let p = s.at(u, v);
            p.z - self.level(level, p.truncate())
        };
        let (mut lo, mut hi) = s.v_range();
        let below = above(lo) < 0.0;
        for _ in 0..64 {
            let mid = 0.5 * (lo + hi);
            if (above(mid) < 0.0) == below {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        0.5 * (lo + hi)
    }

    fn level(&self, level: Level, q: DVec2) -> f64 {
        match level {
            Level::Plane(z) => z,
            Level::Graph(g) => self.height(&self.graphs[g], q),
        }
    }

    /// The height of `graph` over `q`, held to its edge outside it.
    fn height(&self, graph: &Graph, q: DVec2) -> f64 {
        let s = &self.surfaces[graph.surface];
        let along = |x: f64, [a, b]: [f64; 2], (lo, hi): (f64, f64)| lo + (hi - lo) * ((x - a) / (b - a)).clamp(0.0, 1.0);
        s.at(along(q.x, graph.x, s.u_range()), along(q.y, graph.y, s.v_range())).z
    }
}

impl Surface {
    fn at(&self, u: f64, v: f64) -> DVec3 {
        let (i, nu) = basis(&self.u_knots, self.degree[0], u);
        let (j, nv) = basis(&self.v_knots, self.degree[1], v);
        let mut p = DVec3::ZERO;
        for (a, wu) in nu.iter().enumerate().take(self.degree[0] + 1) {
            for (b, wv) in nv.iter().enumerate().take(self.degree[1] + 1) {
                p += wu * wv * DVec3::from(self.points[i - self.degree[0] + a][j - self.degree[1] + b]);
            }
        }
        p
    }

    fn u_range(&self) -> (f64, f64) {
        (self.u_knots[0], self.u_knots[self.u_knots.len() - 1])
    }

    fn v_range(&self) -> (f64, f64) {
        (self.v_knots[0], self.v_knots[self.v_knots.len() - 1])
    }
}

/// The knot span holding `u`, and the `p + 1` basis functions that are not zero on
/// it (The NURBS Book, A2.2).
fn basis(knots: &[f64], p: usize, u: f64) -> (usize, [f64; 4]) {
    let n = knots.len() - p - 1;
    let u = u.clamp(knots[p], knots[n]);
    // The last span holding u, and the last that is not empty at the end.
    let k = (knots.partition_point(|&k| k <= u) - 1).clamp(p, n - 1);
    let k = (p..=k).rev().find(|&k| knots[k] < knots[k + 1]).unwrap_or(p);
    let (mut out, mut left, mut right) = ([0.0; 4], [0.0; 4], [0.0; 4]);
    out[0] = 1.0;
    for j in 1..=p {
        left[j] = u - knots[k + 1 - j];
        right[j] = knots[k + j] - u;
        let mut saved = 0.0;
        for r in 0..j {
            let t = out[r] / (right[r + 1] + left[j - r]);
            out[r] = saved + right[r + 1] * t;
            saved = left[j - r] * t;
        }
        out[j] = saved;
    }
    (k, out)
}

/// Whether the curve through `a` and `b` strays from the chord between them, or the
/// chord is too long: `mid` is the curve's point halfway between them in parameter.
fn too_coarse(a: DVec3, mid: DVec3, b: DVec3) -> bool {
    (mid - (a + b) / 2.0).length() > SAG || a.distance(b) > LONGEST
}

/// How many rows each column needs, doubling until no row's midpoint strays from
/// the chord between its neighbours, on columns spread across `u0..u1`.
fn rows(column: &dyn Fn(f64, usize) -> Vec<DVec3>, u0: f64, u1: f64) -> usize {
    let mut rows = 4;
    while rows < 1 << 14 {
        let fine = (0..256).any(|i| {
            let c = column(u0 + (u1 - u0) * (i as f64 + 0.5) / 256.0, rows * 2);
            c.windows(3).step_by(2).any(|w| too_coarse(w[0], w[1], w[2]))
        });
        if !fine {
            return rows;
        }
        rows *= 2;
    }
    rows
}

/// Columns across `u0..u1`, each split in two until the column halfway between
/// neighbours lies on the chords between them.
fn columns(column: &dyn Fn(f64) -> Vec<DVec3>, u0: f64, u1: f64) -> Vec<Vec<DVec3>> {
    fn between(
        column: &dyn Fn(f64) -> Vec<DVec3>,
        (ua, a): (f64, &[DVec3]),
        (ub, b): (f64, &[DVec3]),
        depth: usize,
        out: &mut Vec<Vec<DVec3>>,
    ) {
        let um = 0.5 * (ua + ub);
        let mid = column(um);
        if depth < 30 && a.iter().zip(&mid).zip(b).any(|((&a, &m), &b)| too_coarse(a, m, b)) {
            between(column, (ua, a), (um, &mid), depth + 1, out);
            out.push(mid.clone());
            between(column, (um, &mid), (ub, b), depth + 1, out);
        }
    }
    const START: usize = 256;
    let mut out = vec![column(u0)];
    for i in 1..=START {
        let u = u0 + (u1 - u0) * i as f64 / START as f64;
        let next = column(u);
        let ua = u0 + (u1 - u0) * (i - 1) as f64 / START as f64;
        let last = out[out.len() - 1].clone();
        between(column, (ua, &last), (u, &next), 0, &mut out);
        out.push(next);
    }
    out
}

/// Two triangles to each cell between neighbouring columns and rows, leaving out
/// those with no area, as where every column starts at one centre.
fn mesh_grid(grid: &[Vec<DVec3>], vertices: &mut Vec<DVec3>, triangles: &mut Vec<[u32; 3]>) {
    let rows = grid[0].len();
    let first = vertices.len() as u32;
    vertices.extend(grid.iter().flatten());
    let at = |i: usize, j: usize| first + (i * rows + j) as u32;
    for i in 0..grid.len() - 1 {
        for j in 0..rows - 1 {
            for t in [[at(i, j), at(i + 1, j), at(i + 1, j + 1)], [at(i, j), at(i + 1, j + 1), at(i, j + 1)]] {
                let [a, b, c] = t.map(|k| vertices[k as usize]);
                if (b - a).cross(c - a).length_squared() > 0.0 {
                    triangles.push(t);
                }
            }
        }
    }
}

/// A floor is meshed along rays from its centre to its rim, which cross only if
/// the rim turns back on itself as seen from there.
fn check_seen_whole(rim: &[Vec<DVec3>], center: DVec2) -> Result<(), String> {
    let mut turned = 0.0;
    for w in rim.windows(2) {
        let (a, b) = (w[0][0].truncate() - center, w[1][0].truncate() - center);
        let step = a.perp_dot(b).atan2(a.dot(b));
        if step * turned < 0.0 {
            return Err(format!("a floor's rim turns back as seen from {center}"));
        }
        turned += step;
    }
    if (turned.abs() - std::f64::consts::TAU).abs() > 1e-6 {
        return Err(format!("a floor's rim goes {turned} radians round {center}, not once"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn panel() -> (FreeformFile, Freeform) {
        let text = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/corpus/freeform_panel.truth.json")).unwrap();
        let json: serde_json::Value = serde_json::from_str(&text).unwrap();
        let file = serde_json::from_value(json["freeform"].clone()).unwrap();
        (file, serde_json::from_value(json["freeform"].clone()).unwrap())
    }

    /// Points spread over a band, exactly on its surface, between its levels.
    fn on_band(f: &FreeformFile, band: &Band, n: usize) -> Vec<DVec3> {
        let (u0, u1) = f.surfaces[band.surface].u_range();
        (0..n)
            .flat_map(|i| {
                // Off the columns' grid, so the points fall between vertices.
                let u = u0 + (u1 - u0) * (i as f64 + 0.37) / n as f64;
                f.column(band, u, 7).into_iter().skip(1).take(6)
            })
            .collect()
    }

    #[test]
    fn the_mesh_lies_on_the_panel() {
        let (f, shape) = panel();
        let mut worst: f64 = 0.0;
        for band in &f.bands {
            for p in on_band(&f, band, 4000) {
                worst = worst.max(shape.distance(p));
            }
        }
        let floor = &f.floors[0];
        let graph = &f.graphs[floor.graph];
        let center = DVec2::from(floor.center);
        for p in on_band(&f, &f.bands[floor.within], 2000) {
            let q = center.lerp(p.truncate(), 0.613);
            worst = worst.max(shape.distance(q.extend(f.height(graph, q))));
        }
        assert!(worst <= 2.0 * SAG, "a point of the panel lies {worst} mm from its mesh");
    }

    #[test]
    fn distances_off_the_panel() {
        let (_, shape) = panel();
        // Under the middle of the bottom, and over the top beside the pocket.
        assert!((shape.distance(DVec3::new(0.0, -100.0, -2.0)) - 2.0).abs() < 1e-9);
        assert!((shape.distance(DVec3::new(-130.0, 0.0, 47.0)) - 3.0).abs() < 1e-9);
        // Beside a concave bend of the bottom rim, outside the part, where OpenCascade's
        // mesh at 0.001 mm reaches: 0.608 mm from the rim, a little less from the wall
        // just above it.
        let off = shape.distance(DVec3::new(-120.4652, -100.5746, 0.0));
        assert!(off > 0.6 && off < 0.608, "{off}");
    }
}
