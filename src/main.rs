//! Score STEP readers on the corpus.
//!
//! ```text
//! step-conformance --out DIR --reference NAME=PROGRAM [--reader NAME=PROGRAM[:file-units]]...
//!     [--tolerances 0.0125,0.05,0.1,0.5] [--reference-tolerance 0.001]
//!     [--timeout 60] [--reference-timeout 600] [--repeats 5]
//!     FILE_OR_DIR...
//! ```
//!
//! A reader is a program run as `PROGRAM OUT TOLERANCE FILE` that writes the file's
//! triangles grouped by B-rep face to `OUT/<stem>.bin`, or why it could not to
//! `OUT/<stem>.err` (see the README for the format), and may write how long it spent
//! reading and meshing to `OUT/<stem>.time`. `:file-units` marks a reader that
//! ignores the file's unit: it is given the tolerance in the file's unit, and its
//! output is scaled to millimetres here.
//!
//! Every part is read by each reader at each tolerance. The mesh is checked against
//! the file's own facts with `step-check` (every face arrives with triangles, nothing
//! lands outside the file's geometry, the mesh reaches every vertex), then measured
//! in both directions against the part's true surface: how far the mesh lies from it,
//! and how far it lies from the mesh, which finds missing surface. A generated part
//! carries its exact shape in a `truth.json` beside it; any other part is measured
//! against the reference reader's mesh at the far finer reference tolerance. A reader
//! passes a part at a tolerance when the checks pass and neither distance exceeds four
//! times the tolerance.
//!
//! A run that passes is repeated for its speed, up to `--repeats` times or for
//! [`REPEAT_FOR`] in all, and the median taken of its time, CPU time, peak memory, and
//! its own account of reading and meshing. The readers are then compared on the runs
//! they pass, each against the first.

mod truth;

use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

use glam::{DVec3, dvec3};
use parry3d_f64::query::PointQuery;
use parry3d_f64::shape::TriMesh;
use step_check::{FileCheck, KernelMesh, Problem, part21};
use truth::Truth;

/// How far past the tolerance a reader may land and still pass. Four, because
/// OpenCascade itself reaches 3.4 times its tolerance on `bowl_01` at 0.0125 mm.
const SLACK: f64 = 4.0;

/// Farther than this from the reference counts as this far: a metre.
const FAR: f64 = 1000.0;

/// A passing run is repeated until it has run this long in all, or `--repeats` times.
const REPEAT_FOR: Duration = Duration::from_secs(10);

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
    reference_timeout: Duration,
    repeats: usize,
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
        reference_timeout: Duration::from_secs(600),
        repeats: 5,
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
            "--reference-timeout" => a.reference_timeout = Duration::from_secs(value()?.parse()?),
            "--repeats" => a.repeats = value()?.parse::<usize>()?.max(1),
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
    #[cfg(unix)]
    if std::env::args().nth(1).as_deref() == Some(RELAY) {
        return relay();
    }
    let mut a = args()?;
    let reference = a.reference.take().ok_or("--reference NAME=PROGRAM is required")?;
    println!(
        "| part | tolerance mm | {} |",
        a.readers.iter().map(|r| r.name.as_str()).collect::<Vec<_>>().join(" | ")
    );
    println!("|---|---|{}", "---|".repeat(a.readers.len()));
    let mut runs = Vec::new();
    for part in &a.parts {
        let stem = stem(part);
        let facts = part21::facts(&String::from_utf8_lossy(&std::fs::read(part)?))?;
        let truth = match Truth::beside(part)? {
            Some(exact) => {
                let samples = exact.samples(64, 256);
                Against::Exact(exact, samples)
            }
            None => match read(&reference, &a.out, a.reference_tolerance, part, &facts, a.reference_timeout)? {
                Ran::Meshed(mesh, _) => Against::Mesh(Surface::new(&mesh)?),
                Ran::Refused(why) => {
                    println!("| {stem} | | the reference could not read it: {why} |");
                    continue;
                }
                Ran::TimedOut => {
                    println!("| {stem} | | the reference timed out |");
                    continue;
                }
            },
        };
        for &tolerance in &a.tolerances {
            let mut cells = Vec::new();
            let mut timed = Vec::new();
            for reader in &mut a.readers {
                // The cell, and what the run cost if it passed.
                let (cell, passed) = match read(reader, &a.out, tolerance, part, &facts, a.timeout)? {
                    Ran::TimedOut => (format!("timed out at {} s", a.timeout.as_secs()), None),
                    Ran::Refused(why) => (format!("could not read: {}", one_line(&why)), None),
                    Ran::Meshed(mesh, first) => {
                        let verdict = verdict(&facts, &mesh, &truth, tolerance)?;
                        if verdict.passed() {
                            let mut costs = vec![first];
                            let mut spent = first.wall;
                            while costs.len() < a.repeats && spent < REPEAT_FOR {
                                let Ran::Meshed(_, again) = read(reader, &a.out, tolerance, part, &facts, a.timeout)?
                                else {
                                    break;
                                };
                                spent += again.wall;
                                costs.push(again);
                            }
                            let cost = Cost::median(&costs);
                            let triangles = mesh.indices.len() / 3;
                            (verdict.cell(cost.wall), Some(Timed { cost, triangles }))
                        } else {
                            (verdict.cell(first.wall), None)
                        }
                    }
                };
                if passed.is_some() {
                    reader.passed += 1;
                } else {
                    reader.failed.push(format!("{stem} at {tolerance} mm: {cell}"));
                }
                cells.push(cell);
                timed.push(passed);
            }
            println!("| {stem} | {tolerance} | {} |", cells.join(" | "));
            runs.push(Run { stem: stem.clone(), tolerance, timed });
        }
    }
    println!();
    for r in &a.readers {
        let runs = r.passed + r.failed.len();
        println!("**{}** passes {} of {runs}.", r.name, r.passed);
        for f in &r.failed {
            println!("- {f}");
        }
        println!();
    }
    speed(&a, &runs);
    Ok(())
}

