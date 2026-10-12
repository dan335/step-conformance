//! Writing solids of revolution and prisms as STEP (AP214), in the dialect a test
//! asks for.

use std::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_2, SQRT_2, TAU};

use glam::{DVec2, DVec3};

use crate::shape::{Element, Prism, Solid};

/// How a solid's faces and edges are written.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Style {
    /// Planes, cylinders, cones, spheres and tori, each face bounded only by the
    /// circles where it meets its neighbours, as Fusion writes them: no seam edge
    /// down a face that wraps all the way round.
    Analytic,
    /// The same, with a seam edge down every face that wraps round between two
    /// circles, as OpenCascade writes them.
    Seam,
    /// Every curved face as the rational B-spline surface that is exactly it, and
    /// every edge as a rational B-spline circle. Planes stay planes.
    Nurbs,
    /// Analytic, with a vertex loop wherever a face pinches to a point on the axis,
    /// such as a cone's apex.
    Pinned,
}

/// The file's length unit.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Unit {
    Millimetre,
    Inch,
}

impl Unit {
    fn mm(self) -> f64 {
        match self {
            Unit::Millimetre => 1.0,
            Unit::Inch => 25.4,
        }
    }
}

/// A STEP file being written: its entities in order, `#1` first.
pub struct Step {
    entities: Vec<String>,
    /// Millimetres per file unit; lengths are divided by it as they are written.
    scale: f64,
}

impl Step {
    pub fn new(unit: Unit) -> Self {
        Self {
            entities: Vec::new(),
            scale: unit.mm(),
        }
    }

    fn add(&mut self, body: String) -> u64 {
        self.entities.push(body);
        self.entities.len() as u64
    }

    fn point(&mut self, p: DVec3) -> u64 {
        let p = p / self.scale;
        self.add(format!("CARTESIAN_POINT('',({},{},{}))", real(p.x), real(p.y), real(p.z)))
    }

    fn direction(&mut self, d: DVec3) -> u64 {
        let d = d.normalize();
        self.add(format!("DIRECTION('',({},{},{}))", real(d.x), real(d.y), real(d.z)))
    }

    fn placement(&mut self, origin: DVec3, z: DVec3, x: DVec3) -> u64 {
        let (o, z, x) = (self.point(origin), self.direction(z), self.direction(x));
        self.add(format!("AXIS2_PLACEMENT_3D('',#{o},#{z},#{x})"))
    }

    fn length(&self, v: f64) -> String {
        real(v / self.scale)
    }

    /// The whole file.
    pub fn text(&self, name: &str) -> String {
        let mut out = format!(
            "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('step-conformance generated part'),'2;1');\n\
             FILE_NAME('{name}.step','2026-10-12T00:00:00',(''),(''),'step-conformance generate',\
             'step-conformance generate','');\n\
             FILE_SCHEMA(('AUTOMOTIVE_DESIGN {{ 1 0 10303 214 1 1 1 1 }}'));\nENDSEC;\nDATA;\n"
        );
        for (i, e) in self.entities.iter().enumerate() {
            out += &format!("#{}={e};\n", i + 1);
        }
        out + "ENDSEC;\nEND-ISO-10303-21;\n"
    }

    /// The representation context: the unit, and an uncertainty of 1e-7 mm.
    pub fn context(&mut self, unit: Unit) -> u64 {
        let mm = self.add("( LENGTH_UNIT() NAMED_UNIT(*) SI_UNIT(.MILLI.,.METRE.) )".into());
        let length = match unit {
            Unit::Millimetre => mm,
            Unit::Inch => {
                let dims = self.add("DIMENSIONAL_EXPONENTS(1.,0.,0.,0.,0.,0.,0.)".into());
                let factor = self.add(format!("LENGTH_MEASURE_WITH_UNIT(LENGTH_MEASURE(25.4),#{mm})"));
                self.add(format!(
                    "( CONVERSION_BASED_UNIT('INCH',#{factor}) LENGTH_UNIT() NAMED_UNIT(#{dims}) )"
                ))
            }
        };
        let angle = self.add("( NAMED_UNIT(*) PLANE_ANGLE_UNIT() SI_UNIT($,.RADIAN.) )".into());
        let solid = self.add("( NAMED_UNIT(*) SI_UNIT($,.STERADIAN.) SOLID_ANGLE_UNIT() )".into());
        let uncertainty = self.add(format!(
            "UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE({}),#{length},'distance_accuracy_value','confusion accuracy')",
            self.length(1e-7)
        ));
        self.add(format!(
            "( GEOMETRIC_REPRESENTATION_CONTEXT(3) GLOBAL_UNCERTAINTY_ASSIGNED_CONTEXT((#{uncertainty})) \
             GLOBAL_UNIT_ASSIGNED_CONTEXT((#{length},#{angle},#{solid})) REPRESENTATION_CONTEXT('','3D') )"
        ))
    }

