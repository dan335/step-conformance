# Results

Run 2026-10-12 on Darwin arm64, step-conformance c4e278b-dirty.

- opencascade: opencascade-rs 0.3.0 with camcraft's patches, OCCT 7.8.1
- truck: truck-stepio 0.3.0, truck-meshalgo 0.4.0 (crates.io)
- truck-master: ricosjp/truck at 88ed005
- ogeom: 0.9.16 (crates.io)
- brepkit: 4.1.49 (crates.io)
- solidcraft: f0292b1 2026-10-12

Each cell: whether the reader passes, how far its mesh lies from the true surface
at worst, and how long it took, the median of its repeats where it passes. The
true surface is the exact shape for the generated parts, and OpenCascade's mesh at
0.001 mm for the rest. See the README for how a reader is scored and timed. Speed
and memory follow the failures.

| part | tolerance mm | opencascade | ogeom | solidcraft | truck | truck-master | brepkit |
|---|---|---|---|---|---|---|---|
| bowl_01 | 0.0125 | ok, 0.042 mm, 1.60 s | ok, 0.021 mm, 58 ms | ok, 0.012 mm, 624 ms | timed out at 60 s | 10.000 mm off, 621 ms | could not read: read_step found no solid (it reads MANIFOLD_SOLID_BREP and BREP_WITH_VOIDS only) |
| bowl_01 | 0.05 | ok, 0.050 mm, 62 ms | ok, 0.082 mm, 14 ms | ok, 0.047 mm, 152 ms | 53.498 mm off, 4.86 s | 0.218 mm off, 16 ms | could not read: read_step found no solid (it reads MANIFOLD_SOLID_BREP and BREP_WITH_VOIDS only) |
| bowl_01 | 0.1 | ok, 0.131 mm, 130 ms | ok, 0.105 mm, 8.6 ms | ok, 0.075 mm, 59 ms | 53.503 mm off, 4.73 s | 0.430 mm off, 12 ms | could not read: read_step found no solid (it reads MANIFOLD_SOLID_BREP and BREP_WITH_VOIDS only) |
| bowl_01 | 0.5 | ok, 0.667 mm, 28 ms | ok, 0.416 mm, 5.8 ms | ok, 0.401 mm, 20 ms | 53.546 mm off, 10.0 ms | 2.111 mm off, 8.0 ms | could not read: read_step found no solid (it reads MANIFOLD_SOLID_BREP and BREP_WITH_VOIDS only) |
| door_handle_guide | 0.0125 | ok, 0.008 mm, 49 ms | ok, 0.008 mm, 19 ms | ok, 0.008 mm, 40 ms | ok, 0.012 mm, 17 ms | ok, 0.012 mm, 18 ms | ok, 0.017 mm, 7.5 ms |
| door_handle_guide | 0.05 | ok, 0.020 mm, 45 ms | ok, 0.031 mm, 18 ms | ok, 0.034 mm, 28 ms | ok, 0.044 mm, 17 ms | ok, 0.044 mm, 18 ms | ok, 0.064 mm, 6.7 ms |
| door_handle_guide | 0.1 | ok, 0.020 mm, 46 ms | ok, 0.061 mm, 17 ms | ok, 0.066 mm, 26 ms | ok, 0.089 mm, 16 ms | ok, 0.089 mm, 16 ms | ok, 0.039 mm, 6.1 ms |
| door_handle_guide | 0.5 | ok, 0.020 mm, 45 ms | ok, 0.061 mm, 17 ms | ok, 0.242 mm, 25 ms | ok, 0.353 mm, 17 ms | ok, 0.353 mm, 17 ms | ok, 0.039 mm, 6.7 ms |
| freeform_panel | 0.0125 | ok, 0.020 mm, 1.14 s | 5.468 mm off, 402 ms | ok, 0.014 mm, 6.45 s | timed out at 60 s | 12.172 mm off, 11.2 s | 14.374 mm off, 689 ms |
| freeform_panel | 0.05 | ok, 0.067 mm, 212 ms | 0.607 mm off, 207 ms | 5.306 mm off, 2.83 s | 12.172 mm off, 22.3 s | 12.172 mm off, 1.76 s | 14.374 mm off, 467 ms |
| freeform_panel | 0.1 | ok, 0.130 mm, 123 ms | 0.608 mm off, 145 ms | 5.306 mm off, 2.25 s | 12.172 mm off, 6.19 s | 12.172 mm off, 732 ms | 14.374 mm off, 454 ms |
| freeform_panel | 0.5 | ok, 0.577 mm, 67 ms | ok, 1.187 mm, 117 ms | outside the file by 123.5 mm on +Y, 149.166 mm off | 12.172 mm off, 1.58 s | 12.172 mm off, 133 ms | nothing of the mesh lies on the file's face #2114; a missing face is a hole in the part, 31.325 mm off |
| hinge_jig | 0.0125 | ok, 0.000 mm, 22 ms | ok, 0.000 mm, 4.9 ms | ok, 0.000 mm, 6.2 ms | ok, 0.000 mm, 8.6 ms | ok, 0.000 mm, 8.6 ms | ok, 0.000 mm, 3.3 ms |
| hinge_jig | 0.05 | ok, 0.000 mm, 22 ms | ok, 0.000 mm, 4.7 ms | ok, 0.000 mm, 6.1 ms | ok, 0.000 mm, 8.5 ms | ok, 0.000 mm, 8.6 ms | ok, 0.000 mm, 3.4 ms |
| hinge_jig | 0.1 | ok, 0.000 mm, 23 ms | ok, 0.000 mm, 4.8 ms | ok, 0.000 mm, 6.4 ms | ok, 0.000 mm, 8.8 ms | ok, 0.000 mm, 8.7 ms | ok, 0.000 mm, 3.3 ms |
| hinge_jig | 0.5 | ok, 0.000 mm, 23 ms | ok, 0.000 mm, 5.0 ms | ok, 0.000 mm, 6.7 ms | ok, 0.000 mm, 8.5 ms | ok, 0.000 mm, 8.7 ms | ok, 0.000 mm, 3.3 ms |
| lego_3_pins | 0.0125 | ok, 0.008 mm, 157 ms | ok, 0.011 mm, 58 ms | ok, 0.012 mm, 154 ms | ok, 0.012 mm, 42 ms | ok, 0.012 mm, 42 ms | ok, 0.017 mm, 37 ms |
| lego_3_pins | 0.05 | ok, 0.030 mm, 103 ms | ok, 0.044 mm, 55 ms | ok, 0.049 mm, 100 ms | ok, 0.048 mm, 35 ms | ok, 0.048 mm, 37 ms | ok, 0.096 mm, 24 ms |
| lego_3_pins | 0.1 | ok, 0.058 mm, 93 ms | ok, 0.061 mm, 54 ms | ok, 0.066 mm, 96 ms | ok, 0.094 mm, 36 ms | ok, 0.094 mm, 34 ms | ok, 0.183 mm, 23 ms |
| lego_3_pins | 0.5 | ok, 0.067 mm, 95 ms | ok, 0.177 mm, 57 ms | ok, 0.298 mm, 85 ms | ok, 0.498 mm, 31 ms | ok, 0.498 mm, 32 ms | ok, 0.140 mm, 22 ms |
| lego_4x2_01 | 0.0125 | ok, 0.008 mm, 65 ms | ok, 0.011 mm, 25 ms | ok, 0.012 mm, 54 ms | ok, 0.012 mm, 19 ms | ok, 0.012 mm, 20 ms | ok, 0.017 mm, 15 ms |
| lego_4x2_01 | 0.05 | ok, 0.030 mm, 47 ms | ok, 0.044 mm, 24 ms | ok, 0.049 mm, 37 ms | ok, 0.048 mm, 16 ms | ok, 0.048 mm, 16 ms | ok, 0.096 mm, 11 ms |
| lego_4x2_01 | 0.1 | ok, 0.058 mm, 43 ms | ok, 0.061 mm, 23 ms | ok, 0.066 mm, 37 ms | ok, 0.094 mm, 15 ms | ok, 0.094 mm, 15 ms | ok, 0.183 mm, 9.5 ms |
| lego_4x2_01 | 0.5 | ok, 0.067 mm, 39 ms | ok, 0.177 mm, 22 ms | ok, 0.298 mm, 31 ms | ok, 0.498 mm, 15 ms | ok, 0.498 mm, 15 ms | ok, 0.140 mm, 11 ms |
| rail_guide | 0.0125 | ok, 0.007 mm, 77 ms | ok, 0.013 mm, 30 ms | ok, 0.012 mm, 72 ms | ok, 0.012 mm, 25 ms | ok, 0.012 mm, 25 ms | ok, 0.014 mm, 11 ms |
| rail_guide | 0.05 | ok, 0.028 mm, 65 ms | ok, 0.048 mm, 27 ms | ok, 0.048 mm, 54 ms | ok, 0.048 mm, 31 ms | ok, 0.048 mm, 24 ms | ok, 0.039 mm, 9.4 ms |
| rail_guide | 0.1 | ok, 0.047 mm, 62 ms | ok, 0.055 mm, 26 ms | ok, 0.064 mm, 48 ms | ok, 0.095 mm, 23 ms | ok, 0.095 mm, 23 ms | ok, 0.078 mm, 9.1 ms |
| rail_guide | 0.5 | ok, 0.069 mm, 60 ms | ok, 0.122 mm, 27 ms | ok, 0.483 mm, 44 ms | ok, 0.483 mm, 21 ms | ok, 0.483 mm, 21 ms | ok, 0.078 mm, 7.8 ms |
| sled_base | 0.0125 | ok, 0.014 mm, 240 ms | ok, 0.017 mm, 74 ms | ok, 0.012 mm, 319 ms | ok, 0.021 mm, 136 ms | ok, 0.043 mm, 77 ms | 3.792 mm off, 48 ms |
| sled_base | 0.05 | ok, 0.052 mm, 162 ms | ok, 0.067 mm, 68 ms | ok, 0.049 mm, 247 ms | ok, 0.084 mm, 73 ms | 0.818 mm off, 55 ms | 4.149 mm off, 28 ms |
| sled_base | 0.1 | ok, 0.095 mm, 165 ms | ok, 0.119 mm, 72 ms | ok, 0.098 mm, 238 ms | timed out at 60 s | ok, 0.223 mm, 54 ms | 4.111 mm off, 26 ms |
| sled_base | 0.5 | ok, 0.371 mm, 148 ms | ok, 0.252 mm, 67 ms | ok, 0.483 mm, 217 ms | 4.953 mm off, 56 ms | ok, 0.621 mm, 49 ms | face 357 empty, 5.959 mm off |
| cone_point | 0.0125 | ok, 0.011 mm, 19 ms | face 1 lies 0.080 mm off every face of the file it could be; its surface was evaluated wrongly, 0.092 mm off | 30.000 mm off, 8.1 ms | outside the file by 0.3 mm on +Y, 30.000 mm off | outside the file by 0.3 mm on +Y, 30.000 mm off | 14.813 mm off, 4.0 ms |
| cone_point | 0.05 | ok, 0.039 mm, 16 ms | ok, 0.137 mm, 4.4 ms | 30.000 mm off, 6.9 ms | outside the file by 0.3 mm on +Y, 30.000 mm off | outside the file by 0.3 mm on +Y, 30.000 mm off | 14.813 mm off, 3.4 ms |
| cone_point | 0.1 | ok, 0.073 mm, 16 ms | ok, 0.235 mm, 4.5 ms | 30.000 mm off, 6.8 ms | outside the file by 0.2 mm on +Y, 30.000 mm off | outside the file by 0.2 mm on +Y, 30.000 mm off | face 1 empty, 30.000 mm off |
| cone_point | 0.5 | ok, 0.205 mm, 15 ms | ok, 0.256 mm, 3.9 ms | 30.000 mm off, 14 ms | 30.000 mm off, 8.2 ms | 30.000 mm off, 6.8 ms | face 1 empty, 30.000 mm off |
| cone_point_vertex | 0.0125 | ok, 0.011 mm, 20 ms | face 1 lies 0.080 mm off every face of the file it could be; its surface was evaluated wrongly, 0.092 mm off | short of the vertices by 30.0 mm on +Z, 30.000 mm off | outside the file by 0.3 mm on +Y, 30.000 mm off | outside the file by 0.3 mm on +Y, 30.000 mm off | could not read: parse error: failed to create wire from edge loop #44: empty wire — at least one |
| cone_point_vertex | 0.05 | ok, 0.039 mm, 15 ms | ok, 0.137 mm, 4.5 ms | short of the vertices by 29.9 mm on +Z, 30.000 mm off | outside the file by 0.3 mm on +Y, 30.000 mm off | outside the file by 0.3 mm on +Y, 30.000 mm off | could not read: parse error: failed to create wire from edge loop #44: empty wire — at least one |
| cone_point_vertex | 0.1 | ok, 0.073 mm, 15 ms | ok, 0.235 mm, 4.1 ms | short of the vertices by 29.9 mm on +Z, 30.000 mm off | outside the file by 0.2 mm on +Y, 30.000 mm off | outside the file by 0.2 mm on +Y, 30.000 mm off | could not read: parse error: failed to create wire from edge loop #44: empty wire — at least one |
| cone_point_vertex | 0.5 | ok, 0.205 mm, 14 ms | ok, 0.256 mm, 3.7 ms | short of the vertices by 29.5 mm on +Z, 30.000 mm off | short of the vertices by 29.5 mm on +Z, 30.000 mm off | short of the vertices by 29.5 mm on +Z, 30.000 mm off | could not read: parse error: failed to create wire from edge loop #44: empty wire — at least one |
| cylinder | 0.0125 | ok, 0.006 mm, 15 ms | ok, 0.012 mm, 4.0 ms | ok, 0.012 mm, 9.3 ms | ok, 0.012 mm, 5.5 ms | ok, 0.012 mm, 5.2 ms | 14.469 mm off, 4.0 ms |
| cylinder | 0.05 | ok, 0.024 mm, 15 ms | ok, 0.048 mm, 3.5 ms | ok, 0.050 mm, 6.5 ms | ok, 0.048 mm, 5.0 ms | ok, 0.048 mm, 4.8 ms | 14.469 mm off, 3.3 ms |
| cylinder | 0.1 | ok, 0.048 mm, 14 ms | ok, 0.048 mm, 3.4 ms | ok, 0.053 mm, 6.1 ms | ok, 0.093 mm, 4.7 ms | ok, 0.093 mm, 4.6 ms | 14.469 mm off, 3.0 ms |
| cylinder | 0.5 | ok, 0.073 mm, 15 ms | ok, 0.192 mm, 4.2 ms | ok, 0.218 mm, 6.5 ms | ok, 0.489 mm, 6.5 ms | ok, 0.489 mm, 6.3 ms | 14.470 mm off, 3.7 ms |
| cylinder_inch | 0.0125 | ok, 0.006 mm, 17 ms | ok, 0.012 mm, 5.3 ms | ok, 0.012 mm, 9.6 ms | ok, 0.012 mm, 5.6 ms | ok, 0.012 mm, 5.6 ms | outside the file by 15.4 mm on -Z, 25.400 mm off |
| cylinder_inch | 0.05 | ok, 0.024 mm, 15 ms | ok, 0.048 mm, 3.5 ms | ok, 0.050 mm, 7.1 ms | ok, 0.048 mm, 5.0 ms | ok, 0.048 mm, 4.8 ms | outside the file by 15.3 mm on -Z, 25.400 mm off |
| cylinder_inch | 0.1 | ok, 0.048 mm, 15 ms | ok, 0.048 mm, 3.5 ms | ok, 0.053 mm, 6.2 ms | ok, 0.093 mm, 5.0 ms | ok, 0.093 mm, 4.7 ms | outside the file by 15.3 mm on -Z, 25.400 mm off |
| cylinder_inch | 0.5 | ok, 0.073 mm, 15 ms | ok, 0.192 mm, 3.9 ms | ok, 0.218 mm, 6.3 ms | ok, 0.489 mm, 5.6 ms | ok, 0.489 mm, 5.5 ms | outside the file by 14.9 mm on -Z, 25.400 mm off |
| cylinder_nurbs | 0.0125 | ok, 0.006 mm, 19 ms | face 1 empty, 15.000 mm off | ok, 0.012 mm, 16 ms | ok, 0.012 mm, 55 ms | ok, 0.012 mm, 34 ms | ok, 0.013 mm, 6.6 ms |
| cylinder_nurbs | 0.05 | ok, 0.025 mm, 16 ms | face 1 empty, 15.000 mm off | ok, 0.046 mm, 8.7 ms | ok, 0.050 mm, 16 ms | ok, 0.050 mm, 10 ms | ok, 0.040 mm, 4.6 ms |
| cylinder_nurbs | 0.1 | ok, 0.049 mm, 15 ms | face 1 empty, 15.000 mm off | ok, 0.053 mm, 7.1 ms | ok, 0.053 mm, 8.7 ms | ok, 0.053 mm, 7.8 ms | ok, 0.040 mm, 4.6 ms |
| cylinder_nurbs | 0.5 | ok, 0.077 mm, 16 ms | face 1 empty, 15.000 mm off | ok, 0.207 mm, 7.5 ms | ok, 0.207 mm, 8.3 ms | ok, 0.207 mm, 7.1 ms | ok, 0.040 mm, 4.5 ms |
| cylinder_placed | 0.0125 | ok, 0.006 mm, 16 ms | ok, 0.012 mm, 4.5 ms | ok, 0.012 mm, 9.5 ms | outside the file by 40.0 mm on -X, 56.630 mm off | ok, 0.012 mm, 6.2 ms | outside the file by 40.0 mm on -X, 56.957 mm off |
| cylinder_placed | 0.05 | ok, 0.024 mm, 16 ms | ok, 0.048 mm, 3.6 ms | ok, 0.050 mm, 7.2 ms | outside the file by 39.9 mm on -X, 56.630 mm off | ok, 0.048 mm, 6.6 ms | outside the file by 39.9 mm on -X, 56.948 mm off |
| cylinder_placed | 0.1 | ok, 0.048 mm, 16 ms | ok, 0.048 mm, 4.4 ms | ok, 0.053 mm, 6.8 ms | outside the file by 39.8 mm on -X, 56.667 mm off | ok, 0.093 mm, 6.3 ms | outside the file by 39.9 mm on -X, 57.025 mm off |
| cylinder_placed | 0.5 | ok, 0.073 mm, 16 ms | ok, 0.192 mm, 4.1 ms | ok, 0.218 mm, 6.1 ms | outside the file by 39.5 mm on -X, 56.851 mm off | ok, 0.489 mm, 4.9 ms | outside the file by 39.3 mm on -X, 56.947 mm off |
| cylinder_seam | 0.0125 | ok, 0.006 mm, 15 ms | ok, 0.012 mm, 4.0 ms | ok, 0.012 mm, 8.9 ms | ok, 0.012 mm, 5.5 ms | ok, 0.012 mm, 5.5 ms | ok, 0.012 mm, 3.4 ms |
| cylinder_seam | 0.05 | ok, 0.024 mm, 15 ms | ok, 0.048 mm, 3.5 ms | ok, 0.048 mm, 6.5 ms | ok, 0.048 mm, 5.0 ms | ok, 0.048 mm, 4.9 ms | ok, 0.048 mm, 3.2 ms |
| cylinder_seam | 0.1 | ok, 0.048 mm, 14 ms | ok, 0.048 mm, 3.5 ms | ok, 0.053 mm, 6.1 ms | ok, 0.093 mm, 5.1 ms | ok, 0.093 mm, 5.1 ms | ok, 0.093 mm, 2.9 ms |
| cylinder_seam | 0.5 | ok, 0.073 mm, 14 ms | ok, 0.192 mm, 3.4 ms | ok, 0.207 mm, 6.1 ms | ok, 0.489 mm, 4.6 ms | ok, 0.489 mm, 4.9 ms | ok, 0.152 mm, 3.1 ms |
| cylinder_twice | 0.0125 | ok, 0.006 mm, 16 ms | ok, 0.012 mm, 5.4 ms | ok, 0.012 mm, 13 ms | short of the vertices by 80.0 mm on +X, 80.555 mm off | ok, 0.012 mm, 5.8 ms | short of the vertices by 80.0 mm on +X, 81.677 mm off |
| cylinder_twice | 0.05 | ok, 0.024 mm, 14 ms | ok, 0.048 mm, 4.0 ms | ok, 0.050 mm, 9.9 ms | short of the vertices by 79.9 mm on +X, 80.595 mm off | ok, 0.048 mm, 5.9 ms | short of the vertices by 79.9 mm on +X, 81.716 mm off |
| cylinder_twice | 0.1 | ok, 0.048 mm, 14 ms | ok, 0.048 mm, 3.9 ms | ok, 0.053 mm, 7.9 ms | short of the vertices by 79.9 mm on +X, 80.623 mm off | ok, 0.093 mm, 5.2 ms | short of the vertices by 79.9 mm on +X, 81.741 mm off |
| cylinder_twice | 0.5 | ok, 0.073 mm, 14 ms | ok, 0.192 mm, 3.5 ms | ok, 0.218 mm, 6.7 ms | short of the vertices by 79.5 mm on +X, 80.623 mm off | ok, 0.489 mm, 5.0 ms | short of the vertices by 79.7 mm on +X, 81.708 mm off |
| dome | 0.0125 | ok, 0.012 mm, 42 ms | face 1 lies 0.064 mm off every face of the file it could be; its surface was evaluated wrongly, 20.000 mm off | ok, 0.012 mm, 49 ms | ok, 0.024 mm, 10 ms | ok, 0.024 mm, 9.5 ms | ok, 0.012 mm, 51 ms |
| dome | 0.05 | ok, 0.048 mm, 18 ms | 20.000 mm off, 7.1 ms | ok, 0.047 mm, 17 ms | ok, 0.091 mm, 5.9 ms | ok, 0.091 mm, 5.7 ms | ok, 0.049 mm, 13 ms |
| dome | 0.1 | ok, 0.083 mm, 17 ms | 20.000 mm off, 5.4 ms | ok, 0.093 mm, 14 ms | ok, 0.191 mm, 5.4 ms | ok, 0.191 mm, 5.3 ms | ok, 0.096 mm, 7.8 ms |
| dome | 0.5 | ok, 0.345 mm, 14 ms | 20.000 mm off, 4.6 ms | ok, 0.415 mm, 7.4 ms | ok, 0.797 mm, 4.7 ms | ok, 0.797 mm, 4.8 ms | ok, 0.304 mm, 3.6 ms |
| dome_nurbs | 0.0125 | ok, 0.013 mm, 128 ms | face 1 empty, 20.000 mm off | ok, 0.012 mm, 55 ms | ok, 0.008 mm, 61 ms | ok, 0.008 mm, 35 ms | 3.906 mm off, 71 ms |
| dome_nurbs | 0.05 | ok, 0.050 mm, 37 ms | face 1 empty, 20.000 mm off | ok, 0.047 mm, 18 ms | ok, 0.032 mm, 17 ms | ok, 0.032 mm, 11 ms | 3.907 mm off, 20 ms |
| dome_nurbs | 0.1 | ok, 0.087 mm, 26 ms | face 1 empty, 20.000 mm off | ok, 0.093 mm, 14 ms | ok, 0.131 mm, 13 ms | ok, 0.100 mm, 10 ms | 3.913 mm off, 7.6 ms |
| dome_nurbs | 0.5 | ok, 0.400 mm, 17 ms | face 1 empty, 20.000 mm off | ok, 0.415 mm, 7.9 ms | ok, 0.538 mm, 5.7 ms | ok, 0.538 mm, 5.3 ms | 3.938 mm off, 4.5 ms |
| far_cylinder | 0.0125 | ok, 0.006 mm, 15 ms | ok, 0.012 mm, 4.2 ms | ok, 0.012 mm, 8.4 ms | ok, 0.012 mm, 5.8 ms | ok, 0.012 mm, 5.3 ms | 14.469 mm off, 3.7 ms |
| far_cylinder | 0.05 | ok, 0.024 mm, 15 ms | ok, 0.048 mm, 3.8 ms | ok, 0.046 mm, 8.1 ms | ok, 0.048 mm, 5.3 ms | ok, 0.048 mm, 5.3 ms | 14.469 mm off, 3.5 ms |
| far_cylinder | 0.1 | ok, 0.048 mm, 15 ms | ok, 0.048 mm, 3.6 ms | ok, 0.053 mm, 6.3 ms | ok, 0.093 mm, 5.2 ms | ok, 0.093 mm, 5.4 ms | 14.469 mm off, 3.4 ms |
| far_cylinder | 0.5 | ok, 0.073 mm, 14 ms | ok, 0.192 mm, 3.6 ms | ok, 0.216 mm, 6.2 ms | ok, 0.489 mm, 4.9 ms | ok, 0.489 mm, 4.9 ms | 14.470 mm off, 2.9 ms |
| grooved_shaft | 0.0125 | ok, 0.006 mm, 523 ms | ok, 0.012 mm, 305 ms | ok, 0.012 mm, 662 ms | ok, 0.012 mm, 109 ms | ok, 0.012 mm, 111 ms | ok, 0.012 mm, 118 ms |
| grooved_shaft | 0.05 | ok, 0.024 mm, 317 ms | ok, 0.048 mm, 284 ms | ok, 0.046 mm, 494 ms | ok, 0.048 mm, 91 ms | ok, 0.048 mm, 95 ms | ok, 0.048 mm, 68 ms |
| grooved_shaft | 0.1 | ok, 0.048 mm, 266 ms | ok, 0.048 mm, 286 ms | ok, 0.053 mm, 429 ms | ok, 0.093 mm, 86 ms | ok, 0.093 mm, 89 ms | ok, 0.093 mm, 58 ms |
| grooved_shaft | 0.5 | ok, 0.073 mm, 331 ms | ok, 0.189 mm, 291 ms | ok, 0.207 mm, 400 ms | ok, 0.489 mm, 84 ms | ok, 0.489 mm, 86 ms | ok, 0.152 mm, 49 ms |
| plate_100_holes | 0.0125 | ok, 0.005 mm, 346 ms | ok, 0.005 mm, 94 ms | ok, 0.005 mm, 465 ms | ok, 0.012 mm, 122 ms | ok, 0.012 mm, 126 ms | ok, 0.012 mm, 67 ms |
| plate_100_holes | 0.05 | ok, 0.022 mm, 134 ms | ok, 0.019 mm, 56 ms | ok, 0.020 mm, 214 ms | ok, 0.049 mm, 52 ms | ok, 0.049 mm, 54 ms | ok, 0.046 mm, 33 ms |
| plate_100_holes | 0.1 | ok, 0.025 mm, 135 ms | ok, 0.077 mm, 32 ms | ok, 0.082 mm, 100 ms | ok, 0.087 mm, 42 ms | ok, 0.087 mm, 41 ms | ok, 0.046 mm, 33 ms |
| plate_100_holes | 0.5 | ok, 0.025 mm, 134 ms | ok, 0.077 mm, 32 ms | ok, 0.303 mm, 76 ms | ok, 0.402 mm, 27 ms | ok, 0.402 mm, 27 ms | ok, 0.046 mm, 33 ms |
| plate_900_holes | 0.0125 | ok, 0.006 mm, 13.5 s | ok, 0.004 mm, 477 ms | ok, 0.005 mm, 19.9 s | ok, 0.012 mm, 5.72 s | ok, 0.012 mm, 5.89 s | ok, 0.012 mm, 2.75 s |
| plate_900_holes | 0.05 | ok, 0.022 mm, 5.33 s | ok, 0.017 mm, 289 ms | ok, 0.018 mm, 5.40 s | ok, 0.049 mm, 1.50 s | ok, 0.049 mm, 1.60 s | ok, 0.046 mm, 1.02 s |
| plate_900_holes | 0.1 | ok, 0.025 mm, 5.34 s | ok, 0.077 mm, 186 ms | ok, 0.083 mm, 1.72 s | ok, 0.087 mm, 901 ms | ok, 0.087 mm, 956 ms | ok, 0.046 mm, 1.03 s |
| plate_900_holes | 0.5 | ok, 0.025 mm, 5.50 s | ok, 0.077 mm, 193 ms | ok, 0.304 mm, 869 ms | ok, 0.401 mm, 363 ms | ok, 0.401 mm, 378 ms | ok, 0.046 mm, 1.48 s |
| sheet_disc | 0.0125 | ok, 0.006 mm, 94 ms | ok, 0.008 mm, 45 ms | ok, 0.009 mm, 100 ms | ok, 0.012 mm, 44 ms | ok, 0.012 mm, 37 ms | 8.906 mm off, 62 ms |
| sheet_disc | 0.05 | ok, 0.025 mm, 40 ms | ok, 0.030 mm, 26 ms | ok, 0.037 mm, 82 ms | ok, 0.050 mm, 27 ms | ok, 0.050 mm, 31 ms | 8.906 mm off, 74 ms |
| sheet_disc | 0.1 | ok, 0.050 mm, 31 ms | ok, 0.030 mm, 35 ms | ok, 0.037 mm, 84 ms | ok, 0.099 mm, 19 ms | ok, 0.099 mm, 24 ms | 8.907 mm off, 19 ms |
| sheet_disc | 0.5 | ok, 0.249 mm, 24 ms | ok, 0.120 mm, 29 ms | ok, 0.443 mm, 25 ms | ok, 0.495 mm, 21 ms | ok, 0.495 mm, 27 ms | 8.920 mm off, 11 ms |
| tiny_pin | 0.0125 | ok, 0.002 mm, 21 ms | ok, 0.005 mm, 10 ms | ok, 0.005 mm, 40 ms | ok, 0.012 mm, 21 ms | ok, 0.012 mm, 14 ms | outside the file by 0.7 mm on -Z, 1.000 mm off |
| tiny_pin | 0.05 | ok, 0.002 mm, 17 ms | ok, 0.005 mm, 5.5 ms | ok, 0.024 mm, 18 ms | ok, 0.048 mm, 10 ms | ok, 0.048 mm, 6.9 ms | face 0 empty, 1000.000 mm off |
| tiny_pin | 0.1 | ok, 0.002 mm, 15 ms | ok, 0.005 mm, 4.0 ms | ok, 0.073 mm, 6.2 ms | ok, 0.073 mm, 5.2 ms | ok, 0.073 mm, 5.0 ms | face 0 empty, 1000.000 mm off |
| tiny_pin | 0.5 | ok, 0.002 mm, 14 ms | ok, 0.005 mm, 3.3 ms | outside the file by 0.2 mm on +Y, 1.164 mm off | ok, 0.125 mm, 5.2 ms | ok, 0.125 mm, 5.1 ms | face 0 empty, 1000.000 mm off |
| torus_ring | 0.0125 | ok, 0.016 mm, 62 ms | ok, 0.014 mm, 28 ms | face 1 lies 3.482 mm off every face of the file it could be; its surface was evaluated wrongly, 5.000 mm off | ok, 0.024 mm, 26 ms | ok, 0.024 mm, 24 ms | ok, 0.012 mm, 17 ms |
| torus_ring | 0.05 | ok, 0.062 mm, 24 ms | ok, 0.054 mm, 10 ms | face 1 lies 3.391 mm off every face of the file it could be; its surface was evaluated wrongly, 5.000 mm off | ok, 0.091 mm, 7.5 ms | ok, 0.091 mm, 7.2 ms | ok, 0.053 mm, 6.2 ms |
| torus_ring | 0.1 | ok, 0.108 mm, 19 ms | ok, 0.125 mm, 5.8 ms | 5.000 mm off, 27 ms | ok, 0.189 mm, 6.5 ms | ok, 0.189 mm, 6.0 ms | ok, 0.105 mm, 4.7 ms |
| torus_ring | 0.5 | ok, 0.213 mm, 16 ms | ok, 0.564 mm, 4.8 ms | face 1 lies 5.000 mm off every face of the file it could be; its surface was evaluated wrongly, 11.352 mm off | ok, 0.813 mm, 5.5 ms | ok, 0.813 mm, 5.2 ms | ok, 0.390 mm, 3.0 ms |
| torus_ring_nurbs | 0.0125 | ok, 0.017 mm, 167 ms | face 1 empty, 5.000 mm off | ok, 0.008 mm, 96 ms | ok, 0.009 mm, 87 ms | ok, 0.010 mm, 42 ms | 0.132 mm off, 69 ms |
| torus_ring_nurbs | 0.05 | ok, 0.054 mm, 39 ms | face 1 empty, 5.000 mm off | ok, 0.033 mm, 27 ms | ok, 0.037 mm, 23 ms | ok, 0.041 mm, 13 ms | 0.362 mm off, 21 ms |
| torus_ring_nurbs | 0.1 | ok, 0.101 mm, 27 ms | face 1 empty, 5.000 mm off | ok, 0.088 mm, 22 ms | ok, 0.088 mm, 19 ms | ok, 0.088 mm, 13 ms | 0.630 mm off, 8.2 ms |
| torus_ring_nurbs | 0.5 | ok, 0.414 mm, 18 ms | face 1 empty, 5.000 mm off | ok, 0.441 mm, 9.1 ms | ok, 0.614 mm, 7.9 ms | ok, 0.441 mm, 6.5 ms | ok, 0.675 mm, 6.2 ms |
| washer | 0.0125 | ok, 0.006 mm, 18 ms | ok, 0.006 mm, 4.4 ms | ok, 0.007 mm, 9.3 ms | ok, 0.012 mm, 5.8 ms | ok, 0.012 mm, 5.6 ms | 1.000 mm off, 5.2 ms |
| washer | 0.05 | ok, 0.025 mm, 16 ms | ok, 0.024 mm, 4.1 ms | ok, 0.027 mm, 8.0 ms | ok, 0.049 mm, 5.3 ms | ok, 0.049 mm, 5.1 ms | 1.000 mm off, 4.2 ms |
| washer | 0.1 | ok, 0.049 mm, 15 ms | ok, 0.096 mm, 3.8 ms | ok, 0.093 mm, 7.5 ms | ok, 0.096 mm, 5.3 ms | ok, 0.096 mm, 5.0 ms | 1.000 mm off, 3.9 ms |
| washer | 0.5 | ok, 0.146 mm, 15 ms | ok, 0.384 mm, 3.8 ms | ok, 0.415 mm, 6.5 ms | ok, 0.495 mm, 5.6 ms | ok, 0.495 mm, 4.9 ms | ok, 1.030 mm, 3.8 ms |

