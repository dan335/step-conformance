#!/usr/bin/env bash
# Build every reader and score it on the corpus, writing results/RESULTS.md.
#
#   ./run.sh                     every reader, on corpus/
#   ./run.sh ../private/parts    on those STEP files too (they are not recorded)
#   SOLIDCRAFT_REV=07e263d TRUCK_REV=<sha> ./run.sh
#
# Clones and builds go under work/, which git ignores. The first build compiles
# OpenCascade from source: it needs cmake and a C++ compiler, and takes minutes.

set -euo pipefail
cd "$(dirname "$0")"

W=work
mkdir -p "$W"
export CMAKE_POLICY_VERSION_MINIMUM=3.5 # OCCT 7.8 asks for a cmake older than 4 allows
SOLIDCRAFT_REV=${SOLIDCRAFT_REV:-main}
TRUCK_REV=${TRUCK_REV:-master}
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

# truck's git master: the same reader, built against the repository.
if [ -d adapters/truck-master ]; then
  build truck-master adapters/truck-master/Cargo.toml "$W/target/truck-master"
  VERSIONS+=("truck-master: $(git ls-remote https://github.com/ricosjp/truck "$TRUCK_REV" | cut -c1-7) ($TRUCK_REV)")
fi

# solidcraft: its own STEP reader, built inside a checkout.
REV=$(clone https://github.com/bherbruck/solidcraft.git "$W/solidcraft" "$SOLIDCRAFT_REV")
mkdir -p "$W/solidcraft/crates/kernel/examples"
cp adapters/solidcraft/camcraft_export.rs "$W/solidcraft/crates/kernel/examples/camcraft_export.rs"
echo "building solidcraft" >&2
(cd "$W/solidcraft" && cargo build --quiet --release -p solvecraft-kernel --example camcraft_export)
VERSIONS+=("solidcraft: $REV")

READERS=(
  --reader "opencascade=$OCCT"
  --reader "solidcraft=$W/solidcraft/target/release/examples/camcraft_export"
  --reader "truck=$W/target/truck/release/truck-reader:file-units"
)
if [ -d adapters/truck-master ]; then
  READERS+=(--reader "truck-master=$W/target/truck-master/release/truck-master-reader:file-units")
fi
for extra in ogeom brepkit; do
  if [ -f "adapters/$extra/Cargo.toml" ]; then
    build "$extra" "adapters/$extra/Cargo.toml" "$W/target/$extra"
    READERS+=(--reader "$extra=$W/target/$extra/release/$extra-reader")
    VERSIONS+=("$extra: $(grep -m1 -E "^$extra" "adapters/$extra/Cargo.toml" | sed 's/ *= */ /')")
  fi
done

echo "scoring" >&2
cargo build --quiet --release
TABLE=$(./target/release/step-conformance --out "$W/out" --reference "reference=$OCCT" \
  "${READERS[@]}" corpus "$@")

if [ $# -eq 0 ]; then
  {
    echo "# Results"
    echo
    echo "Run $(date -u +%Y-%m-%d) on $(uname -sm), step-conformance $(git rev-parse --short HEAD 2>/dev/null || echo uncommitted)."
    echo
    for v in "${VERSIONS[@]}"; do echo "- $v"; done
    echo
    echo "Each cell: whether the reader passes, how far its mesh lies from the reference"
    echo "(OpenCascade at 0.001 mm) at worst, and how long it took. See the README for"
    echo "how a reader is scored."
    echo
    echo "$TABLE"
  } > results/RESULTS.md
  echo "wrote results/RESULTS.md" >&2
else
  echo "$TABLE"
fi
