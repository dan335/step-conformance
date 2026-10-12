//! truck's git master as a step-conformance reader: truck-stepio reads the
//! file and truck-meshalgo's `robust_triangulation` meshes it, as the crates.io
//! reader (`adapters/truck`) does, so the two differ only in
//! truck's version.
//!
//!   truck-master-reader OUT TOLERANCE FILE
//!
//! Writes `OUT/<stem>.bin` in the format the README describes, or why not to
//! `OUT/<stem>.err`, and what it chose along the way to `OUT/<stem>.log`.
//!
//! Master still ignores the file's unit, so the tolerance is in that unit and so
//! is the output; run it with `:file-units`.
//!
//! What master adds over the release is `Table::step_assy`, the product tree with
//! its placements. It is used when it reaches every face the file's shells hold.
//! It reads a product's geometry only from the items of the representation the
//! product names, though, and Fusion and most other exporters hang the solid one
//! representation further on (through a `SHAPE_REPRESENTATION_RELATIONSHIP` to
//! an `ADVANCED_BREP_SHAPE_REPRESENTATION`). There it finds no geometry, and this
//! falls back to the release reader's walk: every shell in the file, by id,
//! unplaced.

use std::path::Path;

use truck_meshalgo::prelude::*;
use truck_stepio::r#in::Table;
use truck_stepio::r#in::convert::ProductShape;
use truck_stepio::r#in::step_geometry::{Curve3D, Surface};
use truck_topology::compress::CompressedShell;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [out, tolerance, file] = &args[..] else {
        eprintln!("usage: truck-master-reader OUT TOLERANCE FILE");
        std::process::exit(2);
    };
    let out = Path::new(out);
    let stem = Path::new(file)
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();
    let (bin, err, log) = (
        out.join(format!("{stem}.bin")),
        out.join(format!("{stem}.err")),
        out.join(format!("{stem}.log")),
    );
    // The trial reads the .err before the .bin, and stops altogether when it finds
    // neither, so one is there from the start: a reader that aborts or overflows its
    // stack still leaves a reason. A panic rewrites it with the panic's message.
    let _ = std::fs::write(
        &err,
        "the reader died before finishing (an abort or a signal)",
    );
    let on_panic = err.clone();
    std::panic::set_hook(Box::new(move |info| {
        let _ = std::fs::write(&on_panic, format!("panicked: {info}"));
    }));

    let mut notes = Vec::new();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        tolerance
            .parse::<f64>()
            .map_err(|e| e.to_string())
            .and_then(|tolerance| mesh(file, tolerance, &mut notes))
    }));
    let _ = std::fs::write(&log, notes.join("\n"));
    let written = match result {
        Ok(Ok(encoded)) => std::fs::write(&bin, encoded).and_then(|()| std::fs::remove_file(&err)),
        Ok(Err(why)) => std::fs::write(&err, why),
        Err(_) => Ok(()), // the hook has written the panic
    };
    if let Err(e) = written {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

/// A shell ready to mesh, and the placement that takes it into the file's frame.
type Placed = (CompressedShell<Point3, Curve3D, Surface>, Matrix4);

fn mesh(file: &str, tolerance: f64, notes: &mut Vec<String>) -> Result<Vec<u8>, String> {
    let text = std::fs::read_to_string(file).map_err(|e| e.to_string())?;
    let table = Table::from_step(&text).ok_or("truck-stepio could not parse the file")?;

    // The release reader's walk: every shell, by id, as the file states it.
    let mut ids: Vec<_> = table.shell.keys().copied().collect();
    ids.sort_unstable();
    let mut flat: Vec<Placed> = Vec::new();
    for id in ids {
        let shell = table
            .to_compressed_shell(&table.shell[&id])
            .map_err(|e| format!("shell #{id}: {e:?}"))?;
        flat.push((shell, Matrix4::identity()));
    }

    let shells = match placed_shells(&table) {
        Ok(placed) if face_count(&placed) >= face_count(&flat) => {
            notes.push(format!(
                "placed by step_assy: {} shells, {} faces",
                placed.len(),
                face_count(&placed)
            ));
            placed
        }
        Ok(placed) => {
            notes.push(format!(
                "step_assy reached {} of the shells' {} faces; every shell unplaced instead",
                face_count(&placed),
                face_count(&flat)
            ));
            flat
        }
        Err(why) => {
            notes.push(format!(
                "step_assy refused ({why}); every shell unplaced instead"
            ));
            flat
        }
    };

    let mut mesh = Soup::default();
    for (shell, matrix) in &shells {
        // A placement that mirrors turns every triangle inside out.
        let mirrored = matrix.determinant() < 0.0;
        for face in &shell.robust_triangulation(tolerance).faces {
            let mut n = 0u64;
            if let Some(polygon) = &face.surface {
                let positions = polygon.positions();
                for tri in polygon.faces().triangle_iter() {
                    let mut corners = [0u32; 3];
                    for (slot, v) in corners.iter_mut().zip(tri.iter()) {
                        let p = matrix * positions[v.pos].to_homogeneous();
                        mesh.vertices.push([p.x / p.w, p.y / p.w, p.z / p.w]);
                        *slot = (mesh.vertices.len() - 1) as u32;
                    }
                    if face.orientation == mirrored {
                        corners.swap(1, 2);
                    }
                    mesh.triangles.push(corners);
                    n += 1;
                }
            } else {
                notes.push(format!("face {} did not mesh", mesh.faces.len()));
            }
            mesh.faces.push(n);
        }
    }
    Ok(mesh.encode())
}

/// Every shell the product tree reaches, once per path to it, with the product of
/// the placements along that path.
fn placed_shells(table: &Table) -> Result<Vec<Placed>, String> {
    let assy = table.step_assy().map_err(|e| format!("{e:?}"))?;
    let mut out = Vec::new();
    for top in assy.top_nodes() {
        for path in assy.paths_iter(top.index()) {
            let mut matrix = Matrix4::identity();
            for edge in path.edges() {
                let step =
                    Matrix4::try_from(&edge.entity().matrix).map_err(|e| format!("{e:?}"))?;
                matrix = matrix * step;
            }
            let Some(node) = path.nodes().last() else {
                continue;
            };
            for shape in &node.entity().shape {
                let shells = match shape {
                    ProductShape::Solid(solid) => &solid.boundaries,
                    ProductShape::Shells(shells) => shells,
                    ProductShape::Matrix(_) => continue,
                };
                out.extend(shells.iter().map(|s| (s.clone(), matrix)));
            }
        }
    }
    Ok(out)
}

fn face_count(shells: &[Placed]) -> usize {
    shells.iter().map(|(s, _)| s.faces.len()).sum()
}

/// The trial's format: vertices, triangles, then triangles per face.
#[derive(Default)]
struct Soup {
    vertices: Vec<[f64; 3]>,
    triangles: Vec<[u32; 3]>,
    faces: Vec<u64>,
}

impl Soup {
    fn encode(&self) -> Vec<u8> {
        let mut b = Vec::new();
        b.extend((self.vertices.len() as u64).to_le_bytes());
        self.vertices
            .iter()
            .flatten()
            .for_each(|c| b.extend(c.to_le_bytes()));
        b.extend((self.triangles.len() as u64).to_le_bytes());
        self.triangles
            .iter()
            .flatten()
            .for_each(|c| b.extend(c.to_le_bytes()));
        b.extend((self.faces.len() as u64).to_le_bytes());
        self.faces.iter().for_each(|n| b.extend(n.to_le_bytes()));
        b
    }
}
