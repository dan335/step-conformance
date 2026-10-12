#!/usr/bin/env python3
"""Write a synthetic freeform panel as a STEP file: a public regression part for B-spline
meshing in truck-based STEP readers.

    python3 generate/freeform_panel.py corpus/freeform_panel.step
    python3 generate/freeform_panel.py OUT.step key=value ...   # override any DEFAULTS entry

Pure Python 3 (no packages), deterministic: the same arguments always give the same bytes.
The geometry is generated from the formulas below; nothing is taken from any other part.

The body (STEP unit inch, 5 faces, one closed shell):

* Outer wall: one cubic (3x3) B_SPLINE_SURFACE_WITH_KNOTS closed in U (u_closed .T., clamped
  knots, C2 across the seam) running round a wavy super-ellipse footprint. The surface runs
  past the part (below z=0 and above the top) and the face is trimmed by the bottom and top
  planes, so the face spans the whole U period and its loop is: bottom ring, seam, top ring,
  seam back. The rings are periodic cubic fits through points on the exact plane/surface
  intersection, so (as in CAD exports) they lie on the wall only to a fitting tolerance of
  about 1e-4..1e-3 mm. The top ring's samples are spaced a hair unevenly (`oskew_t`), which
  puts its parameter midpoint 0.3 um beside the wall's mid-U line.
* Pocket wall: a second closed-U cubic B-spline surface, trimmed by the top plane and by the
  pocket floor.
* Pocket floor: an open 24x24 cubic B-spline surface (z = f(x, y)) that runs past the pocket
  and is trimmed by the pocket wall.
* Bottom: a plane. Top: a plane with the pocket opening as an inner loop.

What it reproduces: OpenCascade meshes it within its tolerance at every tolerance tried;
SolidCraft's reader meshes it correctly at 0.0125 mm but at 0.05, 0.1 and 0.5 mm puts triangles
5 to 150 mm off the outer wall, warning "face wrapping around a closed surface kept whole:
cannot cut a ring edge at the split line"; upstream truck (robust_triangulation) puts triangles
off the wall, and its plain triangulation leaves the B-spline faces empty.

Why: the rings lie off the wall by more than the 1e-6 truck's exact inversion accepts, so every
boundary point falls back to a nearest-point search seeded from the point before. At coarse
tolerances those points are far apart, and at a concave bend the search leaves the face onto
the surface's extrapolated span, where the mesh then follows it. The numbers depend on these
exact bytes: the failure is chaotic in the geometry, so commit the file, don't regenerate it.
"""
import math
import os
import sys

# ---------------------------------------------------------------- B-spline helpers


def lerp(a, b, t):
    return tuple(x + (y - x) * t for x, y in zip(a, b))


def insert_knot(knots, cps, p, u):
    """Boehm: insert u once into a degree-p B-spline (knots, cps). Returns new (knots, cps)."""
    ncp = len(cps)
    # span k with knots[k] <= u <= knots[k+1], inside the domain (u may be its right end)
    k = max(i for i in range(p, ncp) if knots[i] <= u and (u < knots[i + 1] or i == ncp - 1))
    new = []
    for i in range(len(cps) + 1):
        if i <= k - p:
            new.append(cps[i])
        elif i > k:
            new.append(cps[i - 1])
        else:
            a = (u - knots[i]) / (knots[i + p] - knots[i])
            new.append(lerp(cps[i - 1], cps[i], a))
    return knots[: k + 1] + [u] + knots[k + 1:], new


def periodic_to_clamped(q, p=3):
    """A uniform periodic degree-p B-spline with control points q (wrapping) as a clamped,
    non-periodic B-spline: knots 0..n with end multiplicity p+1, n+p control points. The curve
    is closed and C^(p-1) across the seam; its parameter 0 is at (q0 + 4 q1 + q2) / 6."""
    n = len(q)
    cps = list(q) + list(q[:p])
    knots = [float(i) for i in range(n + 2 * p + 1)]
    lo, hi = float(p), float(n + p)
    for _ in range(p):
        knots, cps = insert_knot(knots, cps, p, lo)
    for _ in range(p):
        knots, cps = insert_knot(knots, cps, p, hi)
    i0 = knots.index(lo)
    i1 = len(knots) - 1 - knots[::-1].index(hi)
    knots = knots[i0: i1 + 1]
    cps = cps[i0: i0 + len(knots) - p - 1]
    knots = [k - lo for k in knots]
    return knots, cps


