//! brepkit (https://github.com/andymai/brepkit) as a step-conformance reader:
//! `brepkit_io::step::read_step` reads the file's solids, and
//! `tessellate_solid_grouped_with_tolerance` meshes each one watertight, its
//! shared edges sampled once for both faces, with the triangles grouped by face.
//!
//! LICENCE: brepkit is AGPL-3.0-only, so the program built from this is AGPL-3.0
//! too. It is only ever built and run locally, as a separate process the runner
//! starts, for testing; it is not distributed and nothing links it.
//!
//!   brepkit-reader OUT TOLERANCE FILE
//!
//! Writes `OUT/<stem>.bin` in the format the README describes, or why not to
//! `OUT/<stem>.err`, and what brepkit logged as a warning to `OUT/<stem>.log`.
//!
//! brepkit's reader ignores the file's unit, so the tolerance is in that unit and
//! so is the output; run it with `:file-units`. It reads `MANIFOLD_SOLID_BREP`
//! and `BREP_WITH_VOIDS` only (no surface models) and leaves every solid where
//! its own representation puts it: assembly placements are not applied. A face
//! the tessellator gives up on comes back with no triangles (brepkit logs it and
//! carries on), which the runner reports as an empty face.

use std::path::Path;
use std::sync::Mutex;

use brepkit_io::step::read_step;
use brepkit_math::chord::DEFAULT_ANGULAR_TOL;
use brepkit_operations::tessellate::tessellate_solid_grouped_with_tolerance;
use brepkit_topology::Topology;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [out, tolerance, file] = &args[..] else {
        eprintln!("usage: brepkit-reader OUT TOLERANCE FILE");
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
    let _ = log::set_logger(&WARNINGS);
    log::set_max_level(log::LevelFilter::Warn);
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

    let result = std::panic::catch_unwind(|| {
        tolerance
            .parse::<f64>()
            .map_err(|e| e.to_string())
            .and_then(|tolerance| mesh(file, tolerance))
    });
    let warnings = WARNINGS.0.lock().map(|w| w.join("\n")).unwrap_or_default();
    let _ = std::fs::write(&log, warnings);
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

fn mesh(file: &str, tolerance: f64) -> Result<Vec<u8>, String> {
    let bytes = std::fs::read(file).map_err(|e| e.to_string())?;
    let mut topo = Topology::new();
    let solids =
        read_step(&String::from_utf8_lossy(&bytes), &mut topo).map_err(|e| e.to_string())?;
    if solids.is_empty() {
        return Err(
            "read_step found no solid (it reads MANIFOLD_SOLID_BREP and BREP_WITH_VOIDS only)"
                .into(),
        );
    }
    let mut soup = Soup::default();
    for (i, &solid) in solids.iter().enumerate() {
        // `offsets[f]` is where face f's corners start in `mesh.indices`, one per
        // face of the solid in its own face order, empty faces included, and a last
        // one where the indices end.
        let (mesh, offsets) =
            tessellate_solid_grouped_with_tolerance(&topo, solid, tolerance, DEFAULT_ANGULAR_TOL)
                .map_err(|e| format!("solid {i}: {e}"))?;
        let base = soup.vertices.len() as u32;
        soup.vertices
            .extend(mesh.positions.iter().map(|p| [p.x(), p.y(), p.z()]));
        soup.triangles.extend(
            mesh.indices
                .chunks_exact(3)
                .map(|t| [t[0] + base, t[1] + base, t[2] + base]),
        );
        soup.faces
            .extend(offsets.windows(2).map(|w| u64::from((w[1] - w[0]) / 3)));
    }
    Ok(soup.encode())
}

/// Every warning brepkit logs, kept for the .log file.
struct Warnings(Mutex<Vec<String>>);

static WARNINGS: Warnings = Warnings(Mutex::new(Vec::new()));

impl log::Log for Warnings {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Warn
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata())
            && let Ok(mut lines) = self.0.lock()
        {
            lines.push(format!(
                "{} {}: {}",
                record.level(),
                record.target(),
                record.args()
            ));
        }
    }

    fn flush(&self) {}
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