    /// A product and its definition: `(product definition, its shape)`.
    pub fn product(&mut self, name: &str) -> (u64, u64) {
        let app = self.add("APPLICATION_CONTEXT('automotive design')".into());
        self.add(format!(
            "APPLICATION_PROTOCOL_DEFINITION('international standard','automotive_design',2000,#{app})"
        ));
        let pc = self.add(format!("PRODUCT_CONTEXT('',#{app},'mechanical')"));
        let pdc = self.add(format!("PRODUCT_DEFINITION_CONTEXT('part definition',#{app},'design')"));
        let p = self.add(format!("PRODUCT('{name}','{name}','',(#{pc}))"));
        let pdf = self.add(format!("PRODUCT_DEFINITION_FORMATION('','',#{p})"));
        let pd = self.add(format!("PRODUCT_DEFINITION('design','',#{pdf},#{pdc})"));
        let pds = self.add(format!("PRODUCT_DEFINITION_SHAPE('','',#{pd})"));
        (pd, pds)
    }

    /// A single part: one product whose shape is `solids` and `prisms`, as one B-rep
    /// each.
    pub fn part(&mut self, name: &str, unit: Unit, solids: &[(Solid, Style)], prisms: &[(Prism, Style)]) {
        let ctx = self.context(unit);
        let (_, pds) = self.product(name);
        let origin = self.placement(DVec3::ZERO, DVec3::Z, DVec3::X);
        let mut items = vec![origin];
        for (solid, style) in solids {
            items.push(self.solid(solid, *style));
        }
        for (prism, style) in prisms {
            items.push(self.prism(prism, *style));
        }
        let rep = self.add(format!(
            "ADVANCED_BREP_SHAPE_REPRESENTATION('',({}),#{ctx})",
            refs(&items)
        ));
        self.add(format!("SHAPE_DEFINITION_REPRESENTATION(#{pds},#{rep})"));
    }

    /// An assembly: a root product holding `solid` as a component once for each
    /// placement `(origin, axis, x)` in `at`.
    pub fn assembly(&mut self, name: &str, unit: Unit, solid: &Solid, style: Style, at: &[(DVec3, DVec3, DVec3)]) {
        let ctx = self.context(unit);
        let (root, root_shape) = self.product(&format!("{name} assembly"));
        let (part, part_shape) = self.product(name);
        let part_origin = self.placement(DVec3::ZERO, DVec3::Z, DVec3::X);
        let brep = self.solid(solid, style);
        let part_rep = self.add(format!(
            "ADVANCED_BREP_SHAPE_REPRESENTATION('',(#{brep},#{part_origin}),#{ctx})"
        ));
        self.add(format!("SHAPE_DEFINITION_REPRESENTATION(#{part_shape},#{part_rep})"));
        let root_origin = self.placement(DVec3::ZERO, DVec3::Z, DVec3::X);
        let mut items = vec![root_origin];
        let targets: Vec<u64> = at.iter().map(|&(o, z, x)| self.placement(o, z, x)).collect();
        items.extend(&targets);
        let root_rep = self.add(format!("SHAPE_REPRESENTATION('',({}),#{ctx})", refs(&items)));
        self.add(format!("SHAPE_DEFINITION_REPRESENTATION(#{root_shape},#{root_rep})"));
        for (i, target) in targets.iter().enumerate() {
            let usage = self.add(format!(
                "NEXT_ASSEMBLY_USAGE_OCCURRENCE('{}','{name}','',#{root},#{part},$)",
                i + 1
            ));
            let usage_shape = self.add(format!("PRODUCT_DEFINITION_SHAPE('','',#{usage})"));
            let moved = self.add(format!("ITEM_DEFINED_TRANSFORMATION('','',#{part_origin},#{target})"));
            let rel = self.add(format!(
                "( REPRESENTATION_RELATIONSHIP('','',#{part_rep},#{root_rep}) \
                 REPRESENTATION_RELATIONSHIP_WITH_TRANSFORMATION(#{moved}) SHAPE_REPRESENTATION_RELATIONSHIP() )"
            ));
            self.add(format!("CONTEXT_DEPENDENT_SHAPE_REPRESENTATION(#{rel},#{usage_shape})"));
        }
    }

