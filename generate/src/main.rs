//! Write `corpus/generated/`: parts whose exact shape is known.
//!
//! ```text
//! cargo run --manifest-path generate/Cargo.toml -- corpus/generated
//! ```
//!
//! Every part is solids of revolution or prisms, written as STEP in the dialect the
//! test needs, next to a `.truth.json` saying exactly what it is. The runner
//! measures a reader's mesh against that instead of against the reference mesh:
//! distance to a surface of revolution is exact and cheap, worked in the plane
//! through the axis and the point, and so is distance to a prism, worked in the
//! plane of its profile.
//!
//! Each part targets one way to fail, or is big, to time readers on; each is named
//! in its comment here and in the README. The output is the same every run.

mod shape;
mod step;

use std::path::Path;

use glam::DVec3;

use shape::{Prism, Solid, Truth, arc, hole, seg};
use step::{Step, Style, Unit};

/// A cylinder, 20 mm across and 30 tall, its base on the origin.
fn cylinder() -> Solid {
    Solid::new(vec![
        seg([0.0, 0.0], [0.0, 10.0]),
        seg([0.0, 10.0], [30.0, 10.0]),
        seg([30.0, 10.0], [30.0, 0.0]),
        seg([30.0, 0.0], [0.0, 0.0]),
    ])
}

/// A plate `n * 10` mm square and 10 thick, its corners rounded, with `n` by `n`
/// holes 6 mm across and 10 mm apart.
fn plate(n: usize) -> Prism {
    let (w, r) = (n as f64 * 10.0, 4.0);
    let outside = vec![
        seg([r, 0.0], [w - r, 0.0]),
        arc([w - r, r], r, -90.0, 90.0),
        seg([w, r], [w, w - r]),
        arc([w - r, w - r], r, 0.0, 90.0),
        seg([w - r, w], [r, w]),
        arc([r, w - r], r, 90.0, 90.0),
        seg([0.0, w - r], [0.0, r]),
        arc([r, r], r, 180.0, 90.0),
    ];
    let mut loops = vec![outside];
    for i in 0..n {
        for j in 0..n {
            loops.push(hole([i as f64 * 10.0 + 5.0, j as f64 * 10.0 + 5.0], 3.0));
        }
    }
    Prism {
        origin: [0.0; 3],
        axis: [0.0, 0.0, 1.0],
        x: [1.0, 0.0, 0.0],
        height: 10.0,
        loops,
    }
}

/// A shaft 20 mm across, ringed by `n` V-grooves half a millimetre deep, a groove in
/// every 2 mm, like a thread.
fn grooved_shaft(n: usize) -> Solid {
    let length = 2.0 * n as f64 + 1.0;
    let mut profile = vec![seg([0.0, 0.0], [0.0, 10.0])];
    for k in 0..n {
        let z = 2.0 * k as f64;
        profile.push(seg([z, 10.0], [z + 1.0, 10.0]));
        profile.push(seg([z + 1.0, 10.0], [z + 1.5, 9.5]));
        profile.push(seg([z + 1.5, 9.5], [z + 2.0, 10.0]));
    }
    profile.extend([
        seg([length - 1.0, 10.0], [length, 10.0]),
        seg([length, 10.0], [length, 0.0]),
        seg([length, 0.0], [0.0, 0.0]),
    ]);
    Solid::new(profile)
}

fn write(dir: &Path, name: &str, step: &Step, truth: &Truth) -> std::io::Result<()> {
    std::fs::write(dir.join(format!("{name}.step")), step.text(name))?;
    let truth = serde_json::to_string_pretty(truth).map_err(std::io::Error::other)?;
    std::fs::write(dir.join(format!("{name}.truth.json")), truth + "\n")
}

/// One part of one solid of revolution, unplaced.
fn part(dir: &Path, name: &str, unit: Unit, solid: Solid, style: Style) -> std::io::Result<()> {
    let mut s = Step::new(unit);
    s.part(name, unit, &[(solid.clone(), style)], &[]);
    write(dir, name, &s, &Truth { solids: vec![solid], ..Truth::default() })
}

/// One part of one prism, unplaced.
fn prism_part(dir: &Path, name: &str, prism: Prism, style: Style) -> std::io::Result<()> {
    let mut s = Step::new(Unit::Millimetre);
    s.part(name, Unit::Millimetre, &[], &[(prism.clone(), style)]);
    write(dir, name, &s, &Truth { prisms: vec![prism], ..Truth::default() })
}

