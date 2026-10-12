//! Score STEP readers on the corpus.
//!
//! ```text
//! step-conformance --out DIR --reference NAME=PROGRAM [--reader NAME=PROGRAM[:file-units]]...
//!     [--tolerances 0.0125,0.05,0.1,0.5] [--reference-tolerance 0.001] [--timeout 60]
//!     FILE_OR_DIR...
//! ```
//!
//! A reader is a program run as `PROGRAM OUT TOLERANCE FILE` that writes the file's
//! triangles grouped by B-rep face to `OUT/<stem>.bin`, or why it could not to
//! `OUT/<stem>.err` (see the README for the format). `:file-units` marks a reader
//! that ignores the file's unit: it is given the tolerance in the file's unit, and
//! its output is scaled to millimetres here.
//!
//! Every part is read by each reader at each tolerance. The mesh is checked against
//! the file's own facts with `step-check` (every face arrives with triangles, nothing
//! lands outside the file's geometry, the mesh reaches every vertex), then measured
//! in both directions against the reference reader's mesh at the far finer reference
//! tolerance: how far its points lie from that surface, and how far that surface lies
//! from it, which finds missing surface. A reader passes a part at a tolerance when
//! the checks pass and neither distance exceeds four times the tolerance.

use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

use glam::{DVec3, dvec3};
use parry3d_f64::query::PointQuery;
use parry3d_f64::shape::TriMesh;
use step_check::{FileCheck, KernelMesh, Problem, part21};

/// How far past the tolerance a reader may land and still pass. Four, because
/// OpenCascade itself reaches 3.4 times its tolerance on `bowl_01` at 0.0125 mm.
const SLACK: f64 = 4.0;

/// Farther than this from the reference counts as this far: a metre.
const FAR: f64 = 1000.0;

struct Reader {
    name: String,
    program: PathBuf,
    file_units: bool,
    passed: usize,
    failed: Vec<String>,
}

impl Reader {
    fn parse(spec: &str) -> Result<Self, Box<dyn Error>> {
        let (name, program) = spec.split_once('=').ok_or("expected NAME=PROGRAM")?;
        let (program, file_units) = match program.strip_suffix(":file-units") {
            Some(p) => (p, true),
            None => (program, false),
        };
        Ok(Self {
            name: name.to_string(),
            program: PathBuf::from(program),
            file_units,
            passed: 0,
            failed: Vec::new(),
        })
    }
}

struct Args {
    out: PathBuf,
    reference: Option<Reader>,
    readers: Vec<Reader>,
    tolerances: Vec<f64>,
    reference_tolerance: f64,
    timeout: Duration,
    parts: Vec<PathBuf>,
}