/// How a mesh fared: refused by a file check, too far from the true surface, or
/// passed. Each carries the worst distance to the true surface.
enum Verdict {
    Refused(Problem, f64),
    Off(f64),
    Passed { off: f64, checked: bool },
}

impl Verdict {
    fn passed(&self) -> bool {
        matches!(self, Verdict::Passed { .. })
    }

    /// A cell of the table, with the time the reader took.
    fn cell(&self, took: Duration) -> String {
        let took = time(took);
        match self {
            Verdict::Refused(problem, off) => format!("{}, {off:.3} mm off", short(problem)),
            Verdict::Off(off) => format!("{off:.3} mm off, {took}"),
            Verdict::Passed { off, checked: true } => format!("ok, {off:.3} mm, {took}"),
            Verdict::Passed { off, checked: false } => format!("ok, {off:.3} mm, {took} (file checks skipped)"),
        }
    }
}

fn verdict(
    facts: &part21::StepFacts,
    mesh: &KernelMesh,
    truth: &Against,
    tolerance: f64,
) -> Result<Verdict, Box<dyn Error>> {
    let off = truth.off(&Surface::new(mesh)?);
    Ok(match step_check::check(facts, mesh, tolerance) {
        Err(problem) => Verdict::Refused(problem, off),
        Ok(_) if off > SLACK * tolerance => Verdict::Off(off),
        Ok(FileCheck::Passed) => Verdict::Passed { off, checked: true },
        Ok(FileCheck::Skipped(_)) => Verdict::Passed { off, checked: false },
    })
}

/// One part at one tolerance: what each reader's passing run cost, or `None` where
/// it failed.
struct Run {
    stem: String,
    tolerance: f64,
    timed: Vec<Option<Timed>>,
}

/// What a passing run cost, the median over its repeats, and how many triangles it
/// made.
#[derive(Clone, Copy)]
struct Timed {
    cost: Cost,
    triangles: usize,
}