**opencascade** passes 108 of 108.

**ogeom** passes 87 of 108.
- freeform_panel at 0.0125 mm: 5.468 mm off, 402 ms
- freeform_panel at 0.05 mm: 0.607 mm off, 207 ms
- freeform_panel at 0.1 mm: 0.608 mm off, 145 ms
- cone_point at 0.0125 mm: face 1 lies 0.080 mm off every face of the file it could be; its surface was evaluated wrongly, 0.092 mm off
- cone_point_vertex at 0.0125 mm: face 1 lies 0.080 mm off every face of the file it could be; its surface was evaluated wrongly, 0.092 mm off
- cylinder_nurbs at 0.0125 mm: face 1 empty, 15.000 mm off
- cylinder_nurbs at 0.05 mm: face 1 empty, 15.000 mm off
- cylinder_nurbs at 0.1 mm: face 1 empty, 15.000 mm off
- cylinder_nurbs at 0.5 mm: face 1 empty, 15.000 mm off
- dome at 0.0125 mm: face 1 lies 0.064 mm off every face of the file it could be; its surface was evaluated wrongly, 20.000 mm off
- dome at 0.05 mm: 20.000 mm off, 7.1 ms
- dome at 0.1 mm: 20.000 mm off, 5.4 ms
- dome at 0.5 mm: 20.000 mm off, 4.6 ms
- dome_nurbs at 0.0125 mm: face 1 empty, 20.000 mm off
- dome_nurbs at 0.05 mm: face 1 empty, 20.000 mm off
- dome_nurbs at 0.1 mm: face 1 empty, 20.000 mm off
- dome_nurbs at 0.5 mm: face 1 empty, 20.000 mm off
- torus_ring_nurbs at 0.0125 mm: face 1 empty, 5.000 mm off
- torus_ring_nurbs at 0.05 mm: face 1 empty, 5.000 mm off
- torus_ring_nurbs at 0.1 mm: face 1 empty, 5.000 mm off
- torus_ring_nurbs at 0.5 mm: face 1 empty, 5.000 mm off

