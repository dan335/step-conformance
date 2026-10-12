//! The exact shape of a generated part, from its `truth.json`.
//!
//! Every generated part is solids of revolution: a profile of segments and arcs in
//! the half-plane through an axis, turned a full turn. The nearest point of such a
//! surface to `p` lies in the half-plane through `p`, so distance to it is distance
//! to the profile in that plane: exact, and cheap.

use std::error::Error;
use std::f64::consts::TAU;
use std::path::Path;

use glam::{DVec2, DVec3};
use serde::Deserialize;

/// Every solid in a generated part, where the file puts it.
#[derive(Debug, Deserialize)]
pub struct Truth {
    solids: Vec<Solid>,
}

#[derive(Debug, Deserialize)]
struct Solid {
    origin: [f64; 3],
    axis: [f64; 3],
    profile: Vec<Element>,
}

/// A piece of profile, in `[z, r]`: along the axis and out from it, millimetres.
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
        self.solids
            .iter()
            .flat_map(|s| {
                let q = s.half_plane(p);
                s.profile.iter().filter(|e| !e.on_axis()).map(move |e| e.distance(q))
            })
            .fold(f64::INFINITY, f64::min)
    }

    /// Points spread over every surface of the part: `along` steps down each piece
    /// of profile, `around` steps round the axis.
    pub fn samples(&self, along: usize, around: usize) -> Vec<DVec3> {
        let mut out = Vec::new();
        for s in &self.solids {
            let (o, a) = (DVec3::from(s.origin), DVec3::from(s.axis).normalize());
            let x = a.any_orthonormal_vector();
            let y = a.cross(x);
            for e in s.profile.iter().filter(|e| !e.on_axis()) {
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
        out
    }
}

impl Solid {
    /// `p` as `(z, r)` about this solid's axis.
    fn half_plane(&self, p: DVec3) -> DVec2 {
        let a = DVec3::from(self.axis).normalize();
        let d = p - DVec3::from(self.origin);
        let z = d.dot(a);
        DVec2::new(z, (d - z * a).length())
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
            } => DVec2::from(center) + radius * DVec2::from_angle(start + sweep * t),
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
