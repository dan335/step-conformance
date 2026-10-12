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

On the parts here, released truck puts surface 53 mm off the bowl. On a part that
cannot be published, solidcraft puts surface 108 mm off a B-spline face that wraps all
the way round the part, and truck more than a metre; a public part that does the
same is being made.

## Results

See [results/RESULTS.md](results/RESULTS.md) for every part at every tolerance.

12 October 2026: 23 parts at four tolerances each, 7 shop parts and 16 generated.

| reader | shop parts | generated parts | version |
|---|---|---|---|
| OpenCascade | 28 of 28 | 64 of 64 | opencascade-rs 0.3.0 with camcraft's patches, OCCT 7.8.1 |
| [solidcraft](https://github.com/bherbruck/solidcraft) | 28 of 28 | 51 of 64 | 113ded5 |
| [ogeom](https://github.com/gilbertorconde/ogeom-rs) | 28 of 28 | 46 of 64 | 0.9.16 |
| [truck](https://github.com/ricosjp/truck), master | 23 of 28 | 56 of 64 | 88ed005 |
| truck, released | 22 of 28 | 48 of 64 | truck-stepio 0.3.0, truck-meshalgo 0.4.0 |
| [brepkit](https://github.com/andymai/brepkit) | 20 of 28 | 18 of 64 | 4.1.49 |

Three readers that pass every shop part fail generated ones, which is what the
generated parts are for:

- Every reader but OpenCascade fails a cone that comes to a point; most put surface
  30 mm off it, with or without a vertex at the apex.
- ogeom loses B-spline faces that wrap all the way round with no seam edge (the
  cylinder, dome and torus) and puts the plain dome 20 mm off.
- solidcraft puts the plain half-torus 5 mm off.
- Released truck ignores assembly placements, and on the bowl times out or lands
  53 mm off; master mends most of that.
- brepkit misreads cylinders written as Fusion writes them, and ignores units and
  placements.

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

A reader that ignores the file's length unit is given the tolerance in that unit and
writes in it; mark it `:file-units` in `run.sh`, and the runner scales it.

[adapters/](adapters/) holds the readers so far, each a page or two of Rust. Add
yours there, with a build step in `run.sh`, and send a pull request.

## The parts

[corpus/](corpus/), free for anyone to use under [CC0](corpus/LICENSE): shop parts,
and parts generated to break readers in known ways.

### Generated parts

[corpus/generated/](corpus/generated/) holds parts whose exact shape is known: each is
one or more solids of revolution, a profile of segments and arcs turned about an
axis, written as STEP in the dialect the test needs, beside a `.truth.json` giving
that shape. The nearest point of such a surface lies in the half-plane through the
axis and the point, so the runner measures them exactly.
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
first time, which takes several minutes. Clones and builds go under `work/`.

## Licence

The code is MIT or Apache-2.0, at your option. The parts in `corpus/` are CC0.
`adapters/brepkit` builds a program that links brepkit, which is AGPL-3.0, so that
program is AGPL-3.0 too; `run.sh` builds and runs it locally and never distributes
it.