**solidcraft** passes 92 of 108.
- freeform_panel at 0.05 mm: 5.306 mm off, 2.83 s
- freeform_panel at 0.1 mm: 5.306 mm off, 2.25 s
- freeform_panel at 0.5 mm: outside the file by 123.5 mm on +Y, 149.166 mm off
- cone_point at 0.0125 mm: 30.000 mm off, 8.1 ms
- cone_point at 0.05 mm: 30.000 mm off, 6.9 ms
- cone_point at 0.1 mm: 30.000 mm off, 6.8 ms
- cone_point at 0.5 mm: 30.000 mm off, 14 ms
- cone_point_vertex at 0.0125 mm: short of the vertices by 30.0 mm on +Z, 30.000 mm off
- cone_point_vertex at 0.05 mm: short of the vertices by 29.9 mm on +Z, 30.000 mm off
- cone_point_vertex at 0.1 mm: short of the vertices by 29.9 mm on +Z, 30.000 mm off
- cone_point_vertex at 0.5 mm: short of the vertices by 29.5 mm on +Z, 30.000 mm off
- tiny_pin at 0.5 mm: outside the file by 0.2 mm on +Y, 1.164 mm off
- torus_ring at 0.0125 mm: face 1 lies 3.482 mm off every face of the file it could be; its surface was evaluated wrongly, 5.000 mm off
- torus_ring at 0.05 mm: face 1 lies 3.391 mm off every face of the file it could be; its surface was evaluated wrongly, 5.000 mm off
- torus_ring at 0.1 mm: 5.000 mm off, 27 ms
- torus_ring at 0.5 mm: face 1 lies 5.000 mm off every face of the file it could be; its surface was evaluated wrongly, 11.352 mm off