fn main() -> std::io::Result<()> {
    let out = std::env::args().nth(1).unwrap_or_else(|| "corpus/generated".into());
    let dir = Path::new(&out);
    std::fs::create_dir_all(dir)?;

    // Faces that wrap all the way round, bounded only by two circles, with no seam
    // edge: how Fusion writes every cylinder. The mesher has to find the seam itself.
    part(dir, "cylinder", Unit::Millimetre, cylinder(), Style::Analytic)?;
    // The same with a seam edge, as OpenCascade writes it: the control.
    part(dir, "cylinder_seam", Unit::Millimetre, cylinder(), Style::Seam)?;
    // The side as the rational B-spline surface that is exactly a cylinder, closed
    // round, no seam: the shape that put surface 108 mm off a private part.
    part(dir, "cylinder_nurbs", Unit::Millimetre, cylinder(), Style::Nurbs)?;

    // A cone to a point: the face pinches to its apex, which no edge marks.
    let cone = Solid::new(vec![
        seg([0.0, 0.0], [0.0, 10.0]),
        seg([0.0, 10.0], [30.0, 0.0]),
        seg([30.0, 0.0], [0.0, 0.0]),
    ]);
    part(dir, "cone_point", Unit::Millimetre, cone.clone(), Style::Analytic)?;
    // The same with a vertex loop marking the apex: the control.
    part(dir, "cone_point_vertex", Unit::Millimetre, cone, Style::Pinned)?;

    // A dome: a hemisphere 20 mm in radius on a disc. The pole is inside the face.
    let dome = Solid::new(vec![
        seg([0.0, 0.0], [0.0, 20.0]),
        arc([0.0, 0.0], 20.0, 90.0, -90.0),
        seg([20.0, 0.0], [0.0, 0.0]),
    ]);
    part(dir, "dome", Unit::Millimetre, dome.clone(), Style::Analytic)?;
    // The dome as a rational B-spline: a whole row of control points at the pole.
    part(dir, "dome_nurbs", Unit::Millimetre, dome, Style::Nurbs)?;

    // A washer: annular faces with a hole in them, and an inner cylinder whose
    // normal points at the axis.
    let washer = Solid::new(vec![
        seg([0.0, 5.0], [0.0, 20.0]),
        seg([0.0, 20.0], [3.0, 20.0]),
        seg([3.0, 20.0], [3.0, 5.0]),
        seg([3.0, 5.0], [0.0, 5.0]),
    ]);
    part(dir, "washer", Unit::Millimetre, washer, Style::Analytic)?;

    // Half a torus on a flat ring: a doubly curved face bounded by its inner and
    // outer equators, wrapping round between them.
    let ring = Solid::new(vec![
        seg([0.0, 15.0], [0.0, 25.0]),
        arc([0.0, 20.0], 5.0, 90.0, -180.0),
    ]);
    part(dir, "torus_ring", Unit::Millimetre, ring.clone(), Style::Analytic)?;
    part(dir, "torus_ring_nurbs", Unit::Millimetre, ring, Style::Nurbs)?;

    // The cylinder in inches: a reader that ignores the unit makes it 25.4 times
    // too small.
    part(dir, "cylinder_inch", Unit::Inch, cylinder(), Style::Analytic)?;

    // Scale: a disc 3.6 m across and 19 mm thick, about a 12 ft sheet; and a pin
    // half a millimetre across.
    let sheet = Solid::new(vec![
        seg([0.0, 0.0], [0.0, 1800.0]),
        seg([0.0, 1800.0], [19.0, 1800.0]),
        seg([19.0, 1800.0], [19.0, 0.0]),
        seg([19.0, 0.0], [0.0, 0.0]),
    ]);
    part(dir, "sheet_disc", Unit::Millimetre, sheet, Style::Analytic)?;
    let pin = Solid::new(vec![
        seg([0.0, 0.0], [0.0, 0.25]),
        seg([0.0, 0.25], [2.0, 0.25]),
        seg([2.0, 0.25], [2.0, 0.0]),
        seg([2.0, 0.0], [0.0, 0.0]),
    ]);
    part(dir, "tiny_pin", Unit::Millimetre, pin, Style::Analytic)?;

    // Far from the origin: the cylinder 100 m out along X, where single precision
    // and careless tolerances run out.
    let far = cylinder().placed(DVec3::new(100_000.0, 0.0, 0.0), DVec3::Z, DVec3::X);
    part(dir, "far_cylinder", Unit::Millimetre, far, Style::Analytic)?;

    // In an assembly: the cylinder as a component, tipped over and moved.
    let tipped = (
        DVec3::new(40.0, -25.0, 10.0),
        DVec3::new(1.0, 1.0, 1.0).normalize(),
        DVec3::new(1.0, -1.0, 0.0).normalize(),
    );
    let mut s = Step::new(Unit::Millimetre);
    s.assembly("cylinder_placed", Unit::Millimetre, &cylinder(), Style::Analytic, &[tipped]);
    let solids = vec![cylinder().placed(tipped.0, tipped.1, tipped.2)];
    write(dir, "cylinder_placed", &s, &Truth { solids, ..Truth::default() })?;

    // The same component used twice, the second turned on its side.
    let twice = [
        (DVec3::ZERO, DVec3::Z, DVec3::X),
        (DVec3::new(60.0, 0.0, 10.0), DVec3::X, DVec3::Y),
    ];
    let mut s = Step::new(Unit::Millimetre);
    s.assembly("cylinder_twice", Unit::Millimetre, &cylinder(), Style::Analytic, &twice);
    let solids = twice.iter().map(|&(o, z, x)| cylinder().placed(o, z, x)).collect();
    write(dir, "cylinder_twice", &s, &Truth { solids, ..Truth::default() })?;

    // Big parts, to time readers on, written with seam edges as OpenCascade writes
    // them, which every reader here reads. A plate 300 mm square with 900 holes,
    // like a router's spoilboard: one solid of 910 faces, the two biggest each
    // holding 900 holes. The same plate 100 mm square with 100 holes, to show how
    // a reader's time grows with the part. With 1600 holes the slowest reader here
    // came within seconds of the timeout.
    prism_part(dir, "plate_100_holes", plate(10), Style::Seam)?;
    prism_part(dir, "plate_900_holes", plate(30), Style::Seam)?;
    // A shaft half a metre long, ringed by 250 grooves: 753 faces, each wrapping
    // all the way round.
    part(dir, "grooved_shaft", Unit::Millimetre, grooved_shaft(250), Style::Seam)?;

    Ok(())
}