    /// One solid of revolution as a closed shell: a face for every piece of its
    /// profile off the axis, and a circle edge wherever two of them meet off it.
    fn solid(&mut self, s: &Solid, style: Style) -> u64 {
        let o = DVec3::from(s.origin);
        let a = DVec3::from(s.axis).normalize();
        let x = s.x;
        let y = a.cross(x);
        let world = |zr: DVec2| o + zr.x * a + zr.y * x;

        // A circle edge at every joint off the axis, its one vertex where the turn
        // starts: joint i is where piece i starts.
        let n = s.profile.len();
        let mut circles = Vec::with_capacity(n);
        for el in &s.profile {
            let zr = el.at(0.0);
            if zr.y.abs() < 1e-12 {
                circles.push(None);
                continue;
            }
            let curve = match style {
                Style::Nurbs => self.nurbs_circle(o + zr.x * a, a, x, y, zr.y),
                _ => {
                    let at = self.placement(o + zr.x * a, a, x);
                    let r = self.length(zr.y);
                    self.add(format!("CIRCLE('',#{at},{r})"))
                }
            };
            let p = self.point(world(zr));
            let v = self.add(format!("VERTEX_POINT('',#{p})"));
            let edge = self.add(format!("EDGE_CURVE('',#{v},#{v},#{curve},.T.)"));
            circles.push(Some((edge, v, zr)));
        }

        let mut faces = Vec::new();
        for (i, el) in s.profile.iter().enumerate() {
            if el.on_axis() {
                continue;
            }
            let start = circles[i];
            let end = circles[(i + 1) % n];
            let (surface, same) = self.surface(el, style, o, a, x, y);

            // The circle where a face starts runs counterclockwise about the axis,
            // the one where it ends clockwise; see the README.
            let mut bounds = Vec::new();
            let seam = style == Style::Seam && matches!(el, Element::Segment { .. });
            match (start, end) {
                (Some((se, sv, _)), Some((ee, ev, _))) if seam => {
                    let (sp, ep) = (world(el.at(0.0)), world(el.at(1.0)));
                    let line = self.line(sp, ep);
                    let seam = self.add(format!("EDGE_CURVE('',#{sv},#{ev},#{line},.T.)"));
                    let loop_edges = [
                        self.oriented(se, true),
                        self.oriented(seam, true),
                        self.oriented(ee, false),
                        self.oriented(seam, false),
                    ];
                    let l = self.add(format!("EDGE_LOOP('',({}))", refs(&loop_edges)));
                    bounds.push(self.add(format!("FACE_OUTER_BOUND('',#{l},.T.)")));
                }
                _ => {
                    // A plane's outer bound is its larger circle.
                    let mut loops: Vec<(u64, f64)> = Vec::new();
                    if let Some((e, _, zr)) = start {
                        let oe = self.oriented(e, true);
                        loops.push((self.add(format!("EDGE_LOOP('',(#{oe}))")), zr.y));
                    }
                    if let Some((e, _, zr)) = end {
                        let oe = self.oriented(e, false);
                        loops.push((self.add(format!("EDGE_LOOP('',(#{oe}))")), zr.y));
                    }
                    // A curved face that pinches to a point on the axis: mark it. A
                    // flat disc crosses the axis without pinching.
                    let flat = matches!(el, Element::Segment { from, to } if (to[0] - from[0]).abs() < 1e-12);
                    if style == Style::Pinned && !flat {
                        for (t, joint) in [(0.0, start), (1.0, end)] {
                            if joint.is_none() {
                                let p = self.point(world(el.at(t)));
                                let v = self.add(format!("VERTEX_POINT('',#{p})"));
                                loops.push((self.add(format!("VERTEX_LOOP('',#{v})")), -1.0));
                            }
                        }
                    }
                    loops.sort_by(|p, q| q.1.total_cmp(&p.1));
                    for (k, (l, _)) in loops.iter().enumerate() {
                        let kind = if k == 0 { "FACE_OUTER_BOUND" } else { "FACE_BOUND" };
                        bounds.push(self.add(format!("{kind}('',#{l},.T.)")));
                    }
                }
            }
            let same = if same { ".T." } else { ".F." };
            faces.push(self.add(format!("ADVANCED_FACE('',({}),#{surface},{same})", refs(&bounds))));
        }
        let shell = self.add(format!("CLOSED_SHELL('',({}))", refs(&faces)));
        self.add(format!("MANIFOLD_SOLID_BREP('',#{shell})"))
    }

