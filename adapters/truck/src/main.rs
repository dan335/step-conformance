//! Upstream truck as a step-conformance reader: truck-stepio reads the
//! file and truck-meshalgo's `robust_triangulation` meshes it.
//!
//!   truck-export OUT TOLERANCE FILE
//!
//! Writes `OUT/<stem>.bin` in the format the README describes, or `OUT/<stem>.err`,
//! and how long reading and meshing took to `OUT/<stem>.time`. truck ignores the file's unit, so the tolerance is in that unit and so is the
//! output; the runner scales it (`:file-units`).

use std::path::Path;
use std::time::{Duration, Instant};

use truck_meshalgo::prelude::*;
use truck_stepio::r#in::Table;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [out, tolerance, file] = &args[..] else {
        eprintln!("usage: truck-export OUT TOLERANCE FILE");
        std::process::exit(2);
    };
    let out = Path::new(out);
    let stem = Path::new(file).file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
    let result = tolerance
        .parse::<f64>()
        .map_err(|e| e.to_string())
        .and_then(|tolerance| mesh(file, tolerance));
    let written = match result {
        Ok((bin, times)) => std::fs::write(out.join(format!("{stem}.time")), times)
            .and_then(|()| std::fs::write(out.join(format!("{stem}.bin")), bin)),
        Err(why) => std::fs::write(out.join(format!("{stem}.err")), why),
    };
    if let Err(e) = written {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

/// The mesh, encoded, and how long reading and meshing took.
fn mesh(file: &str, tolerance: f64) -> Result<(Vec<u8>, String), String> {
    let start = Instant::now();
    let text = std::fs::read_to_string(file).map_err(|e| e.to_string())?;
    let table = Table::from_step(&text).ok_or("truck-stepio could not parse the file")?;
    let mut ids: Vec<_> = table.shell.keys().copied().collect();
    ids.sort_unstable();
    let mut shells = Vec::new();
    for id in ids {
        let shell = table
            .to_compressed_shell(&table.shell[&id])
            .map_err(|e| format!("shell #{id}: {e:?}"))?;
        shells.push(shell);
    }
    let read = start.elapsed();
    let (mut vertices, mut triangles, mut faces) = (Vec::new(), Vec::new(), Vec::new());
    for shell in &shells {
        for face in &shell.robust_triangulation(tolerance).faces {
            let mut n = 0u64;
            if let Some(polygon) = &face.surface {
                let positions = polygon.positions();
                for tri in polygon.faces().triangle_iter() {
                    let mut corners = [0u32; 3];
                    for (slot, v) in corners.iter_mut().zip(tri.iter()) {
                        let p = positions[v.pos];
                        vertices.push([p.x, p.y, p.z]);
                        *slot = (vertices.len() - 1) as u32;
                    }
                    if !face.orientation {
                        corners.swap(1, 2);
                    }
                    triangles.push(corners);
                    n += 1;
                }
            }
            faces.push(n);
        }
    }
    let meshed = start.elapsed() - read;
    Ok((encode(&vertices, &triangles, &faces), phases(read, meshed)))
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
