//! Solids of revolution, a closed profile of segments and arcs turned a full turn
//! about an axis, and prisms, a flat profile with holes swept straight along one.
//! The exact shape a generated part stands for, and what its `truth.json` records.

use glam::{DVec2, DVec3};
use serde::Serialize;

/// One piece of a profile, in millimetres. For a solid of revolution it lies in the
/// half-plane through the axis, `[z, r]`: how far along the axis and how far out from
/// it. For a prism it lies in the prism's plane, `[x, y]`.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Element {
    /// A straight piece.
    Segment { from: [f64; 2], to: [f64; 2] },
    /// A circular arc: `center + radius * (cos t, sin t)` for `t` from `start`
    /// through `start + sweep`, radians, in `[z, r]`.
    Arc {
        center: [f64; 2],
        radius: f64,
        start: f64,
        sweep: f64,
    },
}

impl Element {
    /// The point a fraction `t` of the way along, as `(z, r)`.
    pub fn at(&self, t: f64) -> DVec2 {
        match *self {
            Element::Segment { from, to } => DVec2::from(from).lerp(DVec2::from(to), t),
            Element::Arc {
                center,
                radius,
                start,
                sweep,
            } => {
                let a = start + sweep * t;
                DVec2::from(center) + radius * DVec2::new(a.cos(), a.sin())
            }
        }
    }

    /// The direction of travel at `t`, as `(dz, dr)`.
    pub fn tangent(&self, t: f64) -> DVec2 {
        match *self {
            Element::Segment { from, to } => DVec2::from(to) - DVec2::from(from),
            Element::Arc { start, sweep, .. } => {
                let a = start + sweep * t;
                sweep * DVec2::new(-a.sin(), a.cos())
            }
        }
    }

    /// Whether this piece lies on the axis, so is no surface at all.
    pub fn on_axis(&self) -> bool {
        matches!(self, Element::Segment { from, to } if from[1].abs() < 1e-12 && to[1].abs() < 1e-12)
    }

    /// The outward normal at `t`, as `(dz, dr)`, for a profile that runs
    /// counterclockwise seen with r across and z up.
    pub fn outward(&self, t: f64) -> DVec2 {
        let d = self.tangent(t);
        // (dr, dz) turned a quarter clockwise is (dz, -dr); back in (z, r) order.
        DVec2::new(-d.y, d.x).normalize()
    }
}

/// A solid of revolution about `axis` through `origin`, in the frame its profile is
/// measured in. `x` fixes where a turn starts: where seams and edge vertices sit.
#[derive(Debug, Clone, Serialize)]
pub struct Solid {
    pub origin: [f64; 3],
    pub axis: [f64; 3],
    #[serde(skip)]
    pub x: DVec3,
    pub profile: Vec<Element>,
}

impl Solid {
    pub fn new(profile: Vec<Element>) -> Self {
        Self {
            origin: [0.0; 3],
            axis: [0.0, 0.0, 1.0],
            x: DVec3::X,
            profile,
        }
    }

    /// The same solid with its axis frame moved: `origin`, `axis` and `x` in
    /// millimetres, `x` perpendicular to `axis`.
    pub fn placed(&self, origin: DVec3, axis: DVec3, x: DVec3) -> Self {
        Self {
            origin: origin.to_array(),
            axis: axis.normalize().to_array(),
            x: x.normalize(),
            profile: self.profile.clone(),
        }
    }
}

/// A prism: a flat profile swept along `axis` for `height`. The profile lies in the
/// plane through `origin` square to `axis`, its `[x, y]` along `x` and `axis × x`.
/// It is closed loops of pieces: the outside first, running counterclockwise seen
/// from the far end of the axis, then the holes, running clockwise, so that the
/// material is always on a loop's left.
#[derive(Debug, Clone, Serialize)]
pub struct Prism {
    pub origin: [f64; 3],
    pub axis: [f64; 3],
    pub x: [f64; 3],
    pub height: f64,
    pub loops: Vec<Vec<Element>>,
}

/// What a generated part is, exactly: every solid in it, where the file puts it.
#[derive(Debug, Default, Serialize)]
pub struct Truth {
    pub solids: Vec<Solid>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub prisms: Vec<Prism>,
}

/// A straight piece from `(z, r)` to `(z, r)`.
pub fn seg(from: [f64; 2], to: [f64; 2]) -> Element {
    Element::Segment { from, to }
}

/// A round hole in a prism's profile: a full circle about `[x, y]`, clockwise.
pub fn hole(center: [f64; 2], radius: f64) -> Vec<Element> {
    vec![arc(center, radius, 0.0, -360.0)]
}

/// An arc about `(z, r)`, from angle `start` through `sweep`, in degrees.
pub fn arc(center: [f64; 2], radius: f64, start: f64, sweep: f64) -> Element {
    Element::Arc {
        center,
        radius,
        start: start.to_radians(),
        sweep: sweep.to_radians(),
    }
}