def basis(knots, p, u):
    n = len(knots) - p - 1
    if u >= knots[n]:
        u = knots[n] - 1e-12
    k = max(i for i in range(p, n) if knots[i] <= u)
    N = [0.0] * (p + 1)
    N[0] = 1.0
    left = [0.0] * (p + 1)
    right = [0.0] * (p + 1)
    for j in range(1, p + 1):
        left[j] = u - knots[k + 1 - j]
        right[j] = knots[k + j] - u
        saved = 0.0
        for r in range(j):
            tmp = N[r] / (right[r + 1] + left[j - r])
            N[r] = saved + right[r + 1] * tmp
            saved = left[j - r] * tmp
        N[j] = saved
    return k, N


def curve_point(knots, cps, p, u):
    k, N = basis(knots, p, u)
    out = [0.0, 0.0, 0.0]
    for r in range(p + 1):
        c = cps[k - p + r]
        for d in range(3):
            out[d] += N[r] * c[d]
    return tuple(out)


class Surf:
    """Tensor-product B-spline surface; grid[i][j] with i along U."""

    def __init__(self, ku, kv, grid, pu=3, pv=3):
        self.ku, self.kv, self.grid, self.pu, self.pv = ku, kv, grid, pu, pv

    def __call__(self, u, v):
        k, N = basis(self.ku, self.pu, u)
        l, M = basis(self.kv, self.pv, v)
        x = y = z = 0.0
        for a in range(self.pu + 1):
            row = self.grid[k - self.pu + a]
            for b in range(self.pv + 1):
                w = N[a] * M[b]
                p = row[l - self.pv + b]
                x += w * p[0]
                y += w * p[1]
                z += w * p[2]
        return (x, y, z)

    def u_range(self):
        return self.ku[0], self.ku[-1]


def solve_periodic(points):
    """Periodic uniform cubic control points q whose curve (periodic_to_clamped's convention,
    C(k) = (q_k + 4 q_{k+1} + q_{k+2}) / 6) passes through points[k] at parameter k."""
    m = len(points)
    r = [tuple(p) for p in points]
    for _ in range(200):
        r = [tuple((6 * points[k][d] - r[k - 1][d] - r[(k + 1) % m][d]) / 4 for d in range(3)) for k in range(m)]
    return [r[(j - 1) % m] for j in range(m)]


def bisect(f, a, b, it=80):
    fa = f(a)
    for _ in range(it):
        c = 0.5 * (a + b)
        fc = f(c)
        if (fc < 0) == (fa < 0):
            a, fa = c, fc
        else:
            b = c
    return 0.5 * (a + b)


def mults(knots):
    vals, ms = [], []
    for k in knots:
        if vals and abs(k - vals[-1]) < 1e-12:
            ms[-1] += 1
        else:
            vals.append(k)
            ms.append(1)
    return vals, ms


def superellipse(a, b, e, t):
    c, s = math.cos(t), math.sin(t)
    x = a * math.copysign(abs(c) ** (2.0 / e), c)
    y = b * math.copysign(abs(s) ** (2.0 / e), s)
    return x, y


# ---------------------------------------------------------------- STEP writer


def fnum(x):
    if abs(x) < 5e-16:
        x = 0.0
    s = repr(float(x))
    if "e" in s or "E" in s:
        m, e = s.lower().split("e")
        if "." not in m:
            m += "."
        return f"{m}E{int(e)}"
    if s.endswith(".0"):
        return s[:-1]
    return s


