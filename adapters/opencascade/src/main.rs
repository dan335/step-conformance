//! OpenCascade as a step-conformance reader: `Shape::read_step`, then
//! `mesh_with_tolerance`, through opencascade-rs. OpenCascade converts to
//! millimetres itself.
//!
//!   opencascade-reader OUT TOLERANCE_MM FILE
//!
//! Writes `OUT/<stem>.bin` in the format the README describes, or `OUT/<stem>.err`.

use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [out, tolerance, file] = &args[..] else {
        eprintln!("usage: opencascade-reader OUT TOLERANCE_MM FILE");
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
    let shape = opencascade::primitives::Shape::read_step(file).map_err(|e| e.to_string())?;
    let mesh = shape.mesh_with_tolerance(tolerance).map_err(|e| e.to_string())?;
    let mut b = Vec::new();
    b.extend((mesh.vertices.len() as u64).to_le_bytes());
    for v in &mesh.vertices {
        for c in [v.x, v.y, v.z] {
            b.extend(c.to_le_bytes());
        }
    }
    b.extend((mesh.indices.len() as u64 / 3).to_le_bytes());
    for &i in &mesh.indices {
        b.extend(u32::try_from(i).map_err(|e| e.to_string())?.to_le_bytes());
    }
    b.extend((mesh.face_triangles.len() as u64).to_le_bytes());
    for &n in &mesh.face_triangles {
        b.extend((n as u64).to_le_bytes());
    }
    Ok(b)
}
