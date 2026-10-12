//! ogeom (https://github.com/gilbertorconde/ogeom-rs) as a step-conformance
//! reader: `ogeom_io::read_step` reads the file into a document (the model plus
//! the file's product tree), and `ogeom_mesh::face_meshes_for` meshes each placed
//! part face by face, every face drawn to the chords its neighbours agreed on, so
//! faces meet along their shared edges.
//!
//!   ogeom-reader OUT TOLERANCE_MM FILE
//!
//! Writes `OUT/<stem>.bin` in the format the README describes, in millimetres, or why not
//! to `OUT/<stem>.err`, and what the import reported to `OUT/<stem>.log`.
//!
//! ogeom scales the file to millimetres as it reads it, and places every part of
//! an assembly where the tree puts it. A face it will not mesh is written with no
//! triangles, which the runner reports as an empty face; the log says why.

use std::collections::HashSet;
use std::path::Path;

use ogeom_core::Tolerances;
use ogeom_io::read_step;
use ogeom_mesh::{Deflection, face_meshes_for};
use ogeom_topo::{Filter, Model, Shape, ShapeType, explore};

/// The kernel's tolerances for a model in millimetres, which is what the reader
/// builds whatever the file's unit.
const TOL: Tolerances = Tolerances::millimetres();

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [out, tolerance, file] = &args[..] else {
        eprintln!("usage: ogeom-reader OUT TOLERANCE_MM FILE");
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

fn mesh(file: &str, tolerance: f64, notes: &mut Vec<String>) -> Result<Vec<u8>, String> {
    let bytes = std::fs::read(file).map_err(|e| e.to_string())?;
    let import = read_step(&String::from_utf8_lossy(&bytes), TOL).map_err(|e| e.to_string())?;
    let report = &import.report;
    notes.push(format!(
        "{} solids, {} shells; {} mm per file unit; {} warnings",
        import.solids.len(),
        import.shells.len(),
        report.scale_mm,
        report.warnings.len()
    ));
    for w in &report.summary {
        notes.push(format!(
            "  {} x{} (worst {:.3e}, e.g. #{})",
            w.kind, w.count, w.worst, w.exemplar
        ));
    }
    for f in &report.untrimmed_faces {
        notes.push(format!("  untrimmed face #{}", f.entity));
    }

    let document = &import.document;
    let model = document.model();

    // Every part where the product tree places it, then anything the tree did not
    // reach (a body no product claims), as the file states it.
    let mut bodies: Vec<Shape> = Vec::new();
    for root in document.roots() {
        for occurrence in document.occurrences_of(root).map_err(|e| e.to_string())? {
            bodies.push(occurrence.shape);
        }
    }
    let mut reached = HashSet::new();
    for body in &bodies {
        reached.extend(faces_of(model, body)?.iter().map(Shape::node));
    }
    for body in import.solids.iter().chain(&import.shells) {
        if faces_of(model, body)?
            .iter()
            .any(|f| !reached.contains(&f.node()))
        {
            notes.push("a body the product tree does not reach, unplaced".to_string());
            bodies.push(body.clone());
        }
    }

    let deflection = Deflection {
        chord: tolerance,
        ..Deflection::default()
    };
    let mut soup = Soup::default();
    for body in &bodies {
        let (faces, _) =
            face_meshes_for(model, body, deflection, None, TOL).map_err(|e| e.to_string())?;
        for (_, piece) in faces {
            let mesh = match piece {
                Ok(mesh) => mesh,
                Err(why) => {
                    notes.push(format!("face {} would not mesh: {why}", soup.faces.len()));
                    soup.faces.push(0);
                    continue;
                }
            };
            if !mesh.deflection_met {
                notes.push(format!("face {} missed the deflection", soup.faces.len()));
            }
            let base = soup.vertices.len() as u32;
            soup.vertices
                .extend(mesh.positions.iter().map(|p| [p.x, p.y, p.z]));
            soup.triangles
                .extend(mesh.triangles.iter().map(|t| t.map(|i| i + base)));
            soup.faces.push(mesh.triangles.len() as u64);
        }
    }
    Ok(soup.encode())
}

fn faces_of(model: &Model, shape: &Shape) -> Result<Vec<Shape>, String> {
    explore(model, shape, Filter::OfType(ShapeType::Face)).map_err(|e| e.to_string())
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