class Step:
    def __init__(self):
        self.lines = []
        self.n = 0

    def add(self, text):
        self.n += 1
        self.lines.append(f"#{self.n}={text};")
        return self.n

    def point(self, p):
        return self.add(f"CARTESIAN_POINT('',({','.join(fnum(c) for c in p)}))")

    def direction(self, d):
        return self.add(f"DIRECTION('',({','.join(fnum(c) for c in d)}))")

    def axis2(self, o, z, x):
        return self.add(f"AXIS2_PLACEMENT_3D('',#{self.point(o)},#{self.direction(z)},#{self.direction(x)})")

    def plane(self, o, z, x):
        return self.add(f"PLANE('',#{self.axis2(o, z, x)})")

    def bspline_curve(self, p, knots, cps, closed):
        ids = [self.point(c) for c in cps]
        vals, ms = mults(knots)
        return self.add(
            "B_SPLINE_CURVE_WITH_KNOTS('',%d,(%s),.UNSPECIFIED.,%s,.F.,(%s),(%s),.UNSPECIFIED.)"
            % (p, ",".join(f"#{i}" for i in ids), ".T." if closed else ".F.",
               ",".join(str(m) for m in ms), ",".join(fnum(v) for v in vals)))

    def bspline_surface(self, pu, pv, ku, kv, grid, uclosed, vclosed):
        rows = []
        for col in grid:  # outer list: u index
            rows.append("(" + ",".join(f"#{self.point(c)}" for c in col) + ")")
        uv, um = mults(ku)
        vv, vm = mults(kv)
        return self.add(
            "B_SPLINE_SURFACE_WITH_KNOTS('',%d,%d,(%s),.UNSPECIFIED.,%s,%s,.F.,(%s),(%s),(%s),(%s),.UNSPECIFIED.)"
            % (pu, pv, ",".join(rows), ".T." if uclosed else ".F.", ".T." if vclosed else ".F.",
               ",".join(str(m) for m in um), ",".join(str(m) for m in vm),
               ",".join(fnum(v) for v in uv), ",".join(fnum(v) for v in vv)))

    def vertex(self, p):
        return self.add(f"VERTEX_POINT('',#{self.point(p)})")

    def edge(self, v0, v1, curve, same=True):
        return self.add(f"EDGE_CURVE('',#{v0},#{v1},#{curve},{'.T.' if same else '.F.'})")

    def loop(self, oedges):
        ids = [self.add(f"ORIENTED_EDGE('',*,*,#{e},{'.T.' if o else '.F.'})") for e, o in oedges]
        return self.add("EDGE_LOOP('',(%s))" % ",".join(f"#{i}" for i in ids))

    def face(self, loops, surf, same=True):
        bounds = []
        for i, lp in enumerate(loops):
            kind = "FACE_OUTER_BOUND" if i == 0 else "FACE_BOUND"
            bounds.append(self.add(f"{kind}('',#{lp},.T.)"))
        return self.add("ADVANCED_FACE('',(%s),#%d,%s)" % (",".join(f"#{b}" for b in bounds), surf, ".T." if same else ".F."))

    def text(self, faces, name, inch):
        body = list(self.lines)
        self.lines = []
        shell = self.add("CLOSED_SHELL('',(%s))" % ",".join(f"#{f}" for f in faces))
        brep = self.add(f"MANIFOLD_SOLID_BREP('{name}',#{shell})")
        origin = self.axis2((0, 0, 0), (0, 0, 1), (1, 0, 0))
        if inch:
            mm = self.add("( LENGTH_UNIT() NAMED_UNIT(*) SI_UNIT(.MILLI.,.METRE.) )")
            lm = self.add(f"LENGTH_MEASURE_WITH_UNIT(LENGTH_MEASURE(25.4),#{mm})")
            dims = self.add("DIMENSIONAL_EXPONENTS(1.,0.,0.,0.,0.,0.,0.)")
            length = self.add(f"( CONVERSION_BASED_UNIT('INCH',#{lm}) LENGTH_UNIT() NAMED_UNIT(#{dims}) )")
            unc = 1e-6
        else:
            length = self.add("( LENGTH_UNIT() NAMED_UNIT(*) SI_UNIT(.MILLI.,.METRE.) )")
            unc = 1e-5
        angle = self.add("( NAMED_UNIT(*) PLANE_ANGLE_UNIT() SI_UNIT($,.RADIAN.) )")
        solid_angle = self.add("( NAMED_UNIT(*) SI_UNIT($,.STERADIAN.) SOLID_ANGLE_UNIT() )")
        u = self.add(f"UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE({fnum(unc)}),#{length},'distance_accuracy_value','confusion accuracy')")
        ctx = self.add(
            f"( GEOMETRIC_REPRESENTATION_CONTEXT(3) GLOBAL_UNCERTAINTY_ASSIGNED_CONTEXT((#{u})) "
            f"GLOBAL_UNIT_ASSIGNED_CONTEXT((#{length},#{angle},#{solid_angle})) REPRESENTATION_CONTEXT('Context #1','3D Context with UNIT and UNCERTAINTY') )")
        rep = self.add(f"ADVANCED_BREP_SHAPE_REPRESENTATION('',(#{origin},#{brep}),#{ctx})")
        app = self.add("APPLICATION_CONTEXT('core data for automotive mechanical design processes')")
        self.add(f"APPLICATION_PROTOCOL_DEFINITION('international standard','automotive_design',2000,#{app})")
        pc = self.add(f"PRODUCT_CONTEXT('',#{app},'mechanical')")
        prod = self.add(f"PRODUCT('{name}','{name}','',(#{pc}))")
        pdf = self.add(f"PRODUCT_DEFINITION_FORMATION('','',#{prod})")
        pdc = self.add(f"PRODUCT_DEFINITION_CONTEXT('part definition',#{app},'design')")
        pd = self.add(f"PRODUCT_DEFINITION('design','',#{pdf},#{pdc})")
        pds = self.add(f"PRODUCT_DEFINITION_SHAPE('','',#{pd})")
        self.add(f"SHAPE_DEFINITION_REPRESENTATION(#{pds},#{rep})")
        self.add(f"PRODUCT_RELATED_PRODUCT_CATEGORY('part',$,(#{prod}))")
        head = [
            "ISO-10303-21;",
            "HEADER;",
            "FILE_DESCRIPTION(('synthetic freeform panel'),'2;1');",
            f"FILE_NAME('{name}.step','2026-01-01T00:00:00',('camcraft'),('camcraft'),'make_freeform_panel.py','make_freeform_panel.py','');",
            "FILE_SCHEMA(('AUTOMOTIVE_DESIGN { 1 0 10303 214 1 1 1 1 }'));",
            "ENDSEC;",
            "DATA;",
        ]
        return "\n".join(head + body + self.lines + ["ENDSEC;", "END-ISO-10303-21;", ""])