    /// A prism as a closed shell: a cap at each end, and a wall for every piece of
    /// every loop. A wall that wraps all the way round, a round hole, is bounded by
    /// its two circles alone, or with a seam edge as well in `Style::Seam`. Prisms are
    /// written in those two styles only.
    fn prism(&mut self, p: &Prism, style: Style) -> u64 {
        assert!(
            matches!(style, Style::Analytic | Style::Seam),
            "a prism is written plainly or with seams"
        );
        let a = DVec3::from(p.axis).normalize();
        let x = DVec3::from(p.x).normalize();
        let f = Frame { o: DVec3::from(p.origin), a, x, y: a.cross(x) };
        let h = p.height;

        let mut faces = Vec::new();
        // Every loop's edges round the bottom and round the top, in the loop's order.
        let mut rims = Vec::new();
        for pieces in &p.loops {
            let n = pieces.len();
            // A vertex at every joint, below and above: joint i is where piece i starts.
            let mut joints = Vec::with_capacity(n);
            for el in pieces {
                let q = el.at(0.0);
                let (below, above) = (self.point(f.at(q, 0.0)), self.point(f.at(q, h)));
                joints.push((
                    self.add(format!("VERTEX_POINT('',#{below})")),
                    self.add(format!("VERTEX_POINT('',#{above})")),
                ));
            }
            // An edge up from every joint, but none up a round hole written plainly.
            let mut ups = Vec::with_capacity(n);
            for (el, &(below, above)) in pieces.iter().zip(&joints) {
                if n == 1 && style == Style::Analytic {
                    ups.push(None);
                    continue;
                }
                let q = el.at(0.0);
                let line = self.line(f.at(q, 0.0), f.at(q, h));
                ups.push(Some(self.add(format!("EDGE_CURVE('',#{below},#{above},#{line},.T.)"))));
            }
            // Every piece's edge along the bottom and along the top, from its joint to
            // the next.
            let (mut bottom, mut top) = (Vec::with_capacity(n), Vec::with_capacity(n));
            for (i, el) in pieces.iter().enumerate() {
                let (start, end) = (joints[i], joints[(i + 1) % n]);
                bottom.push(self.prism_edge(el, &f, 0.0, start.0, end.0));
                top.push(self.prism_edge(el, &f, h, start.1, end.1));
            }
            // The walls, bounded counterclockwise seen from outside: along the bottom,
            // up, back along the top, and down.
            for (i, el) in pieces.iter().enumerate() {
                let (surface, same) = self.prism_wall(el, &f);
                let mut bounds = Vec::new();
                match (ups[i], ups[(i + 1) % n]) {
                    (Some(down), Some(up)) => {
                        let edges = [
                            self.oriented(bottom[i], true),
                            self.oriented(up, true),
                            self.oriented(top[i], false),
                            self.oriented(down, false),
                        ];
                        let l = self.add(format!("EDGE_LOOP('',({}))", refs(&edges)));
                        bounds.push(self.add(format!("FACE_OUTER_BOUND('',#{l},.T.)")));
                    }
                    _ => {
                        let (b, t) = (self.oriented(bottom[i], true), self.oriented(top[i], false));
                        let lb = self.add(format!("EDGE_LOOP('',(#{b}))"));
                        let lt = self.add(format!("EDGE_LOOP('',(#{t}))"));
                        bounds.push(self.add(format!("FACE_OUTER_BOUND('',#{lb},.T.)")));
                        bounds.push(self.add(format!("FACE_BOUND('',#{lt},.T.)")));
                    }
                }
                let same = if same { ".T." } else { ".F." };
                faces.push(self.add(format!("ADVANCED_FACE('',({}),#{surface},{same})", refs(&bounds))));
            }
            rims.push((bottom, top));
        }
        // The caps. The top looks along the axis, so its loops run as the profile's
        // do; the bottom looks back down it, so its loops run backwards.
        for (w, up) in [(0.0, false), (h, true)] {
            let at = self.placement(f.at(DVec2::ZERO, w), a, x);
            let plane = self.add(format!("PLANE('',#{at})"));
            let mut bounds = Vec::new();
            for (k, (bottom, top)) in rims.iter().enumerate() {
                let edges: Vec<u64> = if up {
                    top.iter().map(|&e| self.oriented(e, true)).collect()
                } else {
                    bottom.iter().rev().map(|&e| self.oriented(e, false)).collect()
                };
                let l = self.add(format!("EDGE_LOOP('',({}))", refs(&edges)));
                let kind = if k == 0 { "FACE_OUTER_BOUND" } else { "FACE_BOUND" };
                bounds.push(self.add(format!("{kind}('',#{l},.T.)")));
            }
            let same = if up { ".T." } else { ".F." };
            faces.push(self.add(format!("ADVANCED_FACE('',({}),#{plane},{same})", refs(&bounds))));
        }
        let shell = self.add(format!("CLOSED_SHELL('',({}))", refs(&faces)));
        self.add(format!("MANIFOLD_SOLID_BREP('',#{shell})"))
    }