**truck** passes 82 of 108.
- bowl_01 at 0.0125 mm: timed out at 60 s
- bowl_01 at 0.05 mm: 53.498 mm off, 4.86 s
- bowl_01 at 0.1 mm: 53.503 mm off, 4.73 s
- bowl_01 at 0.5 mm: 53.546 mm off, 10.0 ms
- freeform_panel at 0.0125 mm: timed out at 60 s
- freeform_panel at 0.05 mm: 12.172 mm off, 22.3 s
- freeform_panel at 0.1 mm: 12.172 mm off, 6.19 s
- freeform_panel at 0.5 mm: 12.172 mm off, 1.58 s
- sled_base at 0.1 mm: timed out at 60 s
- sled_base at 0.5 mm: 4.953 mm off, 56 ms
- cone_point at 0.0125 mm: outside the file by 0.3 mm on +Y, 30.000 mm off
- cone_point at 0.05 mm: outside the file by 0.3 mm on +Y, 30.000 mm off
- cone_point at 0.1 mm: outside the file by 0.2 mm on +Y, 30.000 mm off
- cone_point at 0.5 mm: 30.000 mm off, 8.2 ms
- cone_point_vertex at 0.0125 mm: outside the file by 0.3 mm on +Y, 30.000 mm off
- cone_point_vertex at 0.05 mm: outside the file by 0.3 mm on +Y, 30.000 mm off
- cone_point_vertex at 0.1 mm: outside the file by 0.2 mm on +Y, 30.000 mm off
- cone_point_vertex at 0.5 mm: short of the vertices by 29.5 mm on +Z, 30.000 mm off
- cylinder_placed at 0.0125 mm: outside the file by 40.0 mm on -X, 56.630 mm off
- cylinder_placed at 0.05 mm: outside the file by 39.9 mm on -X, 56.630 mm off
- cylinder_placed at 0.1 mm: outside the file by 39.8 mm on -X, 56.667 mm off
- cylinder_placed at 0.5 mm: outside the file by 39.5 mm on -X, 56.851 mm off
- cylinder_twice at 0.0125 mm: short of the vertices by 80.0 mm on +X, 80.555 mm off
- cylinder_twice at 0.05 mm: short of the vertices by 79.9 mm on +X, 80.595 mm off
- cylinder_twice at 0.1 mm: short of the vertices by 79.9 mm on +X, 80.623 mm off
- cylinder_twice at 0.5 mm: short of the vertices by 79.5 mm on +X, 80.623 mm off

**truck-master** passes 91 of 108.
- bowl_01 at 0.0125 mm: 10.000 mm off, 621 ms
- bowl_01 at 0.05 mm: 0.218 mm off, 16 ms
- bowl_01 at 0.1 mm: 0.430 mm off, 12 ms
- bowl_01 at 0.5 mm: 2.111 mm off, 8.0 ms
- freeform_panel at 0.0125 mm: 12.172 mm off, 11.2 s
- freeform_panel at 0.05 mm: 12.172 mm off, 1.76 s
- freeform_panel at 0.1 mm: 12.172 mm off, 732 ms
- freeform_panel at 0.5 mm: 12.172 mm off, 133 ms
- sled_base at 0.05 mm: 0.818 mm off, 55 ms
- cone_point at 0.0125 mm: outside the file by 0.3 mm on +Y, 30.000 mm off
- cone_point at 0.05 mm: outside the file by 0.3 mm on +Y, 30.000 mm off
- cone_point at 0.1 mm: outside the file by 0.2 mm on +Y, 30.000 mm off
- cone_point at 0.5 mm: 30.000 mm off, 6.8 ms
- cone_point_vertex at 0.0125 mm: outside the file by 0.3 mm on +Y, 30.000 mm off
- cone_point_vertex at 0.05 mm: outside the file by 0.3 mm on +Y, 30.000 mm off
- cone_point_vertex at 0.1 mm: outside the file by 0.2 mm on +Y, 30.000 mm off
- cone_point_vertex at 0.5 mm: short of the vertices by 29.5 mm on +Z, 30.000 mm off

**brepkit** passes 50 of 108.
- bowl_01 at 0.0125 mm: could not read: read_step found no solid (it reads MANIFOLD_SOLID_BREP and BREP_WITH_VOIDS only)
- bowl_01 at 0.05 mm: could not read: read_step found no solid (it reads MANIFOLD_SOLID_BREP and BREP_WITH_VOIDS only)
- bowl_01 at 0.1 mm: could not read: read_step found no solid (it reads MANIFOLD_SOLID_BREP and BREP_WITH_VOIDS only)
- bowl_01 at 0.5 mm: could not read: read_step found no solid (it reads MANIFOLD_SOLID_BREP and BREP_WITH_VOIDS only)
- freeform_panel at 0.0125 mm: 14.374 mm off, 689 ms
- freeform_panel at 0.05 mm: 14.374 mm off, 467 ms
- freeform_panel at 0.1 mm: 14.374 mm off, 454 ms
- freeform_panel at 0.5 mm: nothing of the mesh lies on the file's face #2114; a missing face is a hole in the part, 31.325 mm off
- sled_base at 0.0125 mm: 3.792 mm off, 48 ms
- sled_base at 0.05 mm: 4.149 mm off, 28 ms
- sled_base at 0.1 mm: 4.111 mm off, 26 ms
- sled_base at 0.5 mm: face 357 empty, 5.959 mm off
- cone_point at 0.0125 mm: 14.813 mm off, 4.0 ms
- cone_point at 0.05 mm: 14.813 mm off, 3.4 ms
- cone_point at 0.1 mm: face 1 empty, 30.000 mm off
- cone_point at 0.5 mm: face 1 empty, 30.000 mm off
- cone_point_vertex at 0.0125 mm: could not read: parse error: failed to create wire from edge loop #44: empty wire — at least one
- cone_point_vertex at 0.05 mm: could not read: parse error: failed to create wire from edge loop #44: empty wire — at least one
- cone_point_vertex at 0.1 mm: could not read: parse error: failed to create wire from edge loop #44: empty wire — at least one
- cone_point_vertex at 0.5 mm: could not read: parse error: failed to create wire from edge loop #44: empty wire — at least one
- cylinder at 0.0125 mm: 14.469 mm off, 4.0 ms
- cylinder at 0.05 mm: 14.469 mm off, 3.3 ms
- cylinder at 0.1 mm: 14.469 mm off, 3.0 ms
- cylinder at 0.5 mm: 14.470 mm off, 3.7 ms
- cylinder_inch at 0.0125 mm: outside the file by 15.4 mm on -Z, 25.400 mm off
- cylinder_inch at 0.05 mm: outside the file by 15.3 mm on -Z, 25.400 mm off
- cylinder_inch at 0.1 mm: outside the file by 15.3 mm on -Z, 25.400 mm off
- cylinder_inch at 0.5 mm: outside the file by 14.9 mm on -Z, 25.400 mm off
- cylinder_placed at 0.0125 mm: outside the file by 40.0 mm on -X, 56.957 mm off
- cylinder_placed at 0.05 mm: outside the file by 39.9 mm on -X, 56.948 mm off
- cylinder_placed at 0.1 mm: outside the file by 39.9 mm on -X, 57.025 mm off
- cylinder_placed at 0.5 mm: outside the file by 39.3 mm on -X, 56.947 mm off
- cylinder_twice at 0.0125 mm: short of the vertices by 80.0 mm on +X, 81.677 mm off
- cylinder_twice at 0.05 mm: short of the vertices by 79.9 mm on +X, 81.716 mm off
- cylinder_twice at 0.1 mm: short of the vertices by 79.9 mm on +X, 81.741 mm off
- cylinder_twice at 0.5 mm: short of the vertices by 79.7 mm on +X, 81.708 mm off
- dome_nurbs at 0.0125 mm: 3.906 mm off, 71 ms
- dome_nurbs at 0.05 mm: 3.907 mm off, 20 ms
- dome_nurbs at 0.1 mm: 3.913 mm off, 7.6 ms
- dome_nurbs at 0.5 mm: 3.938 mm off, 4.5 ms
- far_cylinder at 0.0125 mm: 14.469 mm off, 3.7 ms
- far_cylinder at 0.05 mm: 14.469 mm off, 3.5 ms
- far_cylinder at 0.1 mm: 14.469 mm off, 3.4 ms
- far_cylinder at 0.5 mm: 14.470 mm off, 2.9 ms
- sheet_disc at 0.0125 mm: 8.906 mm off, 62 ms
- sheet_disc at 0.05 mm: 8.906 mm off, 74 ms
- sheet_disc at 0.1 mm: 8.907 mm off, 19 ms
- sheet_disc at 0.5 mm: 8.920 mm off, 11 ms
- tiny_pin at 0.0125 mm: outside the file by 0.7 mm on -Z, 1.000 mm off
- tiny_pin at 0.05 mm: face 0 empty, 1000.000 mm off
- tiny_pin at 0.1 mm: face 0 empty, 1000.000 mm off
- tiny_pin at 0.5 mm: face 0 empty, 1000.000 mm off
- torus_ring_nurbs at 0.0125 mm: 0.132 mm off, 69 ms
- torus_ring_nurbs at 0.05 mm: 0.362 mm off, 21 ms
- torus_ring_nurbs at 0.1 mm: 0.630 mm off, 8.2 ms
- washer at 0.0125 mm: 1.000 mm off, 5.2 ms
- washer at 0.05 mm: 1.000 mm off, 4.2 ms
- washer at 0.1 mm: 1.000 mm off, 3.9 ms

## Speed and memory

Every run a reader passes is repeated, up to 5 times or for 10 s, and the median
taken. Time runs from starting the reader to its exit, CPU time is across all its
threads, reading and meshing are the reader's own account of them, and memory is
its peak resident memory.

Against opencascade, on the runs both pass: each figure is the reader's divided by
opencascade's, averaged geometrically, so 0.5× is twice as fast, or half the memory.

| reader | runs both pass | time | CPU time | reading | meshing | peak memory | triangles |
|---|---|---|---|---|---|---|---|
| ogeom | 87 | 0.30× | 0.55× | 0.63× | 0.25× | 0.41× | 1.3× |
| solidcraft | 92 | 0.69× | 1.1× | 1.0× | 0.93× | 0.79× | 1.4× |
| truck | 82 | 0.38× | 0.49× | 0.30× | 0.34× | 0.60× | 0.69× |
| truck-master | 91 | 0.36× | 0.46× | 0.31× | 0.31× | 0.61× | 0.63× |
| brepkit | 50 | 0.23× | 0.27× | 0.038× | 0.51× | 0.28× | 0.86× |

Every run that passes: the time, the reader's own reading and meshing, peak memory,
and triangles.