fn args() -> Result<Args, Box<dyn Error>> {
    let mut a = Args {
        out: PathBuf::from("work/out"),
        reference: None,
        readers: Vec::new(),
        tolerances: vec![0.0125, 0.05, 0.1, 0.5],
        reference_tolerance: 0.001,
        timeout: Duration::from_secs(60),
        parts: Vec::new(),
    };
    let mut it = std::env::args().skip(1);
    while let Some(arg) = it.next() {
        let mut value = || it.next().ok_or_else(|| format!("{arg} needs a value"));
        match arg.as_str() {
            "--out" => a.out = PathBuf::from(value()?),
            "--reference" => a.reference = Some(Reader::parse(&value()?)?),
            "--reader" => a.readers.push(Reader::parse(&value()?)?),
            "--tolerances" => {
                a.tolerances = value()?
                    .split(',')
                    .map(str::parse)
                    .collect::<Result<_, _>>()?;
            }
            "--reference-tolerance" => a.reference_tolerance = value()?.parse()?,
            "--timeout" => a.timeout = Duration::from_secs(value()?.parse()?),
            path => {
                let path = PathBuf::from(path);
                if path.is_dir() {
                    let mut steps: Vec<PathBuf> = std::fs::read_dir(&path)?
                        .filter_map(|e| e.ok().map(|e| e.path()))
                        .filter(|p| {
                            p.extension().is_some_and(|e| {
                                e.eq_ignore_ascii_case("step") || e.eq_ignore_ascii_case("stp")
                            })
                        })
                        .collect();
                    steps.sort();
                    a.parts.extend(steps);
                } else {
                    a.parts.push(path);
                }
            }
        }
    }
    Ok(a)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut a = args()?;
    let reference = a.reference.take().ok_or("--reference NAME=PROGRAM is required")?;
    println!(
        "| part | tolerance mm | {} |",
        a.readers.iter().map(|r| r.name.as_str()).collect::<Vec<_>>().join(" | ")
    );
    println!("|---|---|{}", "---|".repeat(a.readers.len()));
    for part in &a.parts {
        let stem = stem(part);
        let facts = part21::facts(&String::from_utf8_lossy(&std::fs::read(part)?))?;
        let truth = match read(&reference, &a.out, a.reference_tolerance, part, &facts, a.timeout)? {
            Ran::Meshed(mesh, _) => Surface::new(&mesh)?,
            Ran::Refused(why) => {
                println!("| {stem} | | the reference could not read it: {why} |");
                continue;
            }
            Ran::TimedOut => {
                println!("| {stem} | | the reference timed out |");
                continue;
            }
        };
        for &tolerance in &a.tolerances {
            let mut cells = Vec::new();
            for reader in &mut a.readers {
                let (cell, passed) = match read(reader, &a.out, tolerance, part, &facts, a.timeout)? {
                    Ran::TimedOut => (format!("timed out at {} s", a.timeout.as_secs()), false),
                    Ran::Refused(why) => (format!("could not read: {}", one_line(&why)), false),
                    Ran::Meshed(mesh, took) => verdict(&facts, &mesh, &truth, tolerance, took)?,
                };
                if passed {
                    reader.passed += 1;
                } else {
                    reader.failed.push(format!("{stem} at {tolerance} mm: {cell}"));
                }
                cells.push(cell);
            }
            println!("| {stem} | {tolerance} | {} |", cells.join(" | "));
        }
    }
    println!();
    let runs = a.parts.len() * a.tolerances.len();
    for r in &a.readers {
        println!("**{}** passes {} of {runs}.", r.name, r.passed);
        for f in &r.failed {
            println!("- {f}");
        }
        println!();
    }
    Ok(())
}

/// A cell of the table, and whether it passes.
fn verdict(
    facts: &part21::StepFacts,
    mesh: &KernelMesh,
    truth: &Surface,
    tolerance: f64,
    took: Duration,
) -> Result<(String, bool), Box<dyn Error>> {
    let surface = Surface::new(mesh)?;
    let off = surface.farthest_from(truth).max(truth.farthest_from(&surface));
    let secs = took.as_secs_f64();
    Ok(match step_check::check(facts, mesh, tolerance) {
        Err(problem) => (format!("{}, {off:.3} mm off", short(&problem)), false),
        Ok(_) if off > SLACK * tolerance => (format!("{off:.3} mm off, {secs:.1} s"), false),
        Ok(FileCheck::Passed) => (format!("ok, {off:.3} mm, {secs:.1} s"), true),
        Ok(FileCheck::Skipped(_)) => (format!("ok, {off:.3} mm, {secs:.1} s (file checks skipped)"), true),
    })
}

/// A file check's refusal, in a few words.
fn short(p: &Problem) -> String {
    match p {
        Problem::DroppedFaces { in_file, meshed } => format!("{meshed} of {in_file} faces"),
        Problem::EmptyFace { face } => format!("face {face} empty"),
        Problem::OutsideFile { axis, by } => format!("outside the file by {by:.1} mm on {axis}"),
        Problem::ShortOfVertices { axis, by } => format!("short of the vertices by {by:.1} mm on {axis}"),
        other => other.to_string(),
    }
}

fn one_line(s: &str) -> String {
    s.lines().next().unwrap_or("").chars().take(80).collect()
}

fn stem(part: &Path) -> String {
    part.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default()
}

/// A mesh as a surface to measure distances to.
struct Surface {
    points: Vec<DVec3>,
    mesh: Option<TriMesh>,
}

impl Surface {
    fn new(mesh: &KernelMesh) -> Result<Self, Box<dyn Error>> {
        let triangles: Vec<[u32; 3]> = mesh
            .indices
            .chunks_exact(3)
            .map(|t| -> Result<[u32; 3], Box<dyn Error>> {
                Ok([t[0].try_into()?, t[1].try_into()?, t[2].try_into()?])
            })
            .collect::<Result<_, _>>()?;
        // Every vertex, and every triangle's centre, where a flat triangle strays
        // farthest from a curved surface.
        let mut points = mesh.vertices.clone();
        for t in &triangles {
            let corner = |i: u32| mesh.vertices.get(i as usize).copied().unwrap_or(DVec3::NAN);
            points.push((corner(t[0]) + corner(t[1]) + corner(t[2])) / 3.0);
        }
        let shape = if triangles.is_empty() {
            None
        } else {
            Some(TriMesh::new(mesh.vertices.clone(), triangles)?)
        };
        Ok(Self { points, mesh: shape })
    }