    /// A piece of a prism's profile as an edge `w` along the axis, from vertex `from`
    /// to vertex `to`.
    fn prism_edge(&mut self, el: &Element, f: &Frame, w: f64, from: u64, to: u64) -> u64 {
        match *el {
            Element::Segment { from: p, to: q } => {
                let line = self.line(f.at(p.into(), w), f.at(q.into(), w));
                self.add(format!("EDGE_CURVE('',#{from},#{to},#{line},.T.)"))
            }
            Element::Arc { center, radius, start, sweep } => {
                // The circle runs counterclockwise about the axis from where the arc
                // starts; a clockwise arc runs against it.
                let at = self.placement(f.at(center.into(), w), f.a, f.dir(DVec2::from_angle(start)));
                let r = self.length(radius);
                let circle = self.add(format!("CIRCLE('',#{at},{r})"));
                let same = if sweep > 0.0 { ".T." } else { ".F." };
                self.add(format!("EDGE_CURVE('',#{from},#{to},#{circle},{same})"))
            }
        }
    }

    /// The wall a piece of a prism's profile sweeps, and whether its own normal points
    /// out of the material, which lies on the piece's left.
    fn prism_wall(&mut self, el: &Element, f: &Frame) -> (u64, bool) {
        match *el {
            Element::Segment { from, to } => {
                let along = DVec2::from(to) - DVec2::from(from);
                let out = DVec2::new(along.y, -along.x);
                let at = self.placement(f.at(from.into(), 0.0), f.dir(out), f.dir(along));
                (self.add(format!("PLANE('',#{at})")), true)
            }
            Element::Arc { center, radius, start, sweep } => {
                let at = self.placement(f.at(center.into(), 0.0), f.a, f.dir(DVec2::from_angle(start)));
                let r = self.length(radius);
                // A cylinder's normal points away from its axis, which is out of the
                // material where the arc runs counterclockwise.
                (self.add(format!("CYLINDRICAL_SURFACE('',#{at},{r})")), sweep > 0.0)
            }
        }
    }

    fn oriented(&mut self, edge: u64, sense: bool) -> u64 {
        self.add(format!("ORIENTED_EDGE('',*,*,#{edge},{})", if sense { ".T." } else { ".F." }))
    }

    fn line(&mut self, from: DVec3, to: DVec3) -> u64 {
        let p = self.point(from);
        let d = self.direction(to - from);
        let len = self.length((to - from).length());
        let v = self.add(format!("VECTOR('',#{d},{len})"));
        self.add(format!("LINE('',#{p},#{v})"))
    }

