//! solidcraft's STEP reader as a step-conformance reader. `run.sh` copies this into a
//! solidcraft checkout as `crates/kernel/examples/camcraft_export.rs` and builds it.
//! solidcraft (https://github.com/tangency-labs/solidcraft) reads STEP with its own
//! reader (`solidcraft-step`), builds each solid in solidcore (its Rust port of SMLib),
//! and meshes it with its own mesher at a chord tolerance: no facet or edge segment
//! farther than that from the exact surface or curve.
//!
//!   camcraft_export OUT TOLERANCE_MM FILE
//!
//! Writes `OUT/<stem>.bin` in the format the README describes, in millimetres, or
//! `OUT/<stem>.err`; how long reading and meshing took to `OUT/<stem>.time`; and the
//! import's warnings and the faces the mesher left out to `OUT/<stem>.log`.

use std::path::Path;
use std::time::{Duration, Instant};

use solidcraft_kernel::{MeshTolerance, step_import};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [out, tolerance, file] = &args[..] else {
        eprintln!("usage: camcraft_export OUT TOLERANCE_MM FILE");
        std::process::exit(2);
    };
    let out = Path::new(out);
    let stem = Path::new(file).file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
    let err = out.join(format!("{stem}.err"));
    // A reader that aborts or overflows its stack still leaves a reason.
    let _ = std::fs::write(&err, "the reader died before finishing (an abort or a signal)");
    let on_panic = err.clone();
    std::panic::set_hook(Box::new(move |info| {
        let _ = std::fs::write(&on_panic, format!("panicked: {info}"));
    }));
    let result = std::panic::catch_unwind(|| {
        tolerance.parse::<f64>().map_err(|e| e.to_string()).and_then(|tolerance| mesh(file, tolerance))
    });
    let written = match result {
        Ok(Ok(Meshed { bin, times, log })) => std::fs::write(out.join(format!("{stem}.log")), log)
            .and_then(|()| std::fs::write(out.join(format!("{stem}.time")), times))
            .and_then(|()| std::fs::write(out.join(format!("{stem}.bin")), bin))
            .and_then(|()| std::fs::remove_file(&err)),
        Ok(Err(why)) => std::fs::write(&err, why),
        Err(_) => Ok(()), // the hook has written the panic
    };
    if let Err(e) = written {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

struct Meshed {
    bin: Vec<u8>,
    times: String,
    log: String,
}

/// The mesh, encoded, how long reading and meshing took, and what the import said.
fn mesh(file: &str, tolerance: f64) -> Result<Meshed, String> {
    let start = Instant::now();
    let bytes = std::fs::read(file).map_err(|e| e.to_string())?;
    let imported = step_import(&String::from_utf8_lossy(&bytes)).map_err(|e| e.to_string())?;
    let read = start.elapsed();
    let mut log: Vec<String> = imported.warnings.clone();
    let t = MeshTolerance::chord(tolerance);
    let (mut vertices, mut triangles, mut faces) = (Vec::new(), Vec::new(), Vec::new());
    for (i, b) in imported.bodies.iter().enumerate() {
        let m = b.body.mesh(&t).map_err(|e| format!("body {i} ({}): {e}", b.name))?;
        let missing = b.body.unmeshed_faces();
        if missing > 0 {
            log.push(format!("body {i} ({}): {missing} face(s) not meshed", b.name));
        }
        let base = u32::try_from(vertices.len()).map_err(|e| e.to_string())?;
        vertices.extend(m.positions.iter().map(|p| [p.x, p.y, p.z]));
        let mut by_face = vec![Vec::new(); b.body.face_count()];
        for (tri, f) in m.triangles.iter().zip(&m.tri_face) {
            match by_face.get_mut(*f as usize) {
                Some(face) => face.push(tri.map(|i| i + base)),
                None => return Err(format!("body {i}: a triangle on face {f} of {}", b.body.face_count())),
            }
        }
        for face in by_face {
            faces.push(face.len() as u64);
            triangles.extend(face);
        }
    }
    let meshed = start.elapsed() - read;
    Ok(Meshed { bin: encode(&vertices, &triangles, &faces), times: phases(read, meshed), log: log.join("\n") })
}

/// `OUT/<stem>.time`: how long reading the file and meshing it took, in seconds.
fn phases(read: Duration, mesh: Duration) -> String {
    format!("read {}\nmesh {}\n", read.as_secs_f64(), mesh.as_secs_f64())
}

fn encode(vertices: &[[f64; 3]], triangles: &[[u32; 3]], faces: &[u64]) -> Vec<u8> {
    let mut b = Vec::new();
    b.extend((vertices.len() as u64).to_le_bytes());
    vertices.iter().flatten().for_each(|c| b.extend(c.to_le_bytes()));
    b.extend((triangles.len() as u64).to_le_bytes());
    triangles.iter().flatten().for_each(|c| b.extend(c.to_le_bytes()));
    b.extend((faces.len() as u64).to_le_bytes());
    faces.iter().for_each(|n| b.extend(n.to_le_bytes()));
    b
}