/// The speed and memory section: each reader against the first on the runs both
/// pass, then every passing run.
fn speed(a: &Args, runs: &[Run]) {
    let Some(first) = a.readers.first() else {
        return;
    };
    let names: Vec<&str> = a.readers.iter().map(|r| r.name.as_str()).collect();
    println!("## Speed and memory");
    println!();
    println!(
        "Every run a reader passes is repeated, up to {} times or for {} s, and the median",
        a.repeats,
        REPEAT_FOR.as_secs()
    );
    println!("taken. Time runs from starting the reader to its exit, CPU time is across all its");
    println!("threads, reading and meshing are the reader's own account of them, and memory is");
    println!("its peak resident memory.");
    println!();
    println!("Against {}, on the runs both pass: each figure is the reader's divided by", first.name);
    println!("{}'s, averaged geometrically, so 0.5× is twice as fast, or half the memory.", first.name);
    println!();
    println!("| reader | runs both pass | time | CPU time | reading | meshing | peak memory | triangles |");
    println!("|---|---|---|---|---|---|---|---|");
    type Figure = fn(&Timed) -> Option<f64>;
    let figures: [Figure; 6] = [
        |t| Some(t.cost.wall.as_secs_f64()),
        |t| t.cost.cpu.map(|d| d.as_secs_f64()),
        |t| t.cost.read.map(|d| d.as_secs_f64()),
        |t| t.cost.mesh.map(|d| d.as_secs_f64()),
        |t| t.cost.memory.map(|b| b as f64),
        |t| Some(t.triangles as f64),
    ];
    for (i, name) in names.iter().enumerate().skip(1) {
        let pairs: Vec<(&Timed, &Timed)> = runs
            .iter()
            .filter_map(|r| Some((r.timed[i].as_ref()?, r.timed[0].as_ref()?)))
            .collect();
        let cells: Vec<String> = figures
            .iter()
            .map(|figure| {
                let ratios: Vec<f64> = pairs
                    .iter()
                    .filter_map(|(t, base)| Some(figure(t)? / figure(base)?))
                    .filter(|r| r.is_finite() && *r > 0.0)
                    .collect();
                if ratios.is_empty() {
                    return "–".to_string();
                }
                let mean = (ratios.iter().map(|r| r.ln()).sum::<f64>() / ratios.len() as f64).exp();
                format!("{}×", figures2(mean))
            })
            .collect();
        println!("| {name} | {} | {} |", pairs.len(), cells.join(" | "));
    }
    println!();
    println!("Every run that passes: the time, the reader's own reading and meshing, peak memory,");
    println!("and triangles.");
    println!();
    println!("| part | tolerance mm | {} |", names.join(" | "));
    println!("|---|---|{}", "---|".repeat(names.len()));
    for r in runs {
        let cells: Vec<String> = r
            .timed
            .iter()
            .map(|t| match t {
                None => "–".to_string(),
                Some(t) => {
                    let mut cell = time(t.cost.wall);
                    if let (Some(read), Some(mesh)) = (t.cost.read, t.cost.mesh) {
                        cell += &format!(" (read {}, mesh {})", time(read), time(mesh));
                    }
                    if let Some(bytes) = t.cost.memory {
                        cell += &format!(", {}", megabytes(bytes));
                    }
                    cell + &format!(", {} triangles", thousands(t.triangles))
                }
            })
            .collect();
        println!("| {} | {} | {} |", r.stem, r.tolerance, cells.join(" | "));
    }
}

/// A duration in a few figures: 0.8 ms, 85 ms, 1.84 s, 22.4 s.
fn time(d: Duration) -> String {
    let s = d.as_secs_f64();
    if s < 0.01 {
        format!("{:.1} ms", s * 1e3)
    } else if s < 1.0 {
        format!("{:.0} ms", s * 1e3)
    } else if s < 10.0 {
        format!("{s:.2} s")
    } else {
        format!("{s:.1} s")
    }
}

fn megabytes(bytes: u64) -> String {
    let mb = bytes as f64 / 1e6;
    if mb < 10.0 { format!("{mb:.1} MB") } else { format!("{mb:.0} MB") }
}

/// A ratio to two significant figures: 0.042, 0.42, 4.2, 42.
fn figures2(x: f64) -> String {
    let places = (1.0 - x.log10().floor()).max(0.0) as usize;
    format!("{x:.places$}")
}

/// 12345 as 12,345.
fn thousands(n: usize) -> String {
    let digits = n.to_string();
    let mut out = String::new();
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c);
    }
    out
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

/// What a reader's mesh is measured against.
enum Against {
    /// The part's exact shape, and points spread over it.
    Exact(Truth, Vec<DVec3>),
    /// The reference reader's far finer mesh.
    Mesh(Surface),
}