    /// The surface a piece of profile sweeps out, and whether its own normal points
    /// out of the solid.
    fn surface(&mut self, el: &Element, style: Style, o: DVec3, a: DVec3, x: DVec3, y: DVec3) -> (u64, bool) {
        let mid = el.at(0.5);
        let out = el.outward(0.5);
        // Normals below are (dz, dr) in the half-plane through the axis.
        let natural_is_out = |n: DVec2| n.dot(out) > 0.0;
        match *el {
            Element::Segment { from, to } if (to[0] - from[0]).abs() < 1e-12 => {
                let at = self.placement(o + from[0] * a, a, x);
                (self.add(format!("PLANE('',#{at})")), natural_is_out(DVec2::new(1.0, 0.0)))
            }
            _ if style == Style::Nurbs => (self.nurbs_revolved(el, o, a, x, y), true),
            Element::Segment { from, to } if (to[1] - from[1]).abs() < 1e-12 => {
                let at = self.placement(o + from[0] * a, a, x);
                let r = self.length(from[1]);
                (
                    self.add(format!("CYLINDRICAL_SURFACE('',#{at},{r})")),
                    natural_is_out(DVec2::new(0.0, 1.0)),
                )
            }
            Element::Segment { from, to } => {
                // Placed at the wider end, its axis pointing the way it widens.
                let (wide, dz, dr) = if from[1] >= to[1] {
                    (from, from[0] - to[0], from[1] - to[1])
                } else {
                    (to, to[0] - from[0], to[1] - from[1])
                };
                let s = dz.signum();
                let angle = (dr / dz.abs()).atan();
                let at = self.placement(o + wide[0] * a, s * a, x);
                let r = self.length(wide[1]);
                let natural = DVec2::new(-s * angle.sin(), angle.cos());
                (
                    self.add(format!("CONICAL_SURFACE('',#{at},{r},{})", real(angle))),
                    natural_is_out(natural),
                )
            }
            Element::Arc { center, radius, .. } => {
                let c = DVec2::from(center);
                let at = self.placement(o + c.x * a, a, x);
                let natural = (mid - c).normalize();
                let minor = self.length(radius);
                let body = if c.y.abs() < 1e-12 {
                    format!("SPHERICAL_SURFACE('',#{at},{minor})")
                } else {
                    let major = self.length(c.y);
                    format!("TOROIDAL_SURFACE('',#{at},{major},{minor})")
                };
                (self.add(body), natural_is_out(natural))
            }
        }
    }

    /// The profile piece as a rational B-spline: `(control points as (z, r),
    /// weights, degree, knots, multiplicities)`. An arc is split into pieces of at
    /// most a quarter turn, each one exact.
    fn nurbs_profile(el: &Element) -> (Vec<DVec2>, Vec<f64>, usize, Vec<f64>, Vec<usize>) {
        match *el {
            Element::Segment { from, to } => (
                vec![DVec2::from(from), DVec2::from(to)],
                vec![1.0, 1.0],
                1,
                vec![0.0, 1.0],
                vec![2, 2],
            ),
            Element::Arc {
                center,
                radius,
                start,
                sweep,
            } => {
                let pieces = (sweep.abs() / FRAC_PI_2 - 1e-9).ceil().max(1.0) as usize;
                let step = sweep / pieces as f64;
                let c = DVec2::from(center);
                let on = |t: f64| c + radius * DVec2::new(t.cos(), t.sin());
                let mut points = vec![on(start)];
                let mut weights = vec![1.0];
                for k in 0..pieces {
                    let t0 = start + step * k as f64;
                    let half = step / 2.0;
                    points.push(c + radius / half.cos() * DVec2::new((t0 + half).cos(), (t0 + half).sin()));
                    weights.push(half.cos());
                    points.push(on(t0 + step));
                    weights.push(1.0);
                }
                let knots = (0..=pieces).map(|k| k as f64 / pieces as f64).collect();
                let mut mult = vec![2; pieces + 1];
                mult[0] = 3;
                mult[pieces] = 3;
                (points, weights, 2, knots, mult)
            }
        }
    }

