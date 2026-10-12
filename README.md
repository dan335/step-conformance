# step-conformance

How well does a STEP importer read real parts?

Most new CAD kernels say they import STEP and mesh it to a stated tolerance. This
checks. It reads a shared set of parts with each importer, at several tolerances,
and scores the result against two things that depend on no one importer:

- **What the file itself says:** its unit, its faces, where its B-rep vertices are, a
  box its geometry cannot leave, and where its assembly puts each part. These are
  read from the file's text by
  [step-check](https://github.com/dan335/camcraft/tree/main/crates/step-check).
- **A far finer mesh of the same file:** OpenCascade's, at 0.001 mm.

On the parts here, released truck puts surface 53 mm off the bowl. On a part that
cannot be published, solidcraft puts surface 108 mm off a B-spline face that wraps all
the way round the part, and truck more than a metre; a public part that does the
same is being made.

## Results

See [results/RESULTS.md](results/RESULTS.md) for every part at every tolerance.

12 October 2026, on the seven parts below at four tolerances each:

| reader | passes | version |
|---|---|---|
| OpenCascade | 28 of 28 | opencascade-rs 0.3.0 with camcraft's patches, OCCT 7.8.1 |
| [solidcraft](https://github.com/bherbruck/solidcraft) | 28 of 28 | b4491ac |
| [truck](https://github.com/ricosjp/truck) | 22 of 28 | truck-stepio 0.3.0, truck-meshalgo 0.4.0 (crates.io) |

## How a reader is scored

Each part is read at 0.0125, 0.05, 0.1 and 0.5 mm. At each one:

1. **The file checks.** At least as many faces as the file defines, none of them
   empty; no part of the mesh outside the file's own geometry; the mesh reaching
   every vertex the file defines.
2. **The distance to the reference, both ways.** How far the reader's mesh lies from
   the reference surface, and how far the reference lies from it, which finds missing
   surface. Both are measured at every vertex and every triangle's centre.

A reader passes a part at a tolerance when the checks pass and neither distance is
more than four times the tolerance. Four, because OpenCascade's own mesh reaches 3.4
times its tolerance on `bowl_01` at 0.0125 mm, measured against its finer one. A
reader that takes more than 60 seconds fails.

Two limits to keep in mind. The reference is OpenCascade, not the true surface: a
reader that is right where OpenCascade is wrong is scored as wrong, so a failure is
worth a look rather than taken on trust. And distances are measured at points, not
everywhere between them.

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

[corpus/](corpus/), free for anyone to use under [CC0](corpus/LICENSE).

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
