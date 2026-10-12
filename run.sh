#!/usr/bin/env bash
# Build every reader and score it on the corpus, writing results/RESULTS.md.
#
#   ./run.sh                     every reader, on corpus/
#   ./run.sh ../private/parts    on those STEP files too (they are not recorded)
#   SOLIDCRAFT_REV=07e263d ./run.sh   solidcraft at a given revision
#
# The other readers' versions are pinned in their adapters' Cargo.toml.
#
# Clones and builds go under work/, which git ignores. The first build compiles
# OpenCascade from source: it needs cmake and a C++ compiler, and takes minutes.

set -euo pipefail
cd "$(dirname "$0")"

W=work
mkdir -p "$W"
export CMAKE_POLICY_VERSION_MINIMUM=3.5 # OCCT 7.8 asks for a cmake older than 4 allows
SOLIDCRAFT_REV=${SOLIDCRAFT_REV:-main}
VERSIONS=()

build() { # name manifest target-dir
  echo "building $1" >&2
  cargo build --quiet --release --manifest-path "$2" --target-dir "$3"
}

clone() { # url dir rev
  if [ ! -d "$2/.git" ]; then git clone --quiet "$1" "$2"; fi
  git -C "$2" fetch --quiet origin
  git -C "$2" checkout --quiet --detach "origin/$3" 2>/dev/null || git -C "$2" checkout --quiet --detach "$3"
  git -C "$2" log -1 --format='%h %cs'
}

# OpenCascade: the reference, and a reader.
build opencascade adapters/opencascade/Cargo.toml "$W/target/opencascade"
OCCT="$W/target/opencascade/release/opencascade-reader"
VERSIONS+=("opencascade: opencascade-rs 0.3.0 with camcraft's patches, OCCT 7.8.1")

# truck as released on crates.io.
build truck adapters/truck/Cargo.toml "$W/target/truck"
VERSIONS+=("truck: truck-stepio 0.3.0, truck-meshalgo 0.4.0 (crates.io)")

# truck's git master, which has not been released since September 2024.
build truck-master adapters/truck-master/Cargo.toml "$W/target/truck-master"
VERSIONS+=("truck-master: ricosjp/truck at $(grep -m1 -oE 'rev = "[0-9a-f]{7}' adapters/truck-master/Cargo.toml | cut -c8-)")

# ogeom, and brepkit (AGPL-3.0: built and run here only, never distributed).
build ogeom adapters/ogeom/Cargo.toml "$W/target/ogeom"
VERSIONS+=("ogeom: $(grep -m1 -oE '"=[0-9.]+"' adapters/ogeom/Cargo.toml | tr -d '"=') (crates.io)")
build brepkit adapters/brepkit/Cargo.toml "$W/target/brepkit"
VERSIONS+=("brepkit: $(grep -m1 -oE '"=[0-9.]+"' adapters/brepkit/Cargo.toml | tr -d '"=') (crates.io)")

# solidcraft: its own STEP reader, built inside a checkout.
REV=$(clone https://github.com/bherbruck/solidcraft.git "$W/solidcraft" "$SOLIDCRAFT_REV")
mkdir -p "$W/solidcraft/crates/kernel/examples"
cp adapters/solidcraft/camcraft_export.rs "$W/solidcraft/crates/kernel/examples/camcraft_export.rs"
echo "building solidcraft" >&2
(cd "$W/solidcraft" && cargo build --quiet --release -p solvecraft-kernel --example camcraft_export)
VERSIONS+=("solidcraft: $REV")

READERS=(
  --reader "opencascade=$OCCT"
  --reader "ogeom=$W/target/ogeom/release/ogeom-reader"
  --reader "solidcraft=$W/solidcraft/target/release/examples/camcraft_export"
  --reader "truck=$W/target/truck/release/truck-reader:file-units"
  --reader "truck-master=$W/target/truck-master/release/truck-master-reader:file-units"
  --reader "brepkit=$W/target/brepkit/release/brepkit-reader:file-units"
)

echo "scoring" >&2
cargo build --quiet --release
TABLE=$(./target/release/step-conformance --out "$W/out" --reference "reference=$OCCT" \
  "${READERS[@]}" corpus corpus/generated "$@")

if [ $# -eq 0 ]; then
  {
    echo "# Results"
    echo
    echo "Run $(date -u +%Y-%m-%d) on $(uname -sm), step-conformance $(git describe --always --dirty 2>/dev/null || echo uncommitted)."
    echo
    for v in "${VERSIONS[@]}"; do echo "- $v"; done
    echo
    echo "Each cell: whether the reader passes, how far its mesh lies from the true surface"
    echo "at worst, and how long it took. The true surface is the exact shape for the"
    echo "generated parts, and OpenCascade's mesh at 0.001 mm for the rest. See the README"
    echo "for how a reader is scored."
    echo
    echo "$TABLE"
  } > results/RESULTS.md
  echo "wrote results/RESULTS.md" >&2
else
  echo "$TABLE"
fi