impl Against {
    /// The worst distance either way between `mesh` and the true surface.
    fn off(&self, mesh: &Surface) -> f64 {
        match self {
            Against::Exact(truth, samples) => {
                let from = mesh.points.iter().filter(|p| p.is_finite());
                let out = from.map(|p| truth.distance(*p).min(FAR)).fold(0.0, f64::max);
                let missing = samples.iter().map(|p| mesh.distance(*p)).fold(0.0, f64::max);
                out.max(missing)
            }
            Against::Mesh(reference) => mesh
                .farthest_from(reference)
                .max(reference.farthest_from(mesh)),
        }
    }
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

    /// How far `p` is from this surface, up to [`FAR`].
    fn distance(&self, p: DVec3) -> f64 {
        match &self.mesh {
            Some(mesh) => (mesh.project_local_point(p, false).point - p).length().min(FAR),
            None => FAR,
        }
    }

    /// The farthest any of this surface's points lies from `other`, up to [`FAR`].
    fn farthest_from(&self, other: &Surface) -> f64 {
        self.points
            .iter()
            .filter(|p| p.is_finite())
            .map(|p| other.distance(*p))
            .fold(0.0, f64::max)
    }
}

enum Ran {
    Meshed(KernelMesh, Cost),
    Refused(String),
    TimedOut,
}

/// What one run of a reader cost.
#[derive(Debug, Clone, Copy, Default)]
struct Cost {
    /// From starting the reader to its exit.
    wall: Duration,
    /// User and system time across all its threads, where the system reports it.
    cpu: Option<Duration>,
    /// Its peak resident memory in bytes, where the system reports it.
    memory: Option<u64>,
    /// Its own account of reading the file and of meshing it, from `<stem>.time`.
    read: Option<Duration>,
    mesh: Option<Duration>,
}

impl Cost {
    /// The median of each figure over `runs`, taken apart: the median time and
    /// the median memory need not come from the same run.
    fn median(runs: &[Cost]) -> Cost {
        fn mid(mut v: Vec<f64>) -> Option<f64> {
            v.sort_by(f64::total_cmp);
            let n = v.len();
            (n > 0).then(|| (v[(n - 1) / 2] + v[n / 2]) / 2.0)
        }
        let secs = |f: fn(&Cost) -> Option<Duration>| {
            mid(runs.iter().filter_map(f).map(|d| d.as_secs_f64()).collect()).map(Duration::from_secs_f64)
        };
        Cost {
            wall: secs(|c| Some(c.wall)).unwrap_or_default(),
            cpu: secs(|c| c.cpu),
            memory: mid(runs.iter().filter_map(|c| c.memory).map(|b| b as f64).collect()).map(|b| b as u64),
            read: secs(|c| c.read),
            mesh: secs(|c| c.mesh),
        }
    }
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
    let times = out.join(format!("{stem}.time"));
    let report = out.join(format!("{stem}.cost"));
    for stale in [&bin, &err, &times, &report] {
        std::fs::remove_file(stale).ok();
    }
    let asked = if reader.file_units { tolerance / facts.unit.mm } else { tolerance };
    let mut command = Command::new(&reader.program);
    command.arg(&out).arg(asked.to_string()).arg(part);
    let ran = run(command, &report, timeout).map_err(|e| format!("{}: {e}", reader.program.display()))?;
    let Some(mut cost) = ran else {
        return Ok(Ran::TimedOut);
    };
    (cost.read, cost.mesh) = phases(&times);
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
    Ok(Ran::Meshed(mesh, cost))
}

/// The first argument that makes this program [`relay`] instead of scoring.
#[cfg(unix)]
const RELAY: &str = "--relay";