    /// The farthest any of this surface's points lies from `other`, up to [`FAR`].
    fn farthest_from(&self, other: &Surface) -> f64 {
        let Some(mesh) = &other.mesh else { return FAR };
        self.points
            .iter()
            .filter(|p| p.is_finite())
            .map(|p| (mesh.project_local_point(*p, false).point - *p).length().min(FAR))
            .fold(0.0, f64::max)
    }
}

enum Ran {
    Meshed(KernelMesh, Duration),
    Refused(String),
    TimedOut,
}

/// Run one reader on one file at one tolerance, killing it past `timeout`.
fn read(
    reader: &Reader,
    out: &Path,
    tolerance: f64,
    part: &Path,
    facts: &part21::StepFacts,
    timeout: Duration,
) -> Result<Ran, Box<dyn Error>> {
    let out = out.join(&reader.name).join(tolerance.to_string());
    std::fs::create_dir_all(&out)?;
    let stem = stem(part);
    let (bin, err) = (out.join(format!("{stem}.bin")), out.join(format!("{stem}.err")));
    for stale in [&bin, &err] {
        std::fs::remove_file(stale).ok();
    }
    let asked = if reader.file_units { tolerance / facts.unit.mm } else { tolerance };
    let start = Instant::now();
    let mut child = Command::new(&reader.program)
        .arg(&out)
        .arg(asked.to_string())
        .arg(part)
        .spawn()
        .map_err(|e| format!("{}: {e}", reader.program.display()))?;
    loop {
        if child.try_wait()?.is_some() {
            break;
        }
        if start.elapsed() > timeout {
            child.kill().ok();
            child.wait().ok();
            return Ok(Ran::TimedOut);
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    let took = start.elapsed();
    if let Ok(why) = std::fs::read_to_string(&err) {
        return Ok(Ran::Refused(why.trim().to_string()));
    }
    let Ok(bytes) = std::fs::read(&bin) else {
        return Ok(Ran::Refused("it wrote neither a mesh nor a reason".into()));
    };
    let mut mesh = match decode(&bytes) {
        Ok(mesh) => mesh,
        Err(e) => return Ok(Ran::Refused(format!("its mesh file is malformed: {e}"))),
    };
    if reader.file_units {
        for v in &mut mesh.vertices {
            *v *= facts.unit.mm;
        }
    }
    Ok(Ran::Meshed(mesh, took))
}

/// The readers' mesh format; see the README.
fn decode(b: &[u8]) -> Result<KernelMesh, Box<dyn Error>> {
    let mut c = Cursor { b, at: 0 };
    let mut out = KernelMesh::default();
    for _ in 0..c.u64()? {
        let (x, y, z) = (c.u64()?, c.u64()?, c.u64()?);
        out.vertices
            .push(dvec3(f64::from_bits(x), f64::from_bits(y), f64::from_bits(z)));
    }
    for _ in 0..c.u64()? * 3 {
        out.indices.push(c.u32()? as usize);
    }
    for _ in 0..c.u64()? {
        out.face_triangles.push(usize::try_from(c.u64()?)?);
    }
    if out.vertices.iter().any(|v| !v.is_finite()) {
        return Err("a vertex is not finite".into());
    }
    if out.indices.iter().any(|&i| i >= out.vertices.len()) {
        return Err("a triangle names a vertex that is not there".into());
    }
    Ok(out)
}

struct Cursor<'a> {
    b: &'a [u8],
    at: usize,
}

impl Cursor<'_> {
    fn take<const N: usize>(&mut self) -> Result<[u8; N], Box<dyn Error>> {
        let s = self.b.get(self.at..self.at + N).ok_or("it ends early")?;
        self.at += N;
        Ok(s.try_into()?)
    }

    fn u64(&mut self) -> Result<u64, Box<dyn Error>> {
        Ok(u64::from_le_bytes(self.take()?))
    }

    fn u32(&mut self) -> Result<u32, Box<dyn Error>> {
        Ok(u32::from_le_bytes(self.take()?))
    }
}
