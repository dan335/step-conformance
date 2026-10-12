# step-conformance

How well does a STEP importer read real parts?

Most new CAD kernels say they import STEP and mesh it to a stated tolerance. This
checks. It reads a shared set of parts with each importer, at several tolerances,
and scores the result against two things that depend on no one importer:

- **What the file itself says:** its unit, its faces, where its B-rep vertices are, a
  box its geometry cannot leave, and where its assembly puts each part. These are
  read from the file's text by
  [step-check](https://github.com/dan335/camcraft/tree/main/crates/step-check).
- **The true surface:** exact for the generated parts, which know their own shape,
  and OpenCascade's mesh at 0.001 mm for the rest.

It also times each importer, and measures its memory, on the parts it reads right.

On the parts here, every reader but OpenCascade puts surface millimetres to a metre
off a part somewhere: a cone that comes to a point, a B-spline wall that wraps all
the way round, a half-torus, a revolved rim.

## Results

See [results/RESULTS.md](results/RESULTS.md) for every part at every tolerance.

12 October 2026: 27 parts at four tolerances each: 7 shop parts, the freeform panel,
and 19 parts of known shape, three of them big, to time readers on.

| reader | shop parts | freeform panel | parts of known shape | version |
|---|---|---|---|---|
| OpenCascade | 28 of 28 | 4 of 4 | 76 of 76 | opencascade-rs 0.3.0 with camcraft's patches, OCCT 7.8.1 |
| [solidcraft](https://github.com/bherbruck/solidcraft) | 28 of 28 | 1 of 4 | 63 of 76 | f0292b1 |
| [ogeom](https://github.com/gilbertorconde/ogeom-rs) | 28 of 28 | 1 of 4 | 58 of 76 | 0.9.16 |
| [truck](https://github.com/ricosjp/truck), master | 23 of 28 | 0 of 4 | 68 of 76 | 88ed005 |
| truck, released | 21 of 28 | 0 of 4 | 60 of 76 | truck-stepio 0.3.0, truck-meshalgo 0.4.0 |
| [brepkit](https://github.com/andymai/brepkit) | 20 of 28 | 0 of 4 | 30 of 76 | 4.1.49 |

Three readers that pass every shop part fail generated ones, which is what the
generated parts are for:

- The freeform panel defeats every reader but OpenCascade: solidcraft puts surface
  5 to 149 mm off it, ogeom 5.5 mm, truck 12 mm, brepkit 14 mm.
- Every reader but OpenCascade fails a cone that comes to a point; most put surface
  30 mm off it, with or without a vertex at the apex.
- ogeom loses B-spline faces that wrap all the way round with no seam edge (the
  cylinder, dome and torus) and puts the plain dome 20 mm off.
- solidcraft puts the plain half-torus 5 mm off.
- Released truck ignores assembly placements, and on the bowl times out or lands
  53 mm off; master mends most of that. It also lands `sled_base` 0.069 mm off at
  0.0125 mm on x86, which it passed on an Apple M2 Pro.
- brepkit misreads cylinders written as Fusion writes them, and ignores units and
  placements.

Every reader reads the three big parts right, at every tolerance.

### Speed and memory

On an idle Intel Core i7-12700KF, under Linux in WSL2, against OpenCascade, on the
runs both pass, averaged geometrically: 0.5× is twice as fast, or half the memory.

| reader | runs both pass | time | CPU time | reading | meshing | peak memory | triangles |
|---|---|---|---|---|---|---|---|
| solidcraft | 92 | 1.1× | 2.1× | 1.3× | 1.1× | 0.42× | 1.4× |
| ogeom | 87 | 0.47× | 0.81× | 0.74× | 0.25× | 0.37× | 1.3× |
| truck, master | 91 | 0.40× | 0.68× | 0.21× | 0.49× | 0.32× | 0.64× |
| truck, released | 81 | 0.41× | 0.72× | 0.20× | 0.51× | 0.31× | 0.70× |
| brepkit | 50 | 0.26× | 0.39× | 0.031× | 0.50× | 0.27× | 0.87× |

Most of those parts are small, so starting the program counts for much. On Linux
OpenCascade starts quickly, in under 2 ms of its 5.6 on the cylinder; on the M2
Pro it took 12 ms of 15, and every reader beat it there, solidcraft included. The
big plate shows what the small parts cannot. At 0.0125 mm:

| reader | `plate_100_holes` | `plate_900_holes` | growth | peak memory on `plate_900_holes` |
|---|---|---|---|---|
| OpenCascade | 308 ms | 14.8 s | 48× | 1000 MB |
| solidcraft | 453 ms | 19.3 s | 43× | 535 MB |
| ogeom | 99 ms | 879 ms | 9× | 148 MB |
| truck, master | 118 ms | 5.66 s | 48× | 106 MB |
| truck, released | 114 ms | 5.26 s | 46× | 98 MB |
| brepkit | 61 ms | 2.13 s | 35× | 102 MB |

- Every reader but ogeom takes 35 to 48 times as long on a part nine times the size,
  nearly all of it meshing. ogeom takes nine times as long, and meshes the 900 holes
  in a third of a second.
- OpenCascade holds a gigabyte for the plate, and spends 1.7 s reading the 1.4 MB
  file, which brepkit reads in 15 ms.
- On the M2 Pro, with 1600 holes, solidcraft took 57 s and OpenCascade 33 s, so the
  plate stops at 900: past 60 s a reader fails.
- The grooved shaft's 753 faces take every reader under 0.8 s.

## How a reader is scored

Each part is read at 0.0125, 0.05, 0.1 and 0.5 mm. At each one:

1. **The file checks.** At least as many faces as the file defines, none of them
   empty; no part of the mesh outside the file's own geometry; the mesh reaching
   every vertex the file defines.
2. **The distance to the true surface, both ways.** How far the reader's mesh lies
   from it, measured at every vertex and every triangle's centre, and how far it lies
   from the mesh, which finds missing surface. For a generated part both are exact;
   for the rest, the true surface is OpenCascade's mesh at 0.001 mm.

A reader passes a part at a tolerance when the checks pass and neither distance is
more than four times the tolerance. Four, because OpenCascade's own mesh reaches 3.4
times its tolerance on `bowl_01` at 0.0125 mm, measured against its finer one. A
reader that takes more than 60 seconds fails.

Two limits to keep in mind. On the shop parts the reference is OpenCascade, not the
true surface: a reader that is right where OpenCascade is wrong is scored as wrong
there, so a failure on one of those is worth a look rather than taken on trust. And
distances are measured at points, not everywhere between them.

## How a reader is timed

Only a run that passes is timed: a reader that gives up fast is not fast. Each one is
run again, up to five times or for ten seconds in all, and the median taken of:

- **time**, from starting the reader to its exit: everything a user would wait for,
  starting the program and writing the mesh included.
- **CPU time**, user and system, across all its threads. More CPU time than time
  means the reader works in parallel.
- **reading** and **meshing**, the reader's own account of them, for a reader that
  gives one. Every reader here does; see [Adding a reader](#adding-a-reader).
- **peak memory**, the most the reader held in RAM at once, as the operating system
  reports it once the reader exits (on macOS and Linux). Each reader is started by a
  small relay process, since Linux would otherwise charge it the scorer's own peak.

The readers are then compared with OpenCascade on the runs both pass: each figure is
the reader's divided by OpenCascade's, averaged geometrically, so 0.5× is twice as
fast, or half the memory. Most parts here are small, where starting the program is
much of the time, so `plate_100_holes`, `plate_900_holes` and `grooved_shaft` are big
on purpose. They show how a reader's time grows with a part, which a small part
cannot. The triangle count is beside each, since a reader that makes fewer triangles
has less to do.

The figures are from one machine, so they compare readers with each other rather
than promise a speed on yours. Readers run one at a time, but anything else running
slows them, and unevenly: most of all a reader that meshes in parallel, as ogeom
does. Time on an idle machine.

## Adding a reader

A reader is any program, in any language, run as

```text
PROGRAM OUT TOLERANCE FILE
```

It reads the STEP file `FILE`, meshes it so no triangle strays farther than
`TOLERANCE` millimetres from the true surface, and writes `OUT/<stem>.bin`, where
`<stem>` is the file's name without its extension. All numbers little-endian:

| what | type |
|---|---|
| vertex count | u64 |
| each vertex: x, y, z in millimetres | f64 × 3 |
| triangle count | u64 |
| each triangle: three vertex indices | u32 × 3 |
| face count | u64 |
| each face: how many triangles it has | u64 |

Triangles are grouped by B-rep face, in the reader's order of faces: all of face 0's
triangles, then face 1's, and so on. Every face appears, even one with no triangles;
that is how a face given up on is caught. If the reader cannot read or mesh the file,
it writes the reason to `OUT/<stem>.err` instead. Either way it exits with 0.

A reader may also write how long it spent reading the file into its own model, and
meshing that, to `OUT/<stem>.time`, one phase a line, in seconds:

```text
read 0.0421
mesh 0.0034
```

It is optional; without it, a reader is timed from outside only.

A reader that ignores the file's length unit is given the tolerance in that unit and
writes in it; mark it `:file-units` in `run.sh`, and the runner scales it.

[adapters/](adapters/) holds the readers so far, each a page or two of Rust. Add
yours there, with a build step in `run.sh`, and send a pull request.

## The parts

[corpus/](corpus/), free for anyone to use under [CC0](corpus/LICENSE): shop parts,
and parts generated to break readers in known ways.

### Generated parts

[corpus/generated/](corpus/generated/) holds parts whose exact shape is known, written
as STEP in the dialect the test needs, beside a `.truth.json` giving that shape. Each
is solids of revolution, a profile of segments and arcs turned about an axis, or a
prism, a flat profile of segments and arcs, with holes, swept along one. The nearest
point of a surface of revolution lies in the half-plane through the axis and the
point, and the nearest point of a prism's wall lies over the nearest point of its
profile, so the runner measures both exactly.
[generate/](generate/) writes them, the same every run:

```sh
cargo run --manifest-path generate/Cargo.toml -- corpus/generated
```

| part | what it tests |
|---|---|
| `cylinder` | a face wrapping all the way round, bounded only by two circles with no seam edge: how Fusion writes every cylinder |
| `cylinder_seam` | the same with a seam edge, as OpenCascade writes it: the control |
| `cylinder_nurbs` | the side as the rational B-spline that is exactly a cylinder, closed round, no seam |
| `cone_point` | a cone coming to a point that no vertex marks |
| `cone_point_vertex` | the same with a vertex loop at the apex: the control |
| `dome` | a hemisphere on a disc: the pole inside the face |
| `dome_nurbs` | the dome as a rational B-spline, a whole row of control points at the pole |
| `washer` | flat faces with holes, and an inner cylinder whose normal points at the axis |
| `torus_ring` | half a torus, doubly curved, bounded by its inner and outer equators |
| `torus_ring_nurbs` | the same as a rational B-spline |
| `cylinder_inch` | a part in inches |
| `sheet_disc` | 3.6 m across and 19 mm thick, about a 12 ft sheet |
| `tiny_pin` | half a millimetre across |
| `far_cylinder` | 100 m from the origin |
| `cylinder_placed` | a component in an assembly, tipped over and moved |
| `cylinder_twice` | a component used twice, once turned on its side |
| `plate_100_holes` | a plate 100 mm square with 100 holes, its corners rounded: one solid of 110 faces |
| `plate_900_holes` | the plate 300 mm square with 900 holes, like a router's spoilboard: 910 faces, two of them holding 900 holes each |
| `grooved_shaft` | a shaft half a metre long ringed by 250 grooves: 753 faces, each wrapping all the way round |

The last three are big, to time readers on, and are written with seam edges, as
OpenCascade writes them, so that every reader here reads them.

### The freeform panel

`freeform_panel.step` stands in for a customer part that cannot be published, and
breaks readers the same way: a wavy super-ellipse body whose outer and pocket walls are
cubic B-spline surfaces closed round, with no seam edge of their own, trimmed by
fitted ring curves that lie a few ten-thousandths of a millimetre off them, as CAD
exports' do. [generate/freeform_panel.py](generate/freeform_panel.py) writes it from
formulas alone. Readers that search for each ring point on the wall from the point
before it can step off the wall's domain at a concave bend at coarse tolerances, and
the mesh follows. Its failures are sensitive to the exact geometry, so the file is
committed and should not be regenerated in place.

### Shop parts

| file | unit | faces | surfaces | size (mm) | what it exercises |
|---|---|---|---|---|---|
| `hinge_jig.step` | inch | 39 | planar | 25.4×31.8×33.3 | the smallest sane case |
| `lego_4x2_01.step` | inch | 76 | plane, cone | 100.6×49.8×38.1 | cylinders and fillets |
| `door_handle_guide.step` | inch | 89 | plane, cone | 57.1×215.9×33.0 | a real router jig |
| `rail_guide.step` | inch | 134 | plane, cone | 237.0×40.6×12.7 | many faces, all analytic |
| `lego_3_pins.step` | inch | 234 | plane, cone | 135.9×164.1×47.6 | many small round faces |
| `sled_base.step` | inch | 387 | plane, cone, 6 B-spline | 237.0×227.0×76.2 | mixed analytic and NURBS |
| `bowl_01.step` | mm | 5 | 2 surfaces of revolution, seam curves | 380.0×380.0×20.0 | revolved geometry and seams |

All but `bowl_01` are Fusion exports, which write no curves in surface parameters:
a reader has to find each edge on its surface itself. They come from
[camcraft](https://github.com/dan335/camcraft)'s test set.

Parts that break a reader are the most useful thing to add. A part has to be yours
to give away under CC0. When the part that shows a bug cannot be shared, a generated
part that breaks a reader the same way is the next best thing.

## Running it

```sh
./run.sh                     # every reader, on corpus/, writing results/RESULTS.md
./run.sh ../more-parts       # those parts too, printed rather than recorded
SOLIDCRAFT_REV=07e263d TRUCK_REV=<sha> ./run.sh
```

It needs Rust, git, cmake and a C++ compiler; OpenCascade is built from source the
first time, which takes several minutes. Clones and builds go under `work/`. With
everything built, a whole run takes about a quarter of an hour.

## Licence

The code is MIT or Apache-2.0, at your option. The parts in `corpus/` are CC0.
`adapters/brepkit` builds a program that links brepkit, which is AGPL-3.0, so that
program is AGPL-3.0 too; `run.sh` builds and runs it locally and never distributes
it.