# ---------------------------------------------------------------- the part


def build(cfg):
    """STEP text and a few facts about the part (millimetres)."""
    sc = (1.0 / 25.4 if cfg["inch"] else 1.0) * cfg["scale"]  # model mm -> file units
    st = Step()
    log = []

    def S(p):
        return tuple(c * sc for c in p)

    def outward(fp, t):
        dt = 1e-5
        x1, y1 = fp(t + dt)
        x0, y0 = fp(t - dt)
        tx, ty = x1 - x0, y1 - y0
        L = math.hypot(tx, ty)
        return (ty / L, -tx / L)  # outward of the footprint (which runs CCW in t)

    def closed_surface(n, fp, rows, ccw, phase):
        """A band closed in U: n periodic columns round footprint fp (CCW, or CW for a pocket),
        rows = [(z(t), outward offset(t))] along V (4 rows: Bezier in V)."""
        cols = []
        for i in range(n):
            t = phase + 2 * math.pi * i / n * (1 if ccw else -1)
            x, y = fp(t)
            nx, ny = outward(fp, t)
            cols.append([(x + nx * o(t), y + ny * o(t), z(t)) for z, o in rows])
        m = len(rows)
        rows_cl = []
        for j in range(m):
            ku, cl = periodic_to_clamped([c[j] for c in cols], 3)
            rows_cl.append(cl)
        grid = [[rows_cl[j][i] for j in range(m)] for i in range(len(rows_cl[0]))]
        pv = min(3, m - 1)
        inner = m - pv - 1
        kv = [0.0] * (pv + 1) + [float(k) / (inner + 1) for k in range(1, inner + 1)] + [1.0] * (pv + 1)
        return Surf(ku, kv, grid, 3, pv)

    def trim_ring(W, g, msamp, skew=0.0):
        """The closed curve on W where g(point) = 0 (found per U sample by bisection in V),
        fitted by a periodic cubic through msamp samples. Knots normalised to [0, 1].
        Returns (knots, cps, fit deviation estimate in mm)."""
        u0, u1 = W.u_range()
        pts = []
        for k in range(msamp):
            u = u0 + (u1 - u0) * (k / msamp + skew * math.sin(math.pi * k / msamp) ** 2)
            v = bisect(lambda v: g(W(u, v)), 0.0, 1.0)
            pts.append(W(u, v))
        q = solve_periodic(pts)
        kn, cps = periodic_to_clamped(q, 3)
        # fit deviation: distance from the curve between samples to the exact trim curve
        # (golden-section search along U near the sample; for the log only)
        dev = 0.0
        step = (u1 - u0) / msamp

        def trim_dist(u, c):
            v = bisect(lambda v: g(W(u, v)), 0.0, 1.0, 60)
            return math.dist(W(u, v), c)

        for k in range(0, msamp, max(1, msamp // 40)):
            c = curve_point(kn, cps, 3, k + 0.5)
            a, b = max(u0, u0 + step * (k - 1.5)), min(u1, u0 + step * (k + 2.5))
            gr = (math.sqrt(5) - 1) / 2
            x1, x2 = b - gr * (b - a), a + gr * (b - a)
            f1, f2 = trim_dist(x1, c), trim_dist(x2, c)
            for _ in range(50):
                if f1 < f2:
                    b, x2, f2 = x2, x1, f1
                    x1 = b - gr * (b - a)
                    f1 = trim_dist(x1, c)
                else:
                    a, x1, f1 = x1, x2, f2
                    x2 = a + gr * (b - a)
                    f2 = trim_dist(x2, c)
            dev = max(dev, min(f1, f2))
        ks = 1.0 / msamp
        kn = [k * ks for k in kn]
        return kn, cps, dev * cfg["scale"]

    def wall_face(W, ring_lo, ring_hi):
        """Face on a closed band: lower ring forward, seam up, upper ring back, seam down.
        Each ring is one closed edge starting at the seam vertex."""
        surf = st.bspline_surface(3, W.pv, W.ku, W.kv, [[S(p) for p in c] for c in W.grid], True, False)
        vb = st.vertex(S(ring_lo[1][0]))
        rb = st.edge(vb, vb, st.bspline_curve(3, ring_lo[0], [S(p) for p in ring_lo[1]], True))
        vt = st.vertex(S(ring_hi[1][0]))
        rt = st.edge(vt, vt, st.bspline_curve(3, ring_hi[0], [S(p) for p in ring_hi[1]], True))
        se = st.edge(vb, vt, st.bspline_curve(W.pv, W.kv, [S(p) for p in W.grid[0]], False))
        lp = st.loop([(rb, True), (se, True), (rt, False), (se, False)])
        return st.face([lp], surf, True), rb, rt

    A, B, H, E = cfg["a"], cfg["b"], cfg["h"], cfg["e"]
    wav, nw = cfg["wave"], cfg["wave_n"]

    def outer_fp(t):
        x, y = superellipse(A, B, E, t)
        r = 1.0 + wav * math.sin(nw * t + 0.7) + 0.5 * wav * math.cos((nw + 3) * t)
        return x * r, y * r

    eb, et = cfg["ext_b"], cfg["ext_t"]
    bul, fl = cfg["bulge"], cfg["flare"]
    zw = cfg["zwave"]

    def z_lo(t):
        return -eb * (1 + zw * math.sin(2 * t + 0.4))

    def z_hi(t):
        return H + et * (1 + zw * math.sin(3 * t + 1.0))

    # Outer wall rows: flared out below z=0, drafted in above the top (the face is the middle).
    orows = [
        (z_lo, lambda t: fl * (1 + 0.5 * math.sin(4 * t))),
        (lambda t: z_lo(t) + (z_hi(t) - z_lo(t)) / 3, lambda t: bul * (1 + 0.6 * math.sin(3 * t + 0.5))),
        (lambda t: z_lo(t) + 2 * (z_hi(t) - z_lo(t)) / 3, lambda t: bul * (1 + 0.6 * math.sin(3 * t + 2.0))),
        (z_hi, lambda t: -fl * (1 + 0.5 * math.cos(5 * t)) - cfg["draft"]),
    ]
    W = closed_surface(cfg["n"], outer_fp, orows, True, cfg["phase"])
    msamp = cfg["m"] or cfg["n"] * 2
    rb = trim_ring(W, lambda p: p[2] - 0.0, msamp)
    rt = trim_ring(W, lambda p: p[2] - H, msamp, cfg["oskew_t"])
    lo, hi = [1e9] * 3, [-1e9] * 3
    u0_, u1_ = W.u_range()
    for k in range(600):
        u = u0_ + (u1_ - u0_) * k / 600
        vb_ = bisect(lambda v: W(u, v)[2], 0.0, 1.0, 50)
        vt_ = bisect(lambda v: W(u, v)[2] - H, 0.0, 1.0, 50)
        for j in range(13):
            p = W(u, vb_ + (vt_ - vb_) * j / 12)
            lo = [min(a, b) for a, b in zip(lo, p)]
            hi = [max(a, b) for a, b in zip(hi, p)]
    log.append("part size (mm): " + " x ".join(f"{(h - l) * cfg['scale']:.2f}" for l, h in zip(lo, hi)))
    log.append(f"outer wall: {len(W.grid)}x{len(W.grid[0])} control points, closed in U; ring fit deviation {rb[2]:.1e} / {rt[2]:.1e} mm")
    fo, orb, ort = wall_face(W, rb, rt)
    faces = [fo]

    bot = st.plane(S((0, 0, 0)), (0, 0, -1), (1, 0, 0))
    faces.append(st.face([st.loop([(orb, False)])], bot, True))
    top_loops = [st.loop([(ort, True)])]

    pa, pb, pd = cfg["pa"], cfg["pb"], cfg["pd"]
    px, py = cfg["px"], cfg["py"]

    def pocket_fp(t):
        x, y = superellipse(pa, pb, cfg["pe"], t)
        r = 1.0 + cfg["pwave"] * math.sin(cfg["pwave_n"] * t + 1.3)
        return x * r + px, y * r + py

    # Pocket floor: z = f(x, y) as a clamped cubic B-spline over a rectangle wider than the
    # pocket (control points at the Greville abscissae, so x and y are linear in s and t).
    zf = H - pd
    amp = cfg["famp"]
    fx0, fx1 = px - pa * 1.35 - 10, px + pa * 1.35 + 10
    fy0, fy1 = py - pb * 1.35 - 10, py + pb * 1.35 + 10
    ns = cfg["fn"]
    spans = ns - 3
    fk = [0.0] * 4 + [float(i) / spans for i in range(1, spans)] + [1.0] * 4
    grev = [(fk[i + 1] + fk[i + 2] + fk[i + 3]) / 3 for i in range(ns)]

    def fz(x, y):
        a = (x - px) / pa
        b = (y - py) / pb
        return zf + amp * (0.6 * math.sin(2.1 * a + 0.3) * math.cos(1.7 * b) + 0.4 * math.cos(3.3 * a * b + 0.5)) \
            - cfg["fdish"] * (1 - min(1.5, a * a + b * b))

    fgrid = []
    for i in range(ns):
        x = fx0 + (fx1 - fx0) * grev[i]
        fgrid.append([(x, fy0 + (fy1 - fy0) * grev[j], fz(x, fy0 + (fy1 - fy0) * grev[j])) for j in range(ns)])
    F = Surf(fk, fk, fgrid)

    def floor_h(x, y):
        s = (x - fx0) / (fx1 - fx0)
        t = (y - fy0) / (fy1 - fy0)
        return F(min(max(s, 0), 1), min(max(t, 0), 1))[2]

    zmin = min(p[2] for c in fgrid for p in c)
    plo = lambda t: zmin - cfg["pext_b"]
    phi = lambda t: H + cfg["pext_t"] * (1 + zw * math.sin(2 * t))
    prows = [
        (plo, lambda t: -cfg["pflare"]),
        (lambda t: plo(t) + (phi(t) - plo(t)) / 3, lambda t: cfg["pbulge"] * (1 + 0.5 * math.cos(2 * t))),
        (lambda t: plo(t) + 2 * (phi(t) - plo(t)) / 3, lambda t: cfg["pbulge"] * (1 + 0.5 * math.sin(2 * t))),
        (phi, lambda t: cfg["pflare"]),
    ]
    P = closed_surface(cfg["pn"], pocket_fp, prows, False, cfg["pphase"])
    pm = cfg["pm"] or cfg["pn"] * 2
    prb = trim_ring(P, lambda p: p[2] - floor_h(p[0], p[1]), pm)
    prt = trim_ring(P, lambda p: p[2] - H, pm)
    log.append(f"pocket wall: {len(P.grid)}x{len(P.grid[0])} control points, closed in U; ring fit deviation {prb[2]:.1e} / {prt[2]:.1e} mm")
    log.append(f"pocket floor: {ns}x{ns} control points, open")
    fp_, prb_e, prt_e = wall_face(P, prb, prt)
    faces.append(fp_)
    top_loops.append(st.loop([(prt_e, True)]))
    fsurf = st.bspline_surface(3, 3, F.ku, F.kv, [[S(p) for p in c] for c in fgrid], False, False)
    faces.append(st.face([st.loop([(prb_e, False)])], fsurf, True))

    top = st.plane(S((0, 0, H)), (0, 0, 1), (1, 0, 0))
    faces.append(st.face(top_loops, top, True))
    return st.text(faces, cfg["name"], cfg["inch"]), log


# Model units are mm before `scale`; the file is written in inches.
DEFAULTS = dict(
    name="freeform_panel", inch=True, scale=1.0,
    # outer footprint: super-ellipse a x b, exponent e, with a wave of relative amplitude
    # `wave` and `wave_n` lobes; n periodic control columns; seam at angle `phase`
    a=150.0, b=135.0, h=44.0, e=5.0, wave=0.1, wave_n=6, n=120, m=0, phase=0.487,
    # outer wall surface: runs ext_b below z=0 and ext_t above the top; flare/draft offsets
    ext_b=6.0, ext_t=8.0, bulge=0.0, flare=10.0, draft=2.0, zwave=0.8,
    # top ring sample skew: its parameter midpoint lands 0.3 um beside the wall's mid-U line
    oskew_t=3e-7,
    # pocket: super-ellipse pa x pb (exponent pe) at (px, py), depth pd, pn periodic columns
    pa=80.0, pb=60.0, pd=22.0, pe=3.0, pwave=0.05, pwave_n=3, pn=59, pm=0,
    pbulge=2.0, pflare=4.0, pext_b=4.0, pext_t=6.0, px=10.0, py=5.0, pphase=0.0,
    # pocket floor: fn x fn control points, wave amplitude famp, dish depth fdish
    fn=24, famp=4.0, fdish=3.0,
)

def parse(argv):
    cfg = dict(DEFAULTS)
    for a in argv:
        k, v = a.split("=", 1)
        d = DEFAULTS[k]
        if isinstance(d, bool):
            cfg[k] = v.lower() in ("1", "true", "yes", "t")
        elif isinstance(d, int):
            cfg[k] = int(v)
        elif isinstance(d, float):
            cfg[k] = float(v)
        else:
            cfg[k] = v
    return cfg


if __name__ == "__main__":
    args = [a for a in sys.argv[1:] if "=" not in a]
    cfg = parse([a for a in sys.argv[1:] if "=" in a])
    out = args[0] if args else os.path.join(os.path.dirname(os.path.abspath(__file__)), cfg["name"] + ".step")
    cfg["name"] = os.path.splitext(os.path.basename(out))[0]
    text, log = build(cfg)
    with open(out, "w", newline="\n") as fh:
        fh.write(text)
    print(out)
    print("\n".join(log))
