//! solidcraft's STEP reader as a step-conformance reader. `run.sh` copies this into a
//! solidcraft checkout as `crates/kernel/examples/camcraft_export.rs` and builds it.
//! solidcraft (https://github.com/bherbruck/solidcraft) reads STEP with its own
//! reader and meshes with its patched truck-meshalgo; this asks for the refined
//! tessellation, which splits curved triangles until they lie within the tolerance.
//!
//!   camcraft_export OUT TOLERANCE_MM FILE
//!
//! Writes `OUT/<stem>.bin` in the format the README describes, in millimetres, or
//! `OUT/<stem>.err`.

use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [out, tolerance, file] = &args[..] else {
        eprintln!("usage: camcraft_export OUT TOLERANCE_MM FILE");
        std::process::exit(2);
    };
    let out = Path::new(out);
    let stem = Path::new(file).file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
    let result = tolerance
        .parse::<f64>()
        .map_err(|e| e.to_string())
        .and_then(|tolerance| mesh(file, tolerance));
    let written = match result {
        Ok(bin) => std::fs::write(out.join(format!("{stem}.bin")), bin),
        Err(why) => std::fs::write(out.join(format!("{stem}.err")), why),
    };
    if let Err(e) = written {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn mesh(file: &str, tolerance: f64) -> Result<Vec<u8>, String> {
    let bytes = std::fs::read(file).map_err(|e| e.to_string())?;
    let imported = solvecraft_kernel::step_import(&String::from_utf8_lossy(&bytes)).map_err(|e| e.to_string())?;
    let (mut vertices, mut triangles, mut faces) = (Vec::new(), Vec::new(), Vec::new());
    for b in &imported.bodies {
        let m = b.body.tessellate_with(tolerance, true).map_err(|e| e.to_string())?;
        let base = vertices.len() as u32;
        vertices.extend(m.positions.iter().map(|p| [p.x, p.y, p.z]));
        let mut by_face = vec![Vec::new(); b.body.face_count()];
        for (t, f) in m.triangles.iter().zip(&m.tri_face) {
            if let Some(face) = by_face.get_mut(*f as usize) {
                face.push(t.map(|i| i + base));
            }
        }
        for face in by_face {
            faces.push(face.len() as u64);
            triangles.extend(face);
        }
    }
    Ok(encode(&vertices, &triangles, &faces))
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