/// Run `command` to its exit, or kill it past `timeout` and return `None`.
///
/// The reader is not started from here but by a [`relay`], a fresh copy of this
/// program that writes what the reader cost to `report`. Linux counts a process's
/// peak memory from before it replaced itself with the reader, so a reader started
/// from here would be charged this program's own peak, the parts and surfaces it
/// holds. Started from the relay, it is charged the relay's, a few megabytes. The
/// relay leads its own process group, so a timeout kills the reader with it.
#[cfg(unix)]
fn run(command: Command, report: &Path, timeout: Duration) -> std::io::Result<Option<Cost>> {
    use std::os::unix::process::CommandExt;
    let mut relay = Command::new(std::env::current_exe()?);
    relay.arg(RELAY).arg(report).arg(command.get_program()).args(command.get_args()).process_group(0);
    let mut child = relay.spawn()?;
    let pid = child.id() as libc::pid_t;
    let (exited, exit) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let _ = exited.send(child.wait());
    });
    match exit.recv_timeout(timeout) {
        Ok(status) => {
            status?;
            let figures: Vec<u64> = std::fs::read_to_string(report)?
                .split_whitespace()
                .map(|n| n.parse().map_err(std::io::Error::other))
                .collect::<Result<_, _>>()?;
            let [wall, cpu, memory] = figures[..] else {
                return Err(std::io::Error::other(format!("the relay's report {} is malformed", report.display())));
            };
            Ok(Some(Cost {
                wall: Duration::from_nanos(wall),
                cpu: Some(Duration::from_nanos(cpu)),
                memory: Some(memory),
                ..Cost::default()
            }))
        }
        Err(_) => {
            // SAFETY: a plain system call. The relay is not yet reaped, so its group
            // is still its own and the reader's.
            unsafe { libc::kill(-pid, libc::SIGKILL) };
            let _ = exit.recv();
            Ok(None)
        }
    }
}

/// `--relay REPORT PROGRAM ARGS...`: run `PROGRAM ARGS...` and write to `REPORT` how
/// long it ran, its CPU time and its peak memory in bytes, in nanoseconds and bytes
/// on one line. See [`run`].
#[cfg(unix)]
fn relay() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args_os().skip(2);
    let (Some(report), Some(program)) = (args.next(), args.next()) else {
        return Err(format!("{RELAY} REPORT PROGRAM ARGS...").into());
    };
    let start = Instant::now();
    let pid = Command::new(program).args(args).spawn()?.id() as libc::pid_t;
    let mut status = 0;
    // SAFETY: `rusage` is plain integers, for which all zeroes is a valid value, and
    // `wait4` writes only through the two pointers it is given.
    let mut usage: libc::rusage = unsafe { std::mem::zeroed() };
    while unsafe { libc::wait4(pid, &mut status, 0, &mut usage) } < 0 {
        let e = std::io::Error::last_os_error();
        if e.kind() != std::io::ErrorKind::Interrupted {
            return Err(e.into());
        }
    }
    let wall = start.elapsed();
    let nanos = |t: libc::timeval| t.tv_sec as u64 * 1_000_000_000 + t.tv_usec as u64 * 1000;
    // Linux and the BSDs count the peak in kilobytes; macOS in bytes.
    let unit = if cfg!(target_os = "macos") { 1 } else { 1024 };
    let figures = format!(
        "{} {} {}\n",
        wall.as_nanos(),
        nanos(usage.ru_utime) + nanos(usage.ru_stime),
        usage.ru_maxrss.max(0) as u64 * unit
    );
    std::fs::write(report, figures)?;
    Ok(())
}

/// Run `command` to its exit, or kill it past `timeout` and return `None`. Off Unix
/// there is no `wait4`: only the time is known.
#[cfg(not(unix))]
fn run(mut command: Command, _report: &Path, timeout: Duration) -> std::io::Result<Option<Cost>> {
    let start = Instant::now();
    let mut child = command.spawn()?;
    loop {
        if child.try_wait()?.is_some() {
            return Ok(Some(Cost { wall: start.elapsed(), ..Cost::default() }));
        }
        if start.elapsed() > timeout {
            child.kill().ok();
            child.wait().ok();
            return Ok(None);
        }
        std::thread::sleep(Duration::from_millis(1));
    }
}

/// A reader's own account of its time, from `<stem>.time` if it wrote one: lines of
/// a phase and its seconds, `read` and `mesh` among them. Anything else is ignored.
fn phases(times: &Path) -> (Option<Duration>, Option<Duration>) {
    let (mut read, mut mesh) = (None, None);
    for line in std::fs::read_to_string(times).unwrap_or_default().lines() {
        let mut words = line.split_whitespace();
        let (Some(phase), Some(seconds)) = (words.next(), words.next()) else {
            continue;
        };
        let Some(d) = seconds.parse().ok().and_then(|s| Duration::try_from_secs_f64(s).ok()) else {
            continue;
        };
        match phase {
            "read" => read = Some(d),
            "mesh" => mesh = Some(d),
            _ => {}
        }
    }
    (read, mesh)
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