| part | tolerance mm | opencascade | ogeom | solidcraft | truck | truck-master | brepkit |
|---|---|---|---|---|---|---|---|
| bowl_01 | 0.0125 | 1.60 s (read 3.1 ms, mesh 1.59 s), 160 MB, 177,444 triangles | 58 ms (read 1.1 ms, mesh 53 ms), 17 MB, 42,680 triangles | 624 ms (read 8.5 ms, mesh 603 ms), 140 MB, 151,644 triangles | – | – | – |
| bowl_01 | 0.05 | 62 ms (read 3.1 ms, mesh 46 ms), 22 MB, 13,200 triangles | 14 ms (read 1.0 ms, mesh 9.7 ms), 8.2 MB, 11,022 triangles | 152 ms (read 8.4 ms, mesh 136 ms), 71 MB, 38,948 triangles | – | – | – |
| bowl_01 | 0.1 | 130 ms (read 3.1 ms, mesh 113 ms), 31 MB, 22,684 triangles | 8.6 ms (read 1.0 ms, mesh 5.2 ms), 6.8 MB, 6,670 triangles | 59 ms (read 8.5 ms, mesh 45 ms), 36 MB, 14,576 triangles | – | – | – |
| bowl_01 | 0.5 | 28 ms (read 3.2 ms, mesh 11 ms), 14 MB, 3,338 triangles | 5.8 ms (read 1.1 ms, mesh 1.7 ms), 5.4 MB, 1,806 triangles | 20 ms (read 8.7 ms, mesh 5.3 ms), 12 MB, 1,792 triangles | – | – | – |
| door_handle_guide | 0.0125 | 49 ms (read 24 ms, mesh 11 ms), 22 MB, 1,948 triangles | 19 ms (read 12 ms, mesh 3.4 ms), 10 MB, 7,588 triangles | 40 ms (read 17 ms, mesh 16 ms), 19 MB, 10,116 triangles | 17 ms (read 10 ms, mesh 1.3 ms), 12 MB, 1,152 triangles | 18 ms (read 11 ms, mesh 1.4 ms), 13 MB, 1,152 triangles | 7.5 ms (read 1.1 ms, mesh 3.4 ms), 5.9 MB, 1,202 triangles |
| door_handle_guide | 0.05 | 45 ms (read 24 ms, mesh 8.1 ms), 22 MB, 960 triangles | 18 ms (read 12 ms, mesh 1.7 ms), 9.2 MB, 2,524 triangles | 28 ms (read 17 ms, mesh 4.8 ms), 14 MB, 2,640 triangles | 17 ms (read 11 ms, mesh 1.0 ms), 12 MB, 696 triangles | 18 ms (read 11 ms, mesh 1.3 ms), 12 MB, 696 triangles | 6.7 ms (read 1.1 ms, mesh 2.6 ms), 5.5 MB, 742 triangles |
| door_handle_guide | 0.1 | 46 ms (read 23 ms, mesh 8.7 ms), 21 MB, 956 triangles | 17 ms (read 12 ms, mesh 1.1 ms), 9.1 MB, 1,052 triangles | 26 ms (read 18 ms, mesh 2.5 ms), 12 MB, 1,036 triangles | 16 ms (read 11 ms, mesh 1.0 ms), 12 MB, 568 triangles | 16 ms (read 10 ms, mesh 1.1 ms), 12 MB, 568 triangles | 6.1 ms (read 1.1 ms, mesh 2.5 ms), 5.5 MB, 734 triangles |
| door_handle_guide | 0.5 | 45 ms (read 24 ms, mesh 8.2 ms), 21 MB, 956 triangles | 17 ms (read 12 ms, mesh 1.3 ms), 9.1 MB, 1,052 triangles | 25 ms (read 17 ms, mesh 1.7 ms), 12 MB, 488 triangles | 17 ms (read 10 ms, mesh 0.8 ms), 12 MB, 396 triangles | 17 ms (read 10 ms, mesh 0.9 ms), 12 MB, 396 triangles | 6.7 ms (read 1.2 ms, mesh 2.6 ms), 5.7 MB, 734 triangles |
| freeform_panel | 0.0125 | 1.14 s (read 33 ms, mesh 1.09 s), 188 MB, 92,582 triangles | – | 6.45 s (read 1.06 s, mesh 5.37 s), 242 MB, 254,420 triangles | – | – | – |
| freeform_panel | 0.05 | 212 ms (read 33 ms, mesh 164 ms), 48 MB, 22,964 triangles | – | – | – | – | – |
| freeform_panel | 0.1 | 123 ms (read 33 ms, mesh 77 ms), 31 MB, 11,496 triangles | – | – | – | – | – |
| freeform_panel | 0.5 | 67 ms (read 34 ms, mesh 18 ms), 20 MB, 2,506 triangles | 117 ms (read 95 ms, mesh 18 ms), 14 MB, 3,200 triangles | – | – | – | – |
| hinge_jig | 0.0125 | 22 ms (read 8.6 ms, mesh 2.0 ms), 14 MB, 74 triangles | 4.9 ms (read 2.1 ms, mesh 0.4 ms), 6.3 MB, 74 triangles | 6.2 ms (read 1.5 ms, mesh 0.7 ms), 9.5 MB, 74 triangles | 8.6 ms (read 4.0 ms, mesh 0.5 ms), 9.8 MB, 74 triangles | 8.6 ms (read 4.1 ms, mesh 0.4 ms), 9.8 MB, 74 triangles | 3.3 ms (read 0.4 ms, mesh 0.6 ms), 3.7 MB, 74 triangles |
| hinge_jig | 0.05 | 22 ms (read 8.5 ms, mesh 2.0 ms), 14 MB, 74 triangles | 4.7 ms (read 2.1 ms, mesh 0.4 ms), 6.2 MB, 74 triangles | 6.1 ms (read 1.7 ms, mesh 0.7 ms), 9.5 MB, 74 triangles | 8.5 ms (read 4.0 ms, mesh 0.4 ms), 9.7 MB, 74 triangles | 8.6 ms (read 4.0 ms, mesh 0.4 ms), 9.8 MB, 74 triangles | 3.4 ms (read 0.4 ms, mesh 0.6 ms), 3.7 MB, 74 triangles |
| hinge_jig | 0.1 | 23 ms (read 8.7 ms, mesh 2.1 ms), 14 MB, 74 triangles | 4.8 ms (read 2.1 ms, mesh 0.4 ms), 6.3 MB, 74 triangles | 6.4 ms (read 1.6 ms, mesh 0.7 ms), 9.5 MB, 74 triangles | 8.8 ms (read 4.1 ms, mesh 0.5 ms), 9.7 MB, 74 triangles | 8.7 ms (read 4.1 ms, mesh 0.5 ms), 9.9 MB, 74 triangles | 3.3 ms (read 0.5 ms, mesh 0.6 ms), 3.7 MB, 74 triangles |
| hinge_jig | 0.5 | 23 ms (read 8.7 ms, mesh 2.0 ms), 14 MB, 74 triangles | 5.0 ms (read 2.1 ms, mesh 0.5 ms), 6.4 MB, 74 triangles | 6.7 ms (read 1.6 ms, mesh 0.6 ms), 9.5 MB, 74 triangles | 8.5 ms (read 4.0 ms, mesh 0.4 ms), 9.7 MB, 74 triangles | 8.7 ms (read 4.1 ms, mesh 0.5 ms), 10.0 MB, 74 triangles | 3.3 ms (read 0.5 ms, mesh 0.6 ms), 3.7 MB, 74 triangles |
| lego_3_pins | 0.0125 | 157 ms (read 48 ms, mesh 96 ms), 38 MB, 27,592 triangles | 58 ms (read 44 ms, mesh 10 ms), 20 MB, 34,284 triangles | 154 ms (read 69 ms, mesh 77 ms), 36 MB, 49,134 triangles | 42 ms (read 22 ms, mesh 14 ms), 19 MB, 13,348 triangles | 42 ms (read 23 ms, mesh 14 ms), 20 MB, 13,348 triangles | 37 ms (read 2.3 ms, mesh 32 ms), 9.4 MB, 13,930 triangles |
| lego_3_pins | 0.05 | 103 ms (read 48 ms, mesh 42 ms), 34 MB, 11,786 triangles | 55 ms (read 46 ms, mesh 5.2 ms), 15 MB, 14,452 triangles | 100 ms (read 71 ms, mesh 24 ms), 20 MB, 13,368 triangles | 35 ms (read 23 ms, mesh 7.0 ms), 17 MB, 6,814 triangles | 37 ms (read 23 ms, mesh 8.0 ms), 18 MB, 6,814 triangles | 24 ms (read 2.2 ms, mesh 19 ms), 7.2 MB, 7,126 triangles |
| lego_3_pins | 0.1 | 93 ms (read 48 ms, mesh 33 ms), 33 MB, 8,570 triangles | 54 ms (read 45 ms, mesh 4.9 ms), 15 MB, 12,540 triangles | 96 ms (read 70 ms, mesh 19 ms), 18 MB, 10,056 triangles | 36 ms (read 23 ms, mesh 6.0 ms), 16 MB, 4,962 triangles | 34 ms (read 23 ms, mesh 5.5 ms), 17 MB, 4,962 triangles | 23 ms (read 2.4 ms, mesh 18 ms), 7.7 MB, 5,344 triangles |
| lego_3_pins | 0.5 | 95 ms (read 50 ms, mesh 31 ms), 33 MB, 6,680 triangles | 57 ms (read 48 ms, mesh 4.0 ms), 15 MB, 6,780 triangles | 85 ms (read 71 ms, mesh 8.3 ms), 16 MB, 3,552 triangles | 31 ms (read 22 ms, mesh 3.0 ms), 16 MB, 2,374 triangles | 32 ms (read 23 ms, mesh 3.3 ms), 16 MB, 2,374 triangles | 22 ms (read 2.5 ms, mesh 17 ms), 7.5 MB, 4,696 triangles |
| lego_4x2_01 | 0.0125 | 65 ms (read 18 ms, mesh 34 ms), 23 MB, 9,064 triangles | 25 ms (read 17 ms, mesh 4.2 ms), 12 MB, 10,336 triangles | 54 ms (read 23 ms, mesh 25 ms), 21 MB, 14,950 triangles | 19 ms (read 8.2 ms, mesh 4.9 ms), 14 MB, 4,356 triangles | 20 ms (read 8.3 ms, mesh 5.3 ms), 14 MB, 4,356 triangles | 15 ms (read 0.9 ms, mesh 11 ms), 8.1 MB, 4,550 triangles |
| lego_4x2_01 | 0.05 | 47 ms (read 19 ms, mesh 15 ms), 20 MB, 3,862 triangles | 24 ms (read 18 ms, mesh 2.6 ms), 11 MB, 4,528 triangles | 37 ms (read 23 ms, mesh 8.0 ms), 14 MB, 4,084 triangles | 16 ms (read 7.9 ms, mesh 2.6 ms), 12 MB, 2,226 triangles | 16 ms (read 8.0 ms, mesh 2.4 ms), 13 MB, 2,226 triangles | 11 ms (read 0.9 ms, mesh 7.1 ms), 6.8 MB, 2,330 triangles |
| lego_4x2_01 | 0.1 | 43 ms (read 18 ms, mesh 12 ms), 19 MB, 2,790 triangles | 23 ms (read 18 ms, mesh 2.5 ms), 10 MB, 4,080 triangles | 37 ms (read 25 ms, mesh 6.9 ms), 14 MB, 3,252 triangles | 15 ms (read 7.8 ms, mesh 2.0 ms), 12 MB, 1,622 triangles | 15 ms (read 7.8 ms, mesh 2.0 ms), 12 MB, 1,622 triangles | 9.5 ms (read 0.9 ms, mesh 6.0 ms), 5.9 MB, 1,736 triangles |
| lego_4x2_01 | 0.5 | 39 ms (read 17 ms, mesh 9.7 ms), 19 MB, 2,160 triangles | 22 ms (read 17 ms, mesh 1.7 ms), 11 MB, 2,160 triangles | 31 ms (read 23 ms, mesh 3.1 ms), 12 MB, 1,156 triangles | 15 ms (read 8.1 ms, mesh 1.1 ms), 12 MB, 778 triangles | 15 ms (read 8.1 ms, mesh 1.0 ms), 12 MB, 778 triangles | 11 ms (read 1.0 ms, mesh 6.7 ms), 5.8 MB, 1,520 triangles |
| rail_guide | 0.0125 | 77 ms (read 35 ms, mesh 28 ms), 28 MB, 6,484 triangles | 30 ms (read 22 ms, mesh 3.8 ms), 13 MB, 8,744 triangles | 72 ms (read 38 ms, mesh 27 ms), 20 MB, 11,808 triangles | 25 ms (read 16 ms, mesh 3.3 ms), 14 MB, 2,596 triangles | 25 ms (read 15 ms, mesh 3.8 ms), 14 MB, 2,596 triangles | 11 ms (read 1.6 ms, mesh 6.5 ms), 7.0 MB, 2,948 triangles |
| rail_guide | 0.05 | 65 ms (read 35 ms, mesh 17 ms), 26 MB, 2,884 triangles | 27 ms (read 22 ms, mesh 2.2 ms), 11 MB, 3,216 triangles | 54 ms (read 39 ms, mesh 9.2 ms), 15 MB, 4,076 triangles | 31 ms (read 17 ms, mesh 3.0 ms), 13 MB, 1,404 triangles | 24 ms (read 15 ms, mesh 2.0 ms), 14 MB, 1,404 triangles | 9.4 ms (read 1.7 ms, mesh 4.5 ms), 6.7 MB, 1,604 triangles |
| rail_guide | 0.1 | 62 ms (read 33 ms, mesh 15 ms), 26 MB, 2,412 triangles | 26 ms (read 21 ms, mesh 1.8 ms), 11 MB, 2,448 triangles | 48 ms (read 35 ms, mesh 6.4 ms), 14 MB, 2,648 triangles | 23 ms (read 15 ms, mesh 1.5 ms), 13 MB, 1,100 triangles | 23 ms (read 15 ms, mesh 1.7 ms), 13 MB, 1,100 triangles | 9.1 ms (read 1.7 ms, mesh 4.2 ms), 6.8 MB, 1,492 triangles |
| rail_guide | 0.5 | 60 ms (read 32 ms, mesh 13 ms), 26 MB, 2,244 triangles | 27 ms (read 22 ms, mesh 2.1 ms), 11 MB, 2,208 triangles | 44 ms (read 36 ms, mesh 2.8 ms), 13 MB, 804 triangles | 21 ms (read 15 ms, mesh 1.2 ms), 13 MB, 652 triangles | 21 ms (read 14 ms, mesh 1.2 ms), 13 MB, 652 triangles | 7.8 ms (read 1.6 ms, mesh 3.9 ms), 6.6 MB, 1,492 triangles |
| sled_base | 0.0125 | 240 ms (read 97 ms, mesh 128 ms), 68 MB, 33,808 triangles | 74 ms (read 56 ms, mesh 14 ms), 28 MB, 54,166 triangles | 319 ms (read 203 ms, mesh 107 ms), 69 MB, 67,500 triangles | 136 ms (read 41 ms, mesh 87 ms), 53 MB, 66,567 triangles | 77 ms (read 41 ms, mesh 28 ms), 36 MB, 28,640 triangles | – |
| sled_base | 0.05 | 162 ms (read 93 ms, mesh 55 ms), 57 MB, 12,716 triangles | 68 ms (read 57 ms, mesh 6.7 ms), 23 MB, 17,434 triangles | 247 ms (read 205 ms, mesh 34 ms), 31 MB, 20,952 triangles | 73 ms (read 41 ms, mesh 24 ms), 29 MB, 22,127 triangles | – | – |
| sled_base | 0.1 | 165 ms (read 98 ms, mesh 50 ms), 58 MB, 9,254 triangles | 72 ms (read 60 ms, mesh 7.4 ms), 23 MB, 15,386 triangles | 238 ms (read 207 ms, mesh 24 ms), 28 MB, 16,112 triangles | – | 54 ms (read 41 ms, mesh 4.8 ms), 21 MB, 5,210 triangles | – |
| sled_base | 0.5 | 148 ms (read 92 ms, mesh 42 ms), 56 MB, 7,946 triangles | 67 ms (read 57 ms, mesh 6.7 ms), 23 MB, 13,610 triangles | 217 ms (read 203 ms, mesh 8.1 ms), 20 MB, 3,206 triangles | – | 49 ms (read 41 ms, mesh 3.4 ms), 20 MB, 2,194 triangles | – |
| cone_point | 0.0125 | 19 ms (read 1.7 ms, mesh 5.6 ms), 14 MB, 1,563 triangles | – | – | – | – | – |
| cone_point | 0.05 | 16 ms (read 1.8 ms, mesh 1.6 ms), 11 MB, 473 triangles | 4.4 ms (read 1.2 ms, mesh 0.9 ms), 4.6 MB, 653 triangles | – | – | – | – |
| cone_point | 0.1 | 16 ms (read 2.0 ms, mesh 1.0 ms), 11 MB, 293 triangles | 4.5 ms (read 1.3 ms, mesh 0.5 ms), 4.3 MB, 361 triangles | – | – | – | – |
| cone_point | 0.5 | 15 ms (read 1.9 ms, mesh 0.5 ms), 11 MB, 129 triangles | 3.9 ms (read 1.2 ms, mesh 0.3 ms), 4.3 MB, 131 triangles | – | – | – | – |
| cone_point_vertex | 0.0125 | 20 ms (read 2.0 ms, mesh 5.9 ms), 14 MB, 1,563 triangles | – | – | – | – | – |
| cone_point_vertex | 0.05 | 15 ms (read 1.7 ms, mesh 1.5 ms), 11 MB, 473 triangles | 4.5 ms (read 1.2 ms, mesh 0.9 ms), 4.6 MB, 653 triangles | – | – | – | – |
| cone_point_vertex | 0.1 | 15 ms (read 1.7 ms, mesh 0.9 ms), 11 MB, 293 triangles | 4.1 ms (read 1.2 ms, mesh 0.5 ms), 4.4 MB, 361 triangles | – | – | – | – |
| cone_point_vertex | 0.5 | 14 ms (read 1.7 ms, mesh 0.5 ms), 11 MB, 129 triangles | 3.7 ms (read 1.2 ms, mesh 0.2 ms), 4.3 MB, 131 triangles | – | – | – | – |
| cylinder | 0.0125 | 15 ms (read 1.8 ms, mesh 1.4 ms), 11 MB, 352 triangles | 4.0 ms (read 0.9 ms, mesh 0.9 ms), 4.8 MB, 882 triangles | 9.3 ms (read 1.2 ms, mesh 3.4 ms), 11 MB, 1,262 triangles | 5.5 ms (read 0.4 ms, mesh 0.5 ms), 8.5 MB, 248 triangles | 5.2 ms (read 0.5 ms, mesh 0.5 ms), 8.7 MB, 248 triangles | – |
| cylinder | 0.05 | 15 ms (read 1.8 ms, mesh 0.8 ms), 11 MB, 176 triangles | 3.5 ms (read 0.9 ms, mesh 0.3 ms), 4.4 MB, 310 triangles | 6.5 ms (read 1.1 ms, mesh 1.2 ms), 9.6 MB, 374 triangles | 5.0 ms (read 0.4 ms, mesh 0.4 ms), 8.4 MB, 124 triangles | 4.8 ms (read 0.4 ms, mesh 0.4 ms), 8.6 MB, 124 triangles | – |
| cylinder | 0.1 | 14 ms (read 1.9 ms, mesh 0.6 ms), 11 MB, 124 triangles | 3.4 ms (read 0.9 ms, mesh 0.4 ms), 4.4 MB, 310 triangles | 6.1 ms (read 1.1 ms, mesh 0.9 ms), 9.1 MB, 310 triangles | 4.7 ms (read 0.4 ms, mesh 0.3 ms), 8.3 MB, 88 triangles | 4.6 ms (read 0.4 ms, mesh 0.3 ms), 8.5 MB, 88 triangles | – |
| cylinder | 0.5 | 15 ms (read 2.1 ms, mesh 0.5 ms), 11 MB, 100 triangles | 4.2 ms (read 1.1 ms, mesh 0.2 ms), 4.4 MB, 90 triangles | 6.5 ms (read 1.3 ms, mesh 0.5 ms), 8.6 MB, 90 triangles | 6.5 ms (read 0.5 ms, mesh 0.3 ms), 8.2 MB, 36 triangles | 6.3 ms (read 0.5 ms, mesh 0.4 ms), 8.4 MB, 36 triangles | – |
| cylinder_inch | 0.0125 | 17 ms (read 2.1 ms, mesh 1.7 ms), 11 MB, 352 triangles | 5.3 ms (read 1.1 ms, mesh 1.0 ms), 4.8 MB, 882 triangles | 9.6 ms (read 1.1 ms, mesh 3.3 ms), 11 MB, 1,262 triangles | 5.6 ms (read 0.5 ms, mesh 0.6 ms), 8.5 MB, 248 triangles | 5.6 ms (read 0.5 ms, mesh 0.6 ms), 8.8 MB, 248 triangles | – |
| cylinder_inch | 0.05 | 15 ms (read 1.8 ms, mesh 0.8 ms), 11 MB, 176 triangles | 3.5 ms (read 0.9 ms, mesh 0.4 ms), 4.4 MB, 310 triangles | 7.1 ms (read 1.2 ms, mesh 1.2 ms), 9.6 MB, 374 triangles | 5.0 ms (read 0.5 ms, mesh 0.4 ms), 8.4 MB, 124 triangles | 4.8 ms (read 0.5 ms, mesh 0.4 ms), 8.7 MB, 124 triangles | – |
| cylinder_inch | 0.1 | 15 ms (read 1.9 ms, mesh 0.6 ms), 11 MB, 124 triangles | 3.5 ms (read 0.9 ms, mesh 0.4 ms), 4.4 MB, 310 triangles | 6.2 ms (read 1.1 ms, mesh 0.9 ms), 9.2 MB, 310 triangles | 5.0 ms (read 0.5 ms, mesh 0.3 ms), 8.3 MB, 88 triangles | 4.7 ms (read 0.5 ms, mesh 0.3 ms), 8.6 MB, 88 triangles | – |
| cylinder_inch | 0.5 | 15 ms (read 2.0 ms, mesh 0.6 ms), 11 MB, 100 triangles | 3.9 ms (read 1.0 ms, mesh 0.2 ms), 4.4 MB, 90 triangles | 6.3 ms (read 1.2 ms, mesh 0.5 ms), 8.6 MB, 90 triangles | 5.6 ms (read 0.5 ms, mesh 0.3 ms), 8.2 MB, 36 triangles | 5.5 ms (read 0.5 ms, mesh 0.3 ms), 8.4 MB, 36 triangles | – |
| cylinder_nurbs | 0.0125 | 19 ms (read 3.1 ms, mesh 3.3 ms), 14 MB, 372 triangles | – | 16 ms (read 1.3 ms, mesh 9.6 ms), 15 MB, 3,040 triangles | 55 ms (read 0.7 ms, mesh 48 ms), 21 MB, 24,508 triangles | 34 ms (read 0.8 ms, mesh 27 ms), 15 MB, 15,008 triangles | 6.6 ms (read 0.1 ms, mesh 4.1 ms), 3.2 MB, 322 triangles |
| cylinder_nurbs | 0.05 | 16 ms (read 2.7 ms, mesh 1.2 ms), 12 MB, 184 triangles | – | 8.7 ms (read 1.4 ms, mesh 3.1 ms), 11 MB, 846 triangles | 16 ms (read 0.7 ms, mesh 11 ms), 12 MB, 6,104 triangles | 10 ms (read 0.7 ms, mesh 5.0 ms), 10 MB, 3,098 triangles | 4.6 ms (read 0.1 ms, mesh 2.2 ms), 3.0 MB, 178 triangles |
| cylinder_nurbs | 0.1 | 15 ms (read 2.7 ms, mesh 0.8 ms), 12 MB, 132 triangles | – | 7.1 ms (read 1.3 ms, mesh 1.7 ms), 9.2 MB, 310 triangles | 8.7 ms (read 0.7 ms, mesh 4.0 ms), 9.5 MB, 2,046 triangles | 7.8 ms (read 0.7 ms, mesh 3.2 ms), 9.7 MB, 2,046 triangles | 4.6 ms (read 0.1 ms, mesh 2.2 ms), 3.0 MB, 178 triangles |
| cylinder_nurbs | 0.5 | 16 ms (read 3.0 ms, mesh 0.8 ms), 12 MB, 120 triangles | – | 7.5 ms (read 1.3 ms, mesh 1.1 ms), 8.6 MB, 90 triangles | 8.3 ms (read 0.8 ms, mesh 1.6 ms), 8.9 MB, 510 triangles | 7.1 ms (read 0.8 ms, mesh 1.2 ms), 9.0 MB, 510 triangles | 4.5 ms (read 0.1 ms, mesh 1.8 ms), 2.9 MB, 178 triangles |
| cylinder_placed | 0.0125 | 16 ms (read 1.9 ms, mesh 1.4 ms), 11 MB, 352 triangles | 4.5 ms (read 1.0 ms, mesh 1.0 ms), 4.9 MB, 882 triangles | 9.5 ms (read 1.2 ms, mesh 3.5 ms), 11 MB, 1,262 triangles | – | 6.2 ms (read 0.6 ms, mesh 0.5 ms), 8.8 MB, 248 triangles | – |
| cylinder_placed | 0.05 | 16 ms (read 2.2 ms, mesh 0.8 ms), 11 MB, 176 triangles | 3.6 ms (read 0.9 ms, mesh 0.3 ms), 4.4 MB, 310 triangles | 7.2 ms (read 1.2 ms, mesh 1.2 ms), 9.7 MB, 374 triangles | – | 6.6 ms (read 0.6 ms, mesh 0.4 ms), 8.7 MB, 124 triangles | – |
| cylinder_placed | 0.1 | 16 ms (read 2.3 ms, mesh 0.6 ms), 11 MB, 124 triangles | 4.4 ms (read 1.0 ms, mesh 0.4 ms), 4.4 MB, 310 triangles | 6.8 ms (read 1.2 ms, mesh 1.0 ms), 9.3 MB, 310 triangles | – | 6.3 ms (read 0.6 ms, mesh 0.4 ms), 8.6 MB, 88 triangles | – |
| cylinder_placed | 0.5 | 16 ms (read 2.2 ms, mesh 0.6 ms), 11 MB, 100 triangles | 4.1 ms (read 1.1 ms, mesh 0.2 ms), 4.4 MB, 90 triangles | 6.1 ms (read 1.2 ms, mesh 0.5 ms), 8.6 MB, 90 triangles | – | 4.9 ms (read 0.6 ms, mesh 0.3 ms), 8.4 MB, 36 triangles | – |
| cylinder_seam | 0.0125 | 15 ms (read 1.9 ms, mesh 1.5 ms), 11 MB, 352 triangles | 4.0 ms (read 0.9 ms, mesh 0.9 ms), 4.8 MB, 882 triangles | 8.9 ms (read 1.2 ms, mesh 3.2 ms), 11 MB, 1,262 triangles | 5.5 ms (read 0.5 ms, mesh 0.5 ms), 8.5 MB, 248 triangles | 5.5 ms (read 0.5 ms, mesh 0.5 ms), 8.7 MB, 248 triangles | 3.4 ms (read 0.1 ms, mesh 1.0 ms), 3.1 MB, 248 triangles |
| cylinder_seam | 0.05 | 15 ms (read 1.8 ms, mesh 0.8 ms), 11 MB, 176 triangles | 3.5 ms (read 1.0 ms, mesh 0.4 ms), 4.4 MB, 310 triangles | 6.5 ms (read 1.2 ms, mesh 1.2 ms), 9.5 MB, 374 triangles | 5.0 ms (read 0.5 ms, mesh 0.4 ms), 8.4 MB, 124 triangles | 4.9 ms (read 0.5 ms, mesh 0.4 ms), 8.7 MB, 124 triangles | 3.2 ms (read 0.1 ms, mesh 0.7 ms), 2.9 MB, 124 triangles |
| cylinder_seam | 0.1 | 14 ms (read 1.7 ms, mesh 0.6 ms), 10 MB, 124 triangles | 3.5 ms (read 1.0 ms, mesh 0.4 ms), 4.4 MB, 310 triangles | 6.1 ms (read 1.1 ms, mesh 1.0 ms), 9.2 MB, 310 triangles | 5.1 ms (read 0.5 ms, mesh 0.3 ms), 8.3 MB, 88 triangles | 5.1 ms (read 0.5 ms, mesh 0.3 ms), 8.5 MB, 88 triangles | 2.9 ms (read 0.1 ms, mesh 0.6 ms), 2.8 MB, 88 triangles |
| cylinder_seam | 0.5 | 14 ms (read 1.9 ms, mesh 0.5 ms), 10 MB, 100 triangles | 3.4 ms (read 0.9 ms, mesh 0.2 ms), 4.4 MB, 90 triangles | 6.1 ms (read 1.2 ms, mesh 0.5 ms), 8.5 MB, 90 triangles | 4.6 ms (read 0.5 ms, mesh 0.2 ms), 8.2 MB, 36 triangles | 4.9 ms (read 0.5 ms, mesh 0.3 ms), 8.3 MB, 36 triangles | 3.1 ms (read 0.1 ms, mesh 0.6 ms), 2.8 MB, 68 triangles |
| cylinder_twice | 0.0125 | 16 ms (read 1.9 ms, mesh 1.5 ms), 11 MB, 704 triangles | 5.4 ms (read 1.0 ms, mesh 1.9 ms), 5.1 MB, 1,764 triangles | 13 ms (read 2.0 ms, mesh 6.5 ms), 12 MB, 2,524 triangles | – | 5.8 ms (read 0.6 ms, mesh 0.9 ms), 9.2 MB, 496 triangles | – |
| cylinder_twice | 0.05 | 14 ms (read 1.9 ms, mesh 0.8 ms), 11 MB, 352 triangles | 4.0 ms (read 1.0 ms, mesh 0.7 ms), 4.6 MB, 620 triangles | 9.9 ms (read 2.1 ms, mesh 3.6 ms), 11 MB, 1,432 triangles | – | 5.9 ms (read 0.6 ms, mesh 0.6 ms), 9.0 MB, 248 triangles | – |
| cylinder_twice | 0.1 | 14 ms (read 1.9 ms, mesh 0.6 ms), 11 MB, 248 triangles | 3.9 ms (read 0.9 ms, mesh 0.7 ms), 4.6 MB, 620 triangles | 7.9 ms (read 1.9 ms, mesh 1.8 ms), 9.6 MB, 620 triangles | – | 5.2 ms (read 0.6 ms, mesh 0.5 ms), 8.9 MB, 176 triangles | – |
| cylinder_twice | 0.5 | 14 ms (read 1.8 ms, mesh 0.5 ms), 11 MB, 200 triangles | 3.5 ms (read 0.9 ms, mesh 0.3 ms), 4.5 MB, 180 triangles | 6.7 ms (read 1.9 ms, mesh 1.0 ms), 8.9 MB, 180 triangles | – | 5.0 ms (read 0.6 ms, mesh 0.5 ms), 8.7 MB, 72 triangles | – |
| dome | 0.0125 | 42 ms (read 1.8 ms, mesh 28 ms), 26 MB, 8,157 triangles | – | 49 ms (read 2.2 ms, mesh 41 ms), 34 MB, 14,571 triangles | 10 ms (read 0.3 ms, mesh 5.3 ms), 10 MB, 4,111 triangles | 9.5 ms (read 0.3 ms, mesh 5.1 ms), 11 MB, 4,111 triangles | 51 ms (read 0.1 ms, mesh 48 ms), 26 MB, 31,344 triangles |
| dome | 0.05 | 18 ms (read 1.6 ms, mesh 4.9 ms), 13 MB, 2,061 triangles | – | 17 ms (read 2.1 ms, mesh 9.6 ms), 16 MB, 3,639 triangles | 5.9 ms (read 0.3 ms, mesh 1.4 ms), 9.0 MB, 1,087 triangles | 5.7 ms (read 0.3 ms, mesh 1.3 ms), 9.2 MB, 1,087 triangles | 13 ms (read 0.1 ms, mesh 10 ms), 8.5 MB, 7,860 triangles |
| dome | 0.1 | 17 ms (read 1.9 ms, mesh 2.5 ms), 12 MB, 1,091 triangles | – | 14 ms (read 2.2 ms, mesh 6.9 ms), 15 MB, 2,485 triangles | 5.4 ms (read 0.4 ms, mesh 0.9 ms), 8.6 MB, 511 triangles | 5.3 ms (read 0.4 ms, mesh 0.9 ms), 8.7 MB, 511 triangles | 7.8 ms (read 0.1 ms, mesh 5.5 ms), 5.5 MB, 3,948 triangles |
| dome | 0.5 | 14 ms (read 1.6 ms, mesh 0.7 ms), 11 MB, 269 triangles | – | 7.4 ms (read 2.2 ms, mesh 1.3 ms), 9.5 MB, 403 triangles | 4.7 ms (read 0.3 ms, mesh 0.3 ms), 8.2 MB, 119 triangles | 4.8 ms (read 0.3 ms, mesh 0.3 ms), 8.4 MB, 119 triangles | 3.6 ms (read 0.1 ms, mesh 1.4 ms), 3.4 MB, 744 triangles |
| dome_nurbs | 0.0125 | 128 ms (read 2.9 ms, mesh 112 ms), 32 MB, 20,317 triangles | – | 55 ms (read 2.1 ms, mesh 46 ms), 34 MB, 13,623 triangles | 61 ms (read 0.7 ms, mesh 52 ms), 25 MB, 26,987 triangles | 35 ms (read 0.7 ms, mesh 28 ms), 18 MB, 16,827 triangles | – |
| dome_nurbs | 0.05 | 37 ms (read 2.8 ms, mesh 21 ms), 16 MB, 4,941 triangles | – | 18 ms (read 2.0 ms, mesh 11 ms), 16 MB, 3,385 triangles | 17 ms (read 0.7 ms, mesh 11 ms), 12 MB, 6,709 triangles | 11 ms (read 0.7 ms, mesh 5.9 ms), 11 MB, 4,189 triangles | – |
| dome_nurbs | 0.1 | 26 ms (read 2.7 ms, mesh 10 ms), 14 MB, 2,435 triangles | – | 14 ms (read 2.0 ms, mesh 7.5 ms), 13 MB, 2,385 triangles | 13 ms (read 0.6 ms, mesh 7.6 ms), 10 MB, 4,295 triangles | 10 ms (read 0.7 ms, mesh 4.7 ms), 10 MB, 3,388 triangles | – |
| dome_nurbs | 0.5 | 17 ms (read 2.5 ms, mesh 2.2 ms), 12 MB, 563 triangles | – | 7.9 ms (read 2.0 ms, mesh 1.8 ms), 9.7 MB, 409 triangles | 5.7 ms (read 0.6 ms, mesh 1.2 ms), 8.7 MB, 427 triangles | 5.3 ms (read 0.6 ms, mesh 0.8 ms), 8.7 MB, 277 triangles | – |
| far_cylinder | 0.0125 | 15 ms (read 1.8 ms, mesh 1.7 ms), 11 MB, 352 triangles | 4.2 ms (read 0.9 ms, mesh 0.9 ms), 4.8 MB, 882 triangles | 8.4 ms (read 1.1 ms, mesh 3.2 ms), 11 MB, 1,262 triangles | 5.8 ms (read 0.5 ms, mesh 0.5 ms), 8.6 MB, 248 triangles | 5.3 ms (read 0.5 ms, mesh 0.5 ms), 8.8 MB, 248 triangles | – |
| far_cylinder | 0.05 | 15 ms (read 1.9 ms, mesh 0.8 ms), 11 MB, 176 triangles | 3.8 ms (read 1.0 ms, mesh 0.4 ms), 4.3 MB, 310 triangles | 8.1 ms (read 1.1 ms, mesh 2.5 ms), 11 MB, 1,058 triangles | 5.3 ms (read 0.4 ms, mesh 0.3 ms), 8.4 MB, 124 triangles | 5.3 ms (read 0.5 ms, mesh 0.4 ms), 8.7 MB, 124 triangles | – |
| far_cylinder | 0.1 | 15 ms (read 1.9 ms, mesh 0.6 ms), 11 MB, 124 triangles | 3.6 ms (read 0.9 ms, mesh 0.4 ms), 4.4 MB, 310 triangles | 6.3 ms (read 1.0 ms, mesh 1.0 ms), 9.1 MB, 310 triangles | 5.2 ms (read 0.4 ms, mesh 0.3 ms), 8.3 MB, 88 triangles | 5.4 ms (read 0.5 ms, mesh 0.3 ms), 8.5 MB, 88 triangles | – |
| far_cylinder | 0.5 | 14 ms (read 1.9 ms, mesh 0.5 ms), 11 MB, 100 triangles | 3.6 ms (read 1.0 ms, mesh 0.2 ms), 4.3 MB, 90 triangles | 6.2 ms (read 1.2 ms, mesh 0.5 ms), 8.5 MB, 90 triangles | 4.9 ms (read 0.4 ms, mesh 0.3 ms), 8.2 MB, 36 triangles | 4.9 ms (read 0.4 ms, mesh 0.3 ms), 8.3 MB, 36 triangles | – |
| grooved_shaft | 0.0125 | 523 ms (read 120 ms, mesh 386 ms), 102 MB, 132,852 triangles | 305 ms (read 263 ms, mesh 35 ms), 88 MB, 190,878 triangles | 662 ms (read 353 ms, mesh 294 ms), 130 MB, 168,380 triangles | 109 ms (read 70 ms, mesh 28 ms), 67 MB, 94,248 triangles | 111 ms (read 74 ms, mesh 27 ms), 69 MB, 94,248 triangles | 118 ms (read 7.3 ms, mesh 106 ms), 59 MB, 94,748 triangles |
| grooved_shaft | 0.05 | 317 ms (read 120 ms, mesh 180 ms), 88 MB, 67,176 triangles | 284 ms (read 259 ms, mesh 19 ms), 63 MB, 94,686 triangles | 494 ms (read 354 ms, mesh 131 ms), 73 MB, 76,188 triangles | 91 ms (read 70 ms, mesh 13 ms), 51 MB, 47,624 triangles | 95 ms (read 74 ms, mesh 12 ms), 52 MB, 47,624 triangles | 68 ms (read 7.3 ms, mesh 57 ms), 34 MB, 48,124 triangles |
| grooved_shaft | 0.1 | 266 ms (read 121 ms, mesh 131 ms), 85 MB, 47,624 triangles | 286 ms (read 260 ms, mesh 19 ms), 65 MB, 94,686 triangles | 429 ms (read 350 ms, mesh 70 ms), 62 MB, 48,124 triangles | 86 ms (read 69 ms, mesh 9.5 ms), 45 MB, 34,088 triangles | 89 ms (read 73 ms, mesh 8.8 ms), 46 MB, 34,088 triangles | 58 ms (read 7.6 ms, mesh 47 ms), 31 MB, 34,588 triangles |
| grooved_shaft | 0.5 | 331 ms (read 130 ms, mesh 181 ms), 86 MB, 39,100 triangles | 291 ms (read 268 ms, mesh 18 ms), 65 MB, 70,590 triangles | 400 ms (read 354 ms, mesh 39 ms), 47 MB, 24,060 triangles | 84 ms (read 71 ms, mesh 6.0 ms), 35 MB, 15,036 triangles | 86 ms (read 73 ms, mesh 5.8 ms), 36 MB, 15,036 triangles | 49 ms (read 7.9 ms, mesh 37 ms), 23 MB, 27,068 triangles |
| plate_100_holes | 0.0125 | 346 ms (read 54 ms, mesh 273 ms), 71 MB, 20,252 triangles | 94 ms (read 30 ms, mesh 48 ms), 35 MB, 89,868 triangles | 465 ms (read 58 ms, mesh 393 ms), 111 MB, 127,788 triangles | 122 ms (read 16 ms, mesh 99 ms), 22 MB, 14,572 triangles | 126 ms (read 17 ms, mesh 101 ms), 24 MB, 14,572 triangles | 67 ms (read 1.7 ms, mesh 62 ms), 17 MB, 14,596 triangles |
| plate_100_holes | 0.05 | 134 ms (read 52 ms, mesh 68 ms), 46 MB, 10,940 triangles | 56 ms (read 29 ms, mesh 24 ms), 22 MB, 32,108 triangles | 214 ms (read 69 ms, mesh 140 ms), 43 MB, 38,308 triangles | 52 ms (read 16 ms, mesh 30 ms), 17 MB, 7,692 triangles | 54 ms (read 17 ms, mesh 31 ms), 19 MB, 7,692 triangles | 33 ms (read 1.8 ms, mesh 28 ms), 11 MB, 7,820 triangles |
| plate_100_holes | 0.1 | 135 ms (read 52 ms, mesh 70 ms), 50 MB, 10,924 triangles | 32 ms (read 22 ms, mesh 7.1 ms), 16 MB, 9,900 triangles | 100 ms (read 59 ms, mesh 35 ms), 22 MB, 12,900 triangles | 42 ms (read 16 ms, mesh 19 ms), 15 MB, 5,676 triangles | 41 ms (read 17 ms, mesh 18 ms), 17 MB, 5,676 triangles | 33 ms (read 1.7 ms, mesh 28 ms), 11 MB, 7,820 triangles |
| plate_100_holes | 0.5 | 134 ms (read 52 ms, mesh 69 ms), 52 MB, 10,924 triangles | 32 ms (read 21 ms, mesh 7.1 ms), 16 MB, 9,900 triangles | 76 ms (read 58 ms, mesh 12 ms), 16 MB, 5,044 triangles | 27 ms (read 16 ms, mesh 4.9 ms), 14 MB, 2,844 triangles | 27 ms (read 17 ms, mesh 4.9 ms), 15 MB, 2,844 triangles | 33 ms (read 1.8 ms, mesh 28 ms), 12 MB, 7,820 triangles |
| plate_900_holes | 0.0125 | 13.5 s (read 1.34 s, mesh 12.2 s), 1146 MB, 180,252 triangles | 477 ms (read 149 ms, mesh 312 ms), 217 MB, 801,868 triangles | 19.9 s (read 548 ms, mesh 19.3 s), 733 MB, 1,142,188 triangles | 5.72 s (read 133 ms, mesh 5.57 s), 111 MB, 129,772 triangles | 5.89 s (read 137 ms, mesh 5.74 s), 118 MB, 129,772 triangles | 2.75 s (read 14 ms, mesh 2.73 s), 132 MB, 129,796 triangles |
| plate_900_holes | 0.05 | 5.33 s (read 1.30 s, mesh 4.01 s), 533 MB, 97,340 triangles | 289 ms (read 148 ms, mesh 130 ms), 124 MB, 286,508 triangles | 5.40 s (read 600 ms, mesh 4.76 s), 274 MB, 342,308 triangles | 1.50 s (read 130 ms, mesh 1.36 s), 77 MB, 68,492 triangles | 1.60 s (read 135 ms, mesh 1.46 s), 81 MB, 68,492 triangles | 1.02 s (read 14 ms, mesh 999 ms), 73 MB, 68,620 triangles |
| plate_900_holes | 0.1 | 5.34 s (read 1.30 s, mesh 4.01 s), 518 MB, 97,324 triangles | 186 ms (read 135 ms, mesh 46 ms), 84 MB, 88,300 triangles | 1.72 s (read 534 ms, mesh 1.17 s), 134 MB, 115,300 triangles | 901 ms (read 128 ms, mesh 761 ms), 64 MB, 50,476 triangles | 956 ms (read 134 ms, mesh 813 ms), 67 MB, 50,476 triangles | 1.03 s (read 13 ms, mesh 1.01 s), 74 MB, 68,620 triangles |
| plate_900_holes | 0.5 | 5.50 s (read 1.30 s, mesh 4.18 s), 510 MB, 97,324 triangles | 193 ms (read 140 ms, mesh 47 ms), 83 MB, 88,300 triangles | 869 ms (read 542 ms, mesh 315 ms), 78 MB, 45,044 triangles | 363 ms (read 150 ms, mesh 188 ms), 54 MB, 25,244 triangles | 378 ms (read 148 ms, mesh 215 ms), 57 MB, 25,244 triangles | 1.48 s (read 15 ms, mesh 1.45 s), 76 MB, 68,620 triangles |
| sheet_disc | 0.0125 | 94 ms (read 2.2 ms, mesh 73 ms), 31 MB, 4,768 triangles | 45 ms (read 16 ms, mesh 21 ms), 8.6 MB, 5,112 triangles | 100 ms (read 26 ms, mesh 59 ms), 16 MB, 4,092 triangles | 44 ms (read 1.1 ms, mesh 28 ms), 11 MB, 3,368 triangles | 37 ms (read 1.1 ms, mesh 23 ms), 12 MB, 3,368 triangles | – |
| sheet_disc | 0.05 | 40 ms (read 2.2 ms, mesh 19 ms), 16 MB, 2,384 triangles | 26 ms (read 13 ms, mesh 7.0 ms), 7.6 MB, 3,064 triangles | 82 ms (read 27 ms, mesh 22 ms), 13 MB, 2,044 triangles | 27 ms (read 1.1 ms, mesh 12 ms), 9.8 MB, 1,684 triangles | 31 ms (read 1.1 ms, mesh 16 ms), 10 MB, 1,684 triangles | – |
| sheet_disc | 0.1 | 31 ms (read 2.2 ms, mesh 11 ms), 14 MB, 1,684 triangles | 35 ms (read 16 ms, mesh 12 ms), 7.6 MB, 3,064 triangles | 84 ms (read 19 ms, mesh 23 ms), 13 MB, 2,044 triangles | 19 ms (read 1.1 ms, mesh 5.1 ms), 9.5 MB, 1,192 triangles | 24 ms (read 1.1 ms, mesh 11 ms), 9.9 MB, 1,192 triangles | – |
| sheet_disc | 0.5 | 24 ms (read 2.2 ms, mesh 3.5 ms), 12 MB, 752 triangles | 29 ms (read 16 ms, mesh 5.4 ms), 7.5 MB, 1,530 triangles | 25 ms (read 6.0 ms, mesh 5.9 ms), 11 MB, 956 triangles | 21 ms (read 1.1 ms, mesh 7.5 ms), 9.0 MB, 532 triangles | 27 ms (read 1.1 ms, mesh 11 ms), 9.3 MB, 532 triangles | – |
| tiny_pin | 0.0125 | 21 ms (read 2.1 ms, mesh 0.6 ms), 10 MB, 100 triangles | 10 ms (read 1.9 ms, mesh 0.5 ms), 4.2 MB, 150 triangles | 40 ms (read 26 ms, mesh 4.7 ms), 8.8 MB, 180 triangles | 21 ms (read 1.1 ms, mesh 5.8 ms), 8.1 MB, 36 triangles | 14 ms (read 1.1 ms, mesh 0.7 ms), 8.3 MB, 36 triangles | – |
| tiny_pin | 0.05 | 17 ms (read 2.2 ms, mesh 0.7 ms), 11 MB, 100 triangles | 5.5 ms (read 0.9 ms, mesh 0.3 ms), 4.2 MB, 150 triangles | 18 ms (read 2.2 ms, mesh 1.2 ms), 8.4 MB, 56 triangles | 10 ms (read 1.0 ms, mesh 0.3 ms), 8.1 MB, 16 triangles | 6.9 ms (read 0.5 ms, mesh 0.3 ms), 8.3 MB, 16 triangles | – |
| tiny_pin | 0.1 | 15 ms (read 2.1 ms, mesh 0.6 ms), 11 MB, 100 triangles | 4.0 ms (read 0.9 ms, mesh 0.3 ms), 4.2 MB, 150 triangles | 6.2 ms (read 1.2 ms, mesh 0.4 ms), 8.4 MB, 36 triangles | 5.2 ms (read 0.4 ms, mesh 0.3 ms), 8.1 MB, 12 triangles | 5.0 ms (read 0.5 ms, mesh 0.3 ms), 8.3 MB, 12 triangles | – |
| tiny_pin | 0.5 | 14 ms (read 2.0 ms, mesh 0.6 ms), 10 MB, 100 triangles | 3.3 ms (read 0.7 ms, mesh 0.3 ms), 4.2 MB, 150 triangles | – | 5.2 ms (read 0.4 ms, mesh 0.2 ms), 8.0 MB, 8 triangles | 5.1 ms (read 0.5 ms, mesh 0.2 ms), 8.2 MB, 8 triangles | – |
| torus_ring | 0.0125 | 62 ms (read 2.3 ms, mesh 41 ms), 24 MB, 12,218 triangles | 28 ms (read 4.1 ms, mesh 17 ms), 7.7 MB, 8,448 triangles | – | 26 ms (read 1.0 ms, mesh 9.1 ms), 11 MB, 4,772 triangles | 24 ms (read 1.1 ms, mesh 11 ms), 12 MB, 4,772 triangles | 17 ms (read 0.1 ms, mesh 12 ms), 13 MB, 15,978 triangles |
| torus_ring | 0.05 | 24 ms (read 2.2 ms, mesh 8.1 ms), 14 MB, 3,102 triangles | 10 ms (read 2.1 ms, mesh 3.3 ms), 5.8 MB, 2,176 triangles | – | 7.5 ms (read 0.4 ms, mesh 1.9 ms), 9.1 MB, 1,286 triangles | 7.2 ms (read 0.5 ms, mesh 1.7 ms), 9.3 MB, 1,286 triangles | 6.2 ms (read 0.1 ms, mesh 3.5 ms), 5.3 MB, 4,084 triangles |
| torus_ring | 0.1 | 19 ms (read 2.2 ms, mesh 4.4 ms), 13 MB, 1,894 triangles | 5.8 ms (read 1.9 ms, mesh 1.0 ms), 5.0 MB, 832 triangles | – | 6.5 ms (read 0.4 ms, mesh 1.0 ms), 8.8 MB, 632 triangles | 6.0 ms (read 0.4 ms, mesh 1.0 ms), 9.1 MB, 632 triangles | 4.7 ms (read 0.1 ms, mesh 2.1 ms), 4.3 MB, 2,108 triangles |
| torus_ring | 0.5 | 16 ms (read 2.1 ms, mesh 2.0 ms), 12 MB, 780 triangles | 4.8 ms (read 1.7 ms, mesh 0.5 ms), 4.8 MB, 288 triangles | – | 5.5 ms (read 0.4 ms, mesh 0.5 ms), 8.3 MB, 154 triangles | 5.2 ms (read 0.4 ms, mesh 0.4 ms), 8.5 MB, 154 triangles | 3.0 ms (read 0.1 ms, mesh 0.8 ms), 3.0 MB, 432 triangles |
| torus_ring_nurbs | 0.0125 | 167 ms (read 3.6 ms, mesh 150 ms), 36 MB, 29,108 triangles | – | 96 ms (read 2.3 ms, mesh 86 ms), 57 MB, 20,974 triangles | 87 ms (read 0.9 ms, mesh 78 ms), 26 MB, 32,832 triangles | 42 ms (read 0.9 ms, mesh 35 ms), 16 MB, 16,384 triangles | – |
| torus_ring_nurbs | 0.05 | 39 ms (read 3.3 ms, mesh 23 ms), 17 MB, 6,316 triangles | – | 27 ms (read 2.3 ms, mesh 20 ms), 21 MB, 5,366 triangles | 23 ms (read 0.8 ms, mesh 17 ms), 13 MB, 8,224 triangles | 13 ms (read 0.8 ms, mesh 7.2 ms), 11 MB, 4,096 triangles | – |
| torus_ring_nurbs | 0.1 | 27 ms (read 3.3 ms, mesh 11 ms), 15 MB, 3,248 triangles | – | 22 ms (read 2.3 ms, mesh 15 ms), 16 MB, 4,024 triangles | 19 ms (read 0.8 ms, mesh 13 ms), 12 MB, 6,514 triangles | 13 ms (read 0.8 ms, mesh 7.0 ms), 10 MB, 3,770 triangles | – |
| torus_ring_nurbs | 0.5 | 18 ms (read 3.1 ms, mesh 2.7 ms), 12 MB, 746 triangles | – | 9.1 ms (read 2.2 ms, mesh 2.4 ms), 9.7 MB, 396 triangles | 7.9 ms (read 0.8 ms, mesh 2.8 ms), 9.0 MB, 960 triangles | 6.5 ms (read 0.8 ms, mesh 1.4 ms), 9.0 MB, 576 triangles | 6.2 ms (read 0.1 ms, mesh 3.6 ms), 3.9 MB, 1,082 triangles |
| washer | 0.0125 | 18 ms (read 2.4 ms, mesh 3.5 ms), 12 MB, 756 triangles | 4.4 ms (read 1.1 ms, mesh 0.9 ms), 5.2 MB, 1,148 triangles | 9.3 ms (read 1.8 ms, mesh 3.2 ms), 10 MB, 894 triangles | 5.8 ms (read 0.6 ms, mesh 1.0 ms), 9.1 MB, 536 triangles | 5.6 ms (read 0.6 ms, mesh 1.0 ms), 9.4 MB, 536 triangles | – |
| washer | 0.05 | 16 ms (read 2.2 ms, mesh 1.6 ms), 11 MB, 380 triangles | 4.1 ms (read 1.2 ms, mesh 0.5 ms), 4.7 MB, 572 triangles | 8.0 ms (read 1.9 ms, mesh 1.6 ms), 9.4 MB, 384 triangles | 5.3 ms (read 0.6 ms, mesh 0.6 ms), 8.7 MB, 272 triangles | 5.1 ms (read 0.6 ms, mesh 0.6 ms), 9.0 MB, 272 triangles | – |
| washer | 0.1 | 15 ms (read 2.3 ms, mesh 1.2 ms), 11 MB, 284 triangles | 3.8 ms (read 1.1 ms, mesh 0.3 ms), 4.6 MB, 284 triangles | 7.5 ms (read 1.8 ms, mesh 1.3 ms), 9.2 MB, 288 triangles | 5.3 ms (read 0.6 ms, mesh 0.5 ms), 8.6 MB, 192 triangles | 5.0 ms (read 0.6 ms, mesh 0.5 ms), 8.8 MB, 192 triangles | – |
| washer | 0.5 | 15 ms (read 2.3 ms, mesh 1.0 ms), 11 MB, 208 triangles | 3.8 ms (read 1.1 ms, mesh 0.3 ms), 4.6 MB, 188 triangles | 6.5 ms (read 1.8 ms, mesh 0.7 ms), 8.7 MB, 96 triangles | 5.6 ms (read 0.6 ms, mesh 0.3 ms), 8.4 MB, 88 triangles | 4.9 ms (read 0.6 ms, mesh 0.4 ms), 8.6 MB, 88 triangles | 3.8 ms (read 0.1 ms, mesh 1.4 ms), 3.9 MB, 144 triangles |
