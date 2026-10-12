# Results

Run 2026-10-12 on Darwin arm64, step-conformance uncommitted.

- opencascade: opencascade-rs 0.3.0 with camcraft's patches, OCCT 7.8.1
- truck: truck-stepio 0.3.0, truck-meshalgo 0.4.0 (crates.io)
- solidcraft: b4491ac 2026-10-12

Each cell: whether the reader passes, how far its mesh lies from the reference
(OpenCascade at 0.001 mm) at worst, and how long it took. See the README for
how a reader is scored.

| part | tolerance mm | opencascade | solidcraft | truck |
|---|---|---|---|---|
| bowl_01 | 0.0125 | ok, 0.042 mm, 1.7 s | ok, 0.012 mm, 0.9 s | timed out at 60 s |
| bowl_01 | 0.05 | ok, 0.050 mm, 0.1 s | ok, 0.047 mm, 0.2 s | 53.498 mm off, 5.0 s |
| bowl_01 | 0.1 | ok, 0.131 mm, 0.1 s | ok, 0.075 mm, 0.1 s | 53.503 mm off, 4.8 s |
| bowl_01 | 0.5 | ok, 0.667 mm, 0.0 s | ok, 0.401 mm, 0.0 s | 53.546 mm off, 0.0 s |
| door_handle_guide | 0.0125 | ok, 0.008 mm, 0.1 s | ok, 0.008 mm, 0.1 s | ok, 0.012 mm, 0.0 s |
| door_handle_guide | 0.05 | ok, 0.020 mm, 0.1 s | ok, 0.034 mm, 0.0 s | ok, 0.044 mm, 0.0 s |
| door_handle_guide | 0.1 | ok, 0.020 mm, 0.1 s | ok, 0.066 mm, 0.0 s | ok, 0.089 mm, 0.0 s |
| door_handle_guide | 0.5 | ok, 0.020 mm, 0.1 s | ok, 0.242 mm, 0.0 s | ok, 0.353 mm, 0.0 s |
| hinge_jig | 0.0125 | ok, 0.000 mm, 0.0 s | ok, 0.000 mm, 0.0 s | ok, 0.000 mm, 0.0 s |
| hinge_jig | 0.05 | ok, 0.000 mm, 0.0 s | ok, 0.000 mm, 0.0 s | ok, 0.000 mm, 0.0 s |
| hinge_jig | 0.1 | ok, 0.000 mm, 0.0 s | ok, 0.000 mm, 0.0 s | ok, 0.000 mm, 0.0 s |
| hinge_jig | 0.5 | ok, 0.000 mm, 0.0 s | ok, 0.000 mm, 0.0 s | ok, 0.000 mm, 0.0 s |
| lego_3_pins | 0.0125 | ok, 0.008 mm, 0.2 s | ok, 0.012 mm, 0.2 s | ok, 0.012 mm, 0.1 s |
| lego_3_pins | 0.05 | ok, 0.030 mm, 0.1 s | ok, 0.049 mm, 0.2 s | ok, 0.048 mm, 0.0 s |
| lego_3_pins | 0.1 | ok, 0.058 mm, 0.1 s | ok, 0.066 mm, 0.2 s | ok, 0.094 mm, 0.1 s |
| lego_3_pins | 0.5 | ok, 0.067 mm, 0.1 s | ok, 0.298 mm, 0.1 s | ok, 0.498 mm, 0.0 s |
| lego_4x2_01 | 0.0125 | ok, 0.008 mm, 0.1 s | ok, 0.012 mm, 0.1 s | ok, 0.012 mm, 0.0 s |
| lego_4x2_01 | 0.05 | ok, 0.030 mm, 0.1 s | ok, 0.049 mm, 0.1 s | ok, 0.048 mm, 0.0 s |
| lego_4x2_01 | 0.1 | ok, 0.058 mm, 0.1 s | ok, 0.066 mm, 0.1 s | ok, 0.094 mm, 0.1 s |
| lego_4x2_01 | 0.5 | ok, 0.067 mm, 0.1 s | ok, 0.298 mm, 0.1 s | ok, 0.498 mm, 0.0 s |
| rail_guide | 0.0125 | ok, 0.007 mm, 0.1 s | ok, 0.012 mm, 0.1 s | ok, 0.012 mm, 0.1 s |
| rail_guide | 0.05 | ok, 0.028 mm, 0.1 s | ok, 0.048 mm, 0.1 s | ok, 0.048 mm, 0.0 s |
| rail_guide | 0.1 | ok, 0.047 mm, 0.1 s | ok, 0.064 mm, 0.1 s | ok, 0.095 mm, 0.0 s |
| rail_guide | 0.5 | ok, 0.069 mm, 0.1 s | ok, 0.483 mm, 0.1 s | ok, 0.483 mm, 0.1 s |
| sled_base | 0.0125 | ok, 0.014 mm, 0.3 s | ok, 0.012 mm, 0.5 s | ok, 0.021 mm, 0.3 s |
| sled_base | 0.05 | ok, 0.052 mm, 0.2 s | ok, 0.049 mm, 0.3 s | ok, 0.084 mm, 0.1 s |
| sled_base | 0.1 | ok, 0.095 mm, 0.2 s | ok, 0.098 mm, 0.4 s | timed out at 60 s |
| sled_base | 0.5 | ok, 0.371 mm, 0.2 s | ok, 0.483 mm, 0.3 s | 4.953 mm off, 0.1 s |

**opencascade** passes 28 of 28.

**solidcraft** passes 28 of 28.

**truck** passes 22 of 28.
- bowl_01 at 0.0125 mm: timed out at 60 s
- bowl_01 at 0.05 mm: 53.498 mm off, 5.0 s
- bowl_01 at 0.1 mm: 53.503 mm off, 4.8 s
- bowl_01 at 0.5 mm: 53.546 mm off, 0.0 s
- sled_base at 0.1 mm: timed out at 60 s
- sled_base at 0.5 mm: 4.953 mm off, 0.1 s