    /// The surface a profile piece sweeps, as the rational B-spline that is
    /// exactly it: a full circle (degree 2, nine points) around, the profile along.
    /// Its normal, the turn crossed with the profile's direction, points out.
    fn nurbs_revolved(&mut self, el: &Element, o: DVec3, a: DVec3, x: DVec3, y: DVec3) -> u64 {
        let (profile, pw, degree, knots, mult) = Self::nurbs_profile(el);
        let (ring, rw) = circle_net();
        let mut rows = Vec::new();
        let mut weights = Vec::new();
        for (k, c) in ring.iter().enumerate() {
            let mut row = Vec::new();
            let mut wrow = Vec::new();
            for (j, p) in profile.iter().enumerate() {
                let at = o + p.x * a + p.y * (c.x * x + c.y * y);
                row.push(self.point(at));
                wrow.push(real(rw[k] * pw[j]));
            }
            rows.push(format!("({})", refs(&row)));
            weights.push(format!("({})", wrow.join(",")));
        }
        let (u_knots, u_mult) = circle_knots();
        self.add(format!(
            "( BOUNDED_SURFACE() B_SPLINE_SURFACE(2,{degree},({}),.UNSPECIFIED.,.T.,.F.,.F.) \
             B_SPLINE_SURFACE_WITH_KNOTS(({}),({}),({}),({}),.UNSPECIFIED.) \
             GEOMETRIC_REPRESENTATION_ITEM() RATIONAL_B_SPLINE_SURFACE(({})) REPRESENTATION_ITEM('') SURFACE() )",
            rows.join(","),
            ints(&u_mult),
            ints(&mult),
            reals(&u_knots),
            reals(&knots),
            weights.join(",")
        ))
    }

    /// A full circle as a rational B-spline curve, starting where `x` points.
    fn nurbs_circle(&mut self, center: DVec3, _a: DVec3, x: DVec3, y: DVec3, r: f64) -> u64 {
        let (ring, rw) = circle_net();
        let points: Vec<u64> = ring
            .iter()
            .map(|c| self.point(center + r * (c.x * x + c.y * y)))
            .collect();
        let (knots, mult) = circle_knots();
        self.add(format!(
            "( BOUNDED_CURVE() B_SPLINE_CURVE(2,({}),.UNSPECIFIED.,.T.,.F.) \
             B_SPLINE_CURVE_WITH_KNOTS(({}),({}),.UNSPECIFIED.) CURVE() GEOMETRIC_REPRESENTATION_ITEM() \
             RATIONAL_B_SPLINE_CURVE(({})) REPRESENTATION_ITEM('') )",
            refs(&points),
            ints(&mult),
            reals(&knots),
            rw.iter().map(|w| real(*w)).collect::<Vec<_>>().join(",")
        ))
    }
}

/// A prism's frame: the plane of its profile, and the axis it is swept along.
struct Frame {
    o: DVec3,
    a: DVec3,
    x: DVec3,
    y: DVec3,
}

impl Frame {
    /// The profile's point `q`, `w` along the axis.
    fn at(&self, q: DVec2, w: f64) -> DVec3 {
        self.o + q.x * self.x + q.y * self.y + w * self.a
    }

    /// The direction `d` in the profile's plane.
    fn dir(&self, d: DVec2) -> DVec3 {
        d.x * self.x + d.y * self.y
    }
}

/// The nine control points of a unit circle as a rational quadratic B-spline,
/// counterclockwise from (1, 0), with their weights.
fn circle_net() -> (Vec<DVec2>, Vec<f64>) {
    (0..9)
        .map(|k| {
            let t = TAU * k as f64 / 8.0;
            let (s, w) = if k % 2 == 0 { (1.0, 1.0) } else { (SQRT_2, FRAC_1_SQRT_2) };
            (s * DVec2::new(t.cos(), t.sin()), w)
        })
        .unzip()
}

fn circle_knots() -> (Vec<f64>, Vec<usize>) {
    (vec![0.0, 0.25, 0.5, 0.75, 1.0], vec![3, 2, 2, 2, 3])
}

fn refs(ids: &[u64]) -> String {
    ids.iter().map(|i| format!("#{i}")).collect::<Vec<_>>().join(",")
}

fn ints(v: &[usize]) -> String {
    v.iter().map(ToString::to_string).collect::<Vec<_>>().join(",")
}

fn reals(v: &[f64]) -> String {
    v.iter().map(|x| real(*x)).collect::<Vec<_>>().join(",")
}

/// A STEP real: always a decimal point, an upper-case exponent, and the shortest
/// digits that read back to the same number.
fn real(v: f64) -> String {
    let v = if v == 0.0 { 0.0 } else { v }; // no "-0."
    let s = format!("{v:?}");
    match s.split_once('e') {
        Some((m, e)) => {
            let m = if m.contains('.') { m.to_string() } else { format!("{m}.") };
            format!("{m}E{e}")
        }
        None if s.contains('.') => s,
        None => format!("{s}."),
    }
}
