# Results

Run 2026-10-12 on Linux x86_64, step-conformance 7922d0e-dirty.

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
| bowl_01 | 0.0125 | ok, 0.042 mm, 1.88 s | ok, 0.021 mm, 76 ms | ok, 0.012 mm, 645 ms | timed out at 60 s | 10.000 mm off, 244 ms | could not read: read_step found no solid (it reads MANIFOLD_SOLID_BREP and BREP_WITH_VOIDS only) |
| bowl_01 | 0.05 | ok, 0.050 mm, 71 ms | ok, 0.082 mm, 17 ms | ok, 0.047 mm, 188 ms | 53.498 mm off, 4.21 s | 0.218 mm off, 14 ms | could not read: read_step found no solid (it reads MANIFOLD_SOLID_BREP and BREP_WITH_VOIDS only) |
| bowl_01 | 0.1 | ok, 0.098 mm, 137 ms | ok, 0.105 mm, 9.3 ms | ok, 0.075 mm, 68 ms | 53.503 mm off, 3.88 s | 0.430 mm off, 7.8 ms | could not read: read_step found no solid (it reads MANIFOLD_SOLID_BREP and BREP_WITH_VOIDS only) |
| bowl_01 | 0.5 | ok, 0.464 mm, 20 ms | ok, 0.416 mm, 4.4 ms | ok, 0.401 mm, 22 ms | 53.546 mm off, 6.4 ms | 2.111 mm off, 5.8 ms | could not read: read_step found no solid (it reads MANIFOLD_SOLID_BREP and BREP_WITH_VOIDS only) |
| door_handle_guide | 0.0125 | ok, 0.008 mm, 45 ms | ok, 0.008 mm, 22 ms | ok, 0.008 mm, 51 ms | ok, 0.012 mm, 15 ms | ok, 0.012 mm, 15 ms | ok, 0.017 mm, 7.9 ms |
| door_handle_guide | 0.05 | ok, 0.020 mm, 42 ms | ok, 0.031 mm, 21 ms | ok, 0.034 mm, 39 ms | ok, 0.044 mm, 15 ms | ok, 0.044 mm, 15 ms | ok, 0.064 mm, 7.7 ms |
| door_handle_guide | 0.1 | ok, 0.020 mm, 39 ms | ok, 0.061 mm, 18 ms | ok, 0.066 mm, 35 ms | ok, 0.089 mm, 14 ms | ok, 0.089 mm, 14 ms | ok, 0.039 mm, 6.3 ms |
| door_handle_guide | 0.5 | ok, 0.020 mm, 38 ms | ok, 0.061 mm, 18 ms | ok, 0.242 mm, 34 ms | ok, 0.353 mm, 13 ms | ok, 0.353 mm, 14 ms | ok, 0.039 mm, 6.8 ms |
| freeform_panel | 0.0125 | ok, 0.017 mm, 1.40 s | 5.468 mm off, 398 ms | ok, 0.014 mm, 5.72 s | timed out at 60 s | 12.172 mm off, 10.3 s | 14.374 mm off, 605 ms |
| freeform_panel | 0.05 | ok, 0.055 mm, 234 ms | 0.607 mm off, 202 ms | 5.306 mm off, 2.55 s | 12.172 mm off, 22.7 s | 12.172 mm off, 1.73 s | 14.374 mm off, 440 ms |
| freeform_panel | 0.1 | ok, 0.106 mm, 136 ms | 0.608 mm off, 144 ms | 5.306 mm off, 1.92 s | 12.172 mm off, 6.08 s | 12.172 mm off, 711 ms | 14.374 mm off, 427 ms |
| freeform_panel | 0.5 | ok, 0.482 mm, 62 ms | ok, 1.187 mm, 104 ms | outside the file by 123.5 mm on +Y, 149.166 mm off | 12.172 mm off, 1.30 s | 12.172 mm off, 126 ms | nothing of the mesh lies on the file's face #2114; a missing face is a hole in the part, 31.325 mm off |
| hinge_jig | 0.0125 | ok, 0.000 mm, 15 ms | ok, 0.000 mm, 6.5 ms | ok, 0.000 mm, 13 ms | ok, 0.000 mm, 7.5 ms | ok, 0.000 mm, 7.5 ms | ok, 0.000 mm, 3.4 ms |
| hinge_jig | 0.05 | ok, 0.000 mm, 15 ms | ok, 0.000 mm, 6.5 ms | ok, 0.000 mm, 15 ms | ok, 0.000 mm, 7.6 ms | ok, 0.000 mm, 8.0 ms | ok, 0.000 mm, 3.7 ms |
| hinge_jig | 0.1 | ok, 0.000 mm, 16 ms | ok, 0.000 mm, 6.7 ms | ok, 0.000 mm, 15 ms | ok, 0.000 mm, 7.9 ms | ok, 0.000 mm, 7.8 ms | ok, 0.000 mm, 3.8 ms |
| hinge_jig | 0.5 | ok, 0.000 mm, 15 ms | ok, 0.000 mm, 6.8 ms | ok, 0.000 mm, 14 ms | ok, 0.000 mm, 7.4 ms | ok, 0.000 mm, 7.5 ms | ok, 0.000 mm, 3.7 ms |
| lego_3_pins | 0.0125 | ok, 0.008 mm, 183 ms | ok, 0.011 mm, 72 ms | ok, 0.012 mm, 203 ms | ok, 0.012 mm, 42 ms | ok, 0.012 mm, 43 ms | ok, 0.017 mm, 42 ms |
| lego_3_pins | 0.05 | ok, 0.030 mm, 124 ms | ok, 0.044 mm, 69 ms | ok, 0.049 mm, 150 ms | ok, 0.048 mm, 39 ms | ok, 0.048 mm, 37 ms | ok, 0.096 mm, 29 ms |
| lego_3_pins | 0.1 | ok, 0.058 mm, 101 ms | ok, 0.061 mm, 62 ms | ok, 0.066 mm, 130 ms | ok, 0.094 mm, 32 ms | ok, 0.094 mm, 31 ms | ok, 0.183 mm, 24 ms |
| lego_3_pins | 0.5 | ok, 0.067 mm, 97 ms | ok, 0.177 mm, 62 ms | ok, 0.298 mm, 121 ms | ok, 0.498 mm, 33 ms | ok, 0.498 mm, 32 ms | ok, 0.140 mm, 25 ms |
| lego_4x2_01 | 0.0125 | ok, 0.008 mm, 67 ms | ok, 0.011 mm, 28 ms | ok, 0.012 mm, 66 ms | ok, 0.012 mm, 15 ms | ok, 0.012 mm, 15 ms | ok, 0.017 mm, 15 ms |
| lego_4x2_01 | 0.05 | ok, 0.030 mm, 43 ms | ok, 0.044 mm, 29 ms | ok, 0.049 mm, 49 ms | ok, 0.048 mm, 14 ms | ok, 0.048 mm, 14 ms | ok, 0.096 mm, 11 ms |
| lego_4x2_01 | 0.1 | ok, 0.058 mm, 38 ms | ok, 0.061 mm, 30 ms | ok, 0.066 mm, 49 ms | ok, 0.094 mm, 13 ms | ok, 0.094 mm, 13 ms | ok, 0.183 mm, 9.7 ms |
| lego_4x2_01 | 0.5 | ok, 0.067 mm, 36 ms | ok, 0.177 mm, 26 ms | ok, 0.298 mm, 43 ms | ok, 0.498 mm, 12 ms | ok, 0.498 mm, 12 ms | ok, 0.140 mm, 9.4 ms |
| rail_guide | 0.0125 | ok, 0.007 mm, 71 ms | ok, 0.013 mm, 42 ms | ok, 0.012 mm, 74 ms | ok, 0.012 mm, 18 ms | ok, 0.012 mm, 18 ms | ok, 0.014 mm, 11 ms |
| rail_guide | 0.05 | ok, 0.028 mm, 58 ms | ok, 0.048 mm, 37 ms | ok, 0.048 mm, 63 ms | ok, 0.048 mm, 18 ms | ok, 0.048 mm, 19 ms | ok, 0.039 mm, 10.0 ms |
| rail_guide | 0.1 | ok, 0.047 mm, 60 ms | ok, 0.055 mm, 33 ms | ok, 0.064 mm, 61 ms | ok, 0.095 mm, 19 ms | ok, 0.095 mm, 21 ms | ok, 0.078 mm, 9.5 ms |
| rail_guide | 0.5 | ok, 0.069 mm, 60 ms | ok, 0.122 mm, 40 ms | ok, 0.483 mm, 57 ms | ok, 0.483 mm, 19 ms | ok, 0.483 mm, 19 ms | ok, 0.078 mm, 9.2 ms |
| sled_base | 0.0125 | ok, 0.014 mm, 268 ms | ok, 0.017 mm, 107 ms | ok, 0.012 mm, 345 ms | 0.069 mm off, 147 ms | ok, 0.021 mm, 79 ms | 3.792 mm off, 53 ms |
| sled_base | 0.05 | ok, 0.052 mm, 185 ms | ok, 0.067 mm, 88 ms | ok, 0.049 mm, 239 ms | ok, 0.084 mm, 70 ms | 0.818 mm off, 52 ms | 4.149 mm off, 30 ms |
| sled_base | 0.1 | ok, 0.094 mm, 197 ms | ok, 0.119 mm, 101 ms | ok, 0.098 mm, 252 ms | timed out at 60 s | ok, 0.223 mm, 44 ms | 4.111 mm off, 24 ms |
| sled_base | 0.5 | ok, 0.349 mm, 176 ms | ok, 0.252 mm, 92 ms | ok, 0.483 mm, 224 ms | 4.953 mm off, 45 ms | ok, 0.621 mm, 43 ms | face 357 empty, 5.959 mm off |
| cone_point | 0.0125 | ok, 0.011 mm, 11 ms | face 1 lies 0.080 mm off every face of the file it could be; its surface was evaluated wrongly, 0.092 mm off | 30.000 mm off, 4.1 ms | outside the file by 0.3 mm on +Y, 30.000 mm off | outside the file by 0.3 mm on +Y, 30.000 mm off | 14.813 mm off, 1.5 ms |
| cone_point | 0.05 | ok, 0.039 mm, 5.2 ms | ok, 0.137 mm, 3.2 ms | 30.000 mm off, 3.3 ms | outside the file by 0.3 mm on +Y, 30.000 mm off | outside the file by 0.3 mm on +Y, 30.000 mm off | 14.813 mm off, 1.5 ms |
| cone_point | 0.1 | ok, 0.073 mm, 4.6 ms | ok, 0.235 mm, 2.7 ms | 30.000 mm off, 3.5 ms | outside the file by 0.2 mm on +Y, 30.000 mm off | outside the file by 0.2 mm on +Y, 30.000 mm off | face 1 empty, 30.000 mm off |
| cone_point | 0.5 | ok, 0.205 mm, 4.0 ms | ok, 0.256 mm, 2.4 ms | 30.000 mm off, 3.5 ms | 30.000 mm off, 2.2 ms | 30.000 mm off, 2.4 ms | face 1 empty, 30.000 mm off |
| cone_point_vertex | 0.0125 | ok, 0.011 mm, 11 ms | face 1 lies 0.080 mm off every face of the file it could be; its surface was evaluated wrongly, 0.092 mm off | short of the vertices by 30.0 mm on +Z, 30.000 mm off | outside the file by 0.3 mm on +Y, 30.000 mm off | outside the file by 0.3 mm on +Y, 30.000 mm off | could not read: parse error: failed to create wire from edge loop #44: empty wire — at least one |
| cone_point_vertex | 0.05 | ok, 0.039 mm, 5.6 ms | ok, 0.137 mm, 3.4 ms | short of the vertices by 29.9 mm on +Z, 30.000 mm off | outside the file by 0.3 mm on +Y, 30.000 mm off | outside the file by 0.3 mm on +Y, 30.000 mm off | could not read: parse error: failed to create wire from edge loop #44: empty wire — at least one |
| cone_point_vertex | 0.1 | ok, 0.073 mm, 4.6 ms | ok, 0.235 mm, 2.8 ms | short of the vertices by 29.9 mm on +Z, 30.000 mm off | outside the file by 0.2 mm on +Y, 30.000 mm off | outside the file by 0.2 mm on +Y, 30.000 mm off | could not read: parse error: failed to create wire from edge loop #44: empty wire — at least one |
| cone_point_vertex | 0.5 | ok, 0.205 mm, 3.9 ms | ok, 0.256 mm, 2.5 ms | short of the vertices by 29.5 mm on +Z, 30.000 mm off | short of the vertices by 29.5 mm on +Z, 30.000 mm off | short of the vertices by 29.5 mm on +Z, 30.000 mm off | could not read: parse error: failed to create wire from edge loop #44: empty wire — at least one |
| cylinder | 0.0125 | ok, 0.006 mm, 5.6 ms | ok, 0.012 mm, 3.0 ms | ok, 0.012 mm, 7.0 ms | ok, 0.012 mm, 3.1 ms | ok, 0.012 mm, 2.4 ms | 14.469 mm off, 2.3 ms |
| cylinder | 0.05 | ok, 0.024 mm, 4.4 ms | ok, 0.048 mm, 2.3 ms | ok, 0.050 mm, 4.9 ms | ok, 0.048 mm, 2.2 ms | ok, 0.048 mm, 2.8 ms | 14.469 mm off, 1.7 ms |
| cylinder | 0.1 | ok, 0.048 mm, 4.6 ms | ok, 0.048 mm, 2.4 ms | ok, 0.053 mm, 4.8 ms | ok, 0.093 mm, 2.3 ms | ok, 0.093 mm, 2.3 ms | 14.469 mm off, 1.6 ms |
| cylinder | 0.5 | ok, 0.073 mm, 4.1 ms | ok, 0.192 mm, 2.0 ms | ok, 0.218 mm, 4.5 ms | ok, 0.489 mm, 2.3 ms | ok, 0.489 mm, 2.1 ms | 14.470 mm off, 1.4 ms |
| cylinder_inch | 0.0125 | ok, 0.006 mm, 5.3 ms | ok, 0.012 mm, 3.6 ms | ok, 0.012 mm, 7.1 ms | ok, 0.012 mm, 2.8 ms | ok, 0.012 mm, 2.6 ms | outside the file by 15.4 mm on -Z, 25.400 mm off |
| cylinder_inch | 0.05 | ok, 0.024 mm, 4.4 ms | ok, 0.048 mm, 2.3 ms | ok, 0.050 mm, 4.8 ms | ok, 0.048 mm, 2.3 ms | ok, 0.048 mm, 2.7 ms | outside the file by 15.3 mm on -Z, 25.400 mm off |
| cylinder_inch | 0.1 | ok, 0.048 mm, 4.1 ms | ok, 0.048 mm, 2.3 ms | ok, 0.053 mm, 5.2 ms | ok, 0.093 mm, 2.8 ms | ok, 0.093 mm, 2.2 ms | outside the file by 15.3 mm on -Z, 25.400 mm off |
| cylinder_inch | 0.5 | ok, 0.073 mm, 4.0 ms | ok, 0.192 mm, 2.0 ms | ok, 0.218 mm, 4.9 ms | ok, 0.489 mm, 2.5 ms | ok, 0.489 mm, 2.2 ms | outside the file by 14.9 mm on -Z, 25.400 mm off |
| cylinder_nurbs | 0.0125 | ok, 0.006 mm, 9.0 ms | face 1 empty, 15.000 mm off | ok, 0.012 mm, 15 ms | ok, 0.012 mm, 55 ms | ok, 0.012 mm, 31 ms | ok, 0.013 mm, 4.2 ms |
| cylinder_nurbs | 0.05 | ok, 0.025 mm, 6.2 ms | face 1 empty, 15.000 mm off | ok, 0.046 mm, 7.8 ms | ok, 0.050 mm, 15 ms | ok, 0.050 mm, 7.4 ms | ok, 0.040 mm, 2.6 ms |
| cylinder_nurbs | 0.1 | ok, 0.049 mm, 5.8 ms | face 1 empty, 15.000 mm off | ok, 0.053 mm, 5.8 ms | ok, 0.053 mm, 6.7 ms | ok, 0.053 mm, 6.1 ms | ok, 0.040 mm, 2.7 ms |
| cylinder_nurbs | 0.5 | ok, 0.077 mm, 5.6 ms | face 1 empty, 15.000 mm off | ok, 0.207 mm, 5.6 ms | ok, 0.207 mm, 3.8 ms | ok, 0.207 mm, 4.0 ms | ok, 0.040 mm, 2.5 ms |
| cylinder_placed | 0.0125 | ok, 0.006 mm, 5.5 ms | ok, 0.012 mm, 3.4 ms | ok, 0.012 mm, 6.8 ms | outside the file by 40.0 mm on -X, 56.630 mm off | ok, 0.012 mm, 2.5 ms | outside the file by 40.0 mm on -X, 56.957 mm off |
| cylinder_placed | 0.05 | ok, 0.024 mm, 4.8 ms | ok, 0.048 mm, 2.3 ms | ok, 0.050 mm, 5.4 ms | outside the file by 39.9 mm on -X, 56.630 mm off | ok, 0.048 mm, 2.4 ms | outside the file by 39.9 mm on -X, 56.948 mm off |
| cylinder_placed | 0.1 | ok, 0.048 mm, 4.4 ms | ok, 0.048 mm, 2.3 ms | ok, 0.053 mm, 4.6 ms | outside the file by 39.8 mm on -X, 56.667 mm off | ok, 0.093 mm, 2.5 ms | outside the file by 39.9 mm on -X, 57.025 mm off |
| cylinder_placed | 0.5 | ok, 0.073 mm, 4.1 ms | ok, 0.192 mm, 2.1 ms | ok, 0.218 mm, 4.3 ms | outside the file by 39.5 mm on -X, 56.851 mm off | ok, 0.489 mm, 2.5 ms | outside the file by 39.3 mm on -X, 56.947 mm off |
| cylinder_seam | 0.0125 | ok, 0.006 mm, 5.3 ms | ok, 0.012 mm, 2.9 ms | ok, 0.012 mm, 7.5 ms | ok, 0.012 mm, 2.7 ms | ok, 0.012 mm, 2.6 ms | ok, 0.012 mm, 1.7 ms |
| cylinder_seam | 0.05 | ok, 0.024 mm, 4.3 ms | ok, 0.048 mm, 2.2 ms | ok, 0.048 mm, 5.5 ms | ok, 0.048 mm, 2.7 ms | ok, 0.048 mm, 2.5 ms | ok, 0.048 mm, 1.4 ms |
| cylinder_seam | 0.1 | ok, 0.048 mm, 4.1 ms | ok, 0.048 mm, 2.3 ms | ok, 0.053 mm, 5.0 ms | ok, 0.093 mm, 2.6 ms | ok, 0.093 mm, 2.2 ms | ok, 0.093 mm, 1.3 ms |
| cylinder_seam | 0.5 | ok, 0.073 mm, 3.9 ms | ok, 0.192 mm, 2.0 ms | ok, 0.207 mm, 4.6 ms | ok, 0.489 mm, 2.7 ms | ok, 0.489 mm, 2.2 ms | ok, 0.152 mm, 1.3 ms |
| cylinder_twice | 0.0125 | ok, 0.006 mm, 5.5 ms | ok, 0.012 mm, 4.3 ms | ok, 0.012 mm, 13 ms | short of the vertices by 80.0 mm on +X, 80.555 mm off | ok, 0.012 mm, 3.0 ms | short of the vertices by 80.0 mm on +X, 81.677 mm off |
| cylinder_twice | 0.05 | ok, 0.024 mm, 4.7 ms | ok, 0.048 mm, 2.8 ms | ok, 0.050 mm, 9.6 ms | short of the vertices by 79.9 mm on +X, 80.595 mm off | ok, 0.048 mm, 3.3 ms | short of the vertices by 79.9 mm on +X, 81.716 mm off |
| cylinder_twice | 0.1 | ok, 0.048 mm, 4.3 ms | ok, 0.048 mm, 2.9 ms | ok, 0.053 mm, 9.0 ms | short of the vertices by 79.9 mm on +X, 80.623 mm off | ok, 0.093 mm, 2.6 ms | short of the vertices by 79.9 mm on +X, 81.741 mm off |
| cylinder_twice | 0.5 | ok, 0.073 mm, 4.2 ms | ok, 0.192 mm, 2.3 ms | ok, 0.218 mm, 7.1 ms | short of the vertices by 79.5 mm on +X, 80.623 mm off | ok, 0.489 mm, 2.7 ms | short of the vertices by 79.7 mm on +X, 81.708 mm off |
| dome | 0.0125 | ok, 0.012 mm, 45 ms | face 1 lies 0.064 mm off every face of the file it could be; its surface was evaluated wrongly, 20.000 mm off | ok, 0.012 mm, 56 ms | ok, 0.024 mm, 7.7 ms | ok, 0.024 mm, 7.9 ms | ok, 0.012 mm, 49 ms |
| dome | 0.05 | ok, 0.048 mm, 9.8 ms | 20.000 mm off, 5.9 ms | ok, 0.047 mm, 16 ms | ok, 0.091 mm, 3.5 ms | ok, 0.091 mm, 3.3 ms | ok, 0.049 mm, 11 ms |
| dome | 0.1 | ok, 0.083 mm, 6.7 ms | 20.000 mm off, 3.9 ms | ok, 0.093 mm, 13 ms | ok, 0.191 mm, 3.0 ms | ok, 0.191 mm, 3.0 ms | ok, 0.096 mm, 6.0 ms |
| dome | 0.5 | ok, 0.345 mm, 4.2 ms | 20.000 mm off, 3.2 ms | ok, 0.415 mm, 6.2 ms | ok, 0.797 mm, 2.2 ms | ok, 0.797 mm, 2.3 ms | ok, 0.304 mm, 2.1 ms |
| dome_nurbs | 0.0125 | ok, 0.013 mm, 129 ms | face 1 empty, 20.000 mm off | ok, 0.012 mm, 62 ms | ok, 0.008 mm, 65 ms | ok, 0.008 mm, 32 ms | 3.906 mm off, 70 ms |
| dome_nurbs | 0.05 | ok, 0.050 mm, 31 ms | face 1 empty, 20.000 mm off | ok, 0.047 mm, 20 ms | ok, 0.032 mm, 17 ms | ok, 0.032 mm, 8.9 ms | 3.907 mm off, 17 ms |
| dome_nurbs | 0.1 | ok, 0.087 mm, 18 ms | face 1 empty, 20.000 mm off | ok, 0.093 mm, 13 ms | ok, 0.131 mm, 12 ms | ok, 0.100 mm, 8.2 ms | 3.913 mm off, 5.7 ms |
| dome_nurbs | 0.5 | ok, 0.400 mm, 7.7 ms | face 1 empty, 20.000 mm off | ok, 0.415 mm, 6.9 ms | ok, 0.538 mm, 4.3 ms | ok, 0.538 mm, 3.0 ms | 3.938 mm off, 2.8 ms |
| far_cylinder | 0.0125 | ok, 0.006 mm, 6.1 ms | ok, 0.012 mm, 3.3 ms | ok, 0.012 mm, 7.6 ms | ok, 0.012 mm, 2.4 ms | ok, 0.012 mm, 2.5 ms | 14.469 mm off, 2.2 ms |
| far_cylinder | 0.05 | ok, 0.024 mm, 4.7 ms | ok, 0.048 mm, 2.4 ms | ok, 0.046 mm, 7.3 ms | ok, 0.048 mm, 2.9 ms | ok, 0.048 mm, 2.9 ms | 14.469 mm off, 1.7 ms |
| far_cylinder | 0.1 | ok, 0.048 mm, 4.4 ms | ok, 0.048 mm, 2.4 ms | ok, 0.053 mm, 4.8 ms | ok, 0.093 mm, 2.7 ms | ok, 0.093 mm, 2.7 ms | 14.469 mm off, 1.6 ms |
| far_cylinder | 0.5 | ok, 0.073 mm, 4.3 ms | ok, 0.192 mm, 2.2 ms | ok, 0.216 mm, 4.4 ms | ok, 0.489 mm, 2.5 ms | ok, 0.489 mm, 3.1 ms | 14.470 mm off, 1.5 ms |
| grooved_shaft | 0.0125 | ok, 0.006 mm, 728 ms | ok, 0.012 mm, 440 ms | ok, 0.012 mm, 651 ms | ok, 0.012 mm, 112 ms | ok, 0.012 mm, 120 ms | ok, 0.012 mm, 143 ms |
| grooved_shaft | 0.05 | ok, 0.024 mm, 391 ms | ok, 0.048 mm, 415 ms | ok, 0.046 mm, 508 ms | ok, 0.048 mm, 93 ms | ok, 0.048 mm, 98 ms | ok, 0.048 mm, 85 ms |
| grooved_shaft | 0.1 | ok, 0.048 mm, 328 ms | ok, 0.048 mm, 430 ms | ok, 0.053 mm, 460 ms | ok, 0.093 mm, 85 ms | ok, 0.093 mm, 87 ms | ok, 0.093 mm, 64 ms |
| grooved_shaft | 0.5 | ok, 0.073 mm, 312 ms | ok, 0.189 mm, 423 ms | ok, 0.207 mm, 427 ms | ok, 0.489 mm, 78 ms | ok, 0.489 mm, 79 ms | ok, 0.152 mm, 54 ms |
| plate_100_holes | 0.0125 | ok, 0.005 mm, 308 ms | ok, 0.005 mm, 99 ms | ok, 0.005 mm, 453 ms | ok, 0.012 mm, 114 ms | ok, 0.012 mm, 118 ms | ok, 0.012 mm, 61 ms |
| plate_100_holes | 0.05 | ok, 0.022 mm, 159 ms | ok, 0.019 mm, 79 ms | ok, 0.020 mm, 202 ms | ok, 0.049 mm, 48 ms | ok, 0.049 mm, 49 ms | ok, 0.046 mm, 34 ms |
| plate_100_holes | 0.1 | ok, 0.025 mm, 160 ms | ok, 0.077 mm, 68 ms | ok, 0.082 mm, 115 ms | ok, 0.087 mm, 34 ms | ok, 0.087 mm, 35 ms | ok, 0.046 mm, 31 ms |
| plate_100_holes | 0.5 | ok, 0.025 mm, 163 ms | ok, 0.077 mm, 67 ms | ok, 0.303 mm, 89 ms | ok, 0.402 mm, 22 ms | ok, 0.402 mm, 23 ms | ok, 0.046 mm, 32 ms |
| plate_900_holes | 0.0125 | ok, 0.006 mm, 14.8 s | ok, 0.004 mm, 879 ms | ok, 0.005 mm, 19.3 s | ok, 0.012 mm, 5.26 s | ok, 0.012 mm, 5.66 s | ok, 0.012 mm, 2.13 s |
| plate_900_holes | 0.05 | ok, 0.022 mm, 6.22 s | ok, 0.017 mm, 681 ms | ok, 0.018 mm, 5.53 s | ok, 0.049 mm, 1.38 s | ok, 0.049 mm, 1.48 s | ok, 0.046 mm, 880 ms |
| plate_900_holes | 0.1 | ok, 0.025 mm, 6.18 s | ok, 0.077 mm, 592 ms | ok, 0.083 mm, 1.66 s | ok, 0.087 mm, 799 ms | ok, 0.087 mm, 851 ms | ok, 0.046 mm, 853 ms |
| plate_900_holes | 0.5 | ok, 0.025 mm, 6.00 s | ok, 0.077 mm, 582 ms | ok, 0.304 mm, 858 ms | ok, 0.401 mm, 266 ms | ok, 0.401 mm, 280 ms | ok, 0.046 mm, 854 ms |
| sheet_disc | 0.0125 | ok, 0.006 mm, 81 ms | ok, 0.008 mm, 25 ms | ok, 0.009 mm, 39 ms | ok, 0.012 mm, 18 ms | ok, 0.012 mm, 19 ms | 8.906 mm off, 38 ms |
| sheet_disc | 0.05 | ok, 0.025 mm, 27 ms | ok, 0.030 mm, 15 ms | ok, 0.037 mm, 17 ms | ok, 0.050 mm, 6.8 ms | ok, 0.050 mm, 7.5 ms | 8.906 mm off, 15 ms |
| sheet_disc | 0.1 | ok, 0.050 mm, 18 ms | ok, 0.030 mm, 15 ms | ok, 0.037 mm, 17 ms | ok, 0.099 mm, 5.2 ms | ok, 0.099 mm, 4.9 ms | 8.907 mm off, 8.2 ms |
| sheet_disc | 0.5 | ok, 0.249 mm, 8.1 ms | ok, 0.120 mm, 11 ms | ok, 0.443 mm, 9.8 ms | ok, 0.495 mm, 2.7 ms | ok, 0.495 mm, 3.4 ms | 8.920 mm off, 4.1 ms |
| tiny_pin | 0.0125 | ok, 0.002 mm, 4.4 ms | ok, 0.005 mm, 1.9 ms | ok, 0.005 mm, 4.5 ms | ok, 0.012 mm, 2.1 ms | ok, 0.012 mm, 2.3 ms | outside the file by 0.7 mm on -Z, 1.000 mm off |
| tiny_pin | 0.05 | ok, 0.002 mm, 4.3 ms | ok, 0.005 mm, 2.0 ms | ok, 0.024 mm, 4.5 ms | ok, 0.048 mm, 2.4 ms | ok, 0.048 mm, 2.8 ms | face 0 empty, 1000.000 mm off |
| tiny_pin | 0.1 | ok, 0.002 mm, 4.6 ms | ok, 0.005 mm, 2.1 ms | ok, 0.073 mm, 5.2 ms | ok, 0.073 mm, 2.3 ms | ok, 0.073 mm, 2.2 ms | face 0 empty, 1000.000 mm off |
| tiny_pin | 0.5 | ok, 0.002 mm, 4.3 ms | ok, 0.005 mm, 2.1 ms | outside the file by 0.2 mm on +Y, 1.164 mm off | ok, 0.125 mm, 2.5 ms | ok, 0.125 mm, 2.2 ms | face 0 empty, 1000.000 mm off |
| torus_ring | 0.0125 | ok, 0.016 mm, 53 ms | ok, 0.014 mm, 14 ms | face 1 lies 3.482 mm off every face of the file it could be; its surface was evaluated wrongly, 5.000 mm off | ok, 0.024 mm, 9.7 ms | ok, 0.024 mm, 9.8 ms | ok, 0.012 mm, 15 ms |
| torus_ring | 0.05 | ok, 0.062 mm, 13 ms | ok, 0.054 mm, 5.9 ms | face 1 lies 3.391 mm off every face of the file it could be; its surface was evaluated wrongly, 5.000 mm off | ok, 0.091 mm, 4.0 ms | ok, 0.091 mm, 4.0 ms | ok, 0.053 mm, 4.5 ms |
| torus_ring | 0.1 | ok, 0.108 mm, 9.2 ms | ok, 0.125 mm, 3.8 ms | 5.000 mm off, 30 ms | ok, 0.189 mm, 3.3 ms | ok, 0.189 mm, 3.1 ms | ok, 0.105 mm, 3.0 ms |
| torus_ring | 0.5 | ok, 0.213 mm, 6.0 ms | ok, 0.564 mm, 3.3 ms | face 1 lies 5.000 mm off every face of the file it could be; its surface was evaluated wrongly, 11.352 mm off | ok, 0.813 mm, 2.3 ms | ok, 0.813 mm, 2.5 ms | ok, 0.390 mm, 1.5 ms |
| torus_ring_nurbs | 0.0125 | ok, 0.011 mm, 97 ms | face 1 empty, 5.000 mm off | ok, 0.008 mm, 103 ms | ok, 0.009 mm, 87 ms | ok, 0.010 mm, 39 ms | 0.132 mm off, 65 ms |
| torus_ring_nurbs | 0.05 | ok, 0.050 mm, 23 ms | face 1 empty, 5.000 mm off | ok, 0.033 mm, 28 ms | ok, 0.037 mm, 22 ms | ok, 0.041 mm, 10 ms | 0.362 mm off, 17 ms |
| torus_ring_nurbs | 0.1 | ok, 0.094 mm, 18 ms | face 1 empty, 5.000 mm off | ok, 0.088 mm, 21 ms | ok, 0.088 mm, 16 ms | ok, 0.088 mm, 9.6 ms | 0.630 mm off, 5.7 ms |
| torus_ring_nurbs | 0.5 | ok, 0.463 mm, 7.6 ms | face 1 empty, 5.000 mm off | ok, 0.441 mm, 7.7 ms | ok, 0.614 mm, 5.1 ms | ok, 0.441 mm, 3.8 ms | ok, 0.675 mm, 4.1 ms |
| washer | 0.0125 | ok, 0.006 mm, 9.9 ms | ok, 0.006 mm, 3.4 ms | ok, 0.007 mm, 9.0 ms | ok, 0.012 mm, 3.2 ms | ok, 0.012 mm, 3.3 ms | 1.000 mm off, 5.8 ms |
| washer | 0.05 | ok, 0.025 mm, 5.9 ms | ok, 0.024 mm, 2.7 ms | ok, 0.027 mm, 7.1 ms | ok, 0.049 mm, 2.7 ms | ok, 0.049 mm, 2.6 ms | 1.000 mm off, 3.7 ms |
| washer | 0.1 | ok, 0.049 mm, 5.7 ms | ok, 0.096 mm, 2.5 ms | ok, 0.093 mm, 7.2 ms | ok, 0.096 mm, 2.6 ms | ok, 0.096 mm, 2.7 ms | 1.000 mm off, 3.0 ms |
| washer | 0.5 | ok, 0.146 mm, 5.2 ms | ok, 0.384 mm, 2.5 ms | ok, 0.415 mm, 7.2 ms | ok, 0.495 mm, 2.4 ms | ok, 0.495 mm, 2.6 ms | ok, 1.030 mm, 3.0 ms |

**opencascade** passes 108 of 108.

**ogeom** passes 87 of 108.
- freeform_panel at 0.0125 mm: 5.468 mm off, 398 ms
- freeform_panel at 0.05 mm: 0.607 mm off, 202 ms
- freeform_panel at 0.1 mm: 0.608 mm off, 144 ms
- cone_point at 0.0125 mm: face 1 lies 0.080 mm off every face of the file it could be; its surface was evaluated wrongly, 0.092 mm off
- cone_point_vertex at 0.0125 mm: face 1 lies 0.080 mm off every face of the file it could be; its surface was evaluated wrongly, 0.092 mm off
- cylinder_nurbs at 0.0125 mm: face 1 empty, 15.000 mm off
- cylinder_nurbs at 0.05 mm: face 1 empty, 15.000 mm off
- cylinder_nurbs at 0.1 mm: face 1 empty, 15.000 mm off
- cylinder_nurbs at 0.5 mm: face 1 empty, 15.000 mm off
- dome at 0.0125 mm: face 1 lies 0.064 mm off every face of the file it could be; its surface was evaluated wrongly, 20.000 mm off
- dome at 0.05 mm: 20.000 mm off, 5.9 ms
- dome at 0.1 mm: 20.000 mm off, 3.9 ms
- dome at 0.5 mm: 20.000 mm off, 3.2 ms
- dome_nurbs at 0.0125 mm: face 1 empty, 20.000 mm off
- dome_nurbs at 0.05 mm: face 1 empty, 20.000 mm off
- dome_nurbs at 0.1 mm: face 1 empty, 20.000 mm off
- dome_nurbs at 0.5 mm: face 1 empty, 20.000 mm off
- torus_ring_nurbs at 0.0125 mm: face 1 empty, 5.000 mm off
- torus_ring_nurbs at 0.05 mm: face 1 empty, 5.000 mm off
- torus_ring_nurbs at 0.1 mm: face 1 empty, 5.000 mm off
- torus_ring_nurbs at 0.5 mm: face 1 empty, 5.000 mm off

**solidcraft** passes 92 of 108.
- freeform_panel at 0.05 mm: 5.306 mm off, 2.55 s
- freeform_panel at 0.1 mm: 5.306 mm off, 1.92 s
- freeform_panel at 0.5 mm: outside the file by 123.5 mm on +Y, 149.166 mm off
- cone_point at 0.0125 mm: 30.000 mm off, 4.1 ms
- cone_point at 0.05 mm: 30.000 mm off, 3.3 ms
- cone_point at 0.1 mm: 30.000 mm off, 3.5 ms
- cone_point at 0.5 mm: 30.000 mm off, 3.5 ms
- cone_point_vertex at 0.0125 mm: short of the vertices by 30.0 mm on +Z, 30.000 mm off
- cone_point_vertex at 0.05 mm: short of the vertices by 29.9 mm on +Z, 30.000 mm off
- cone_point_vertex at 0.1 mm: short of the vertices by 29.9 mm on +Z, 30.000 mm off
- cone_point_vertex at 0.5 mm: short of the vertices by 29.5 mm on +Z, 30.000 mm off
- tiny_pin at 0.5 mm: outside the file by 0.2 mm on +Y, 1.164 mm off
- torus_ring at 0.0125 mm: face 1 lies 3.482 mm off every face of the file it could be; its surface was evaluated wrongly, 5.000 mm off
- torus_ring at 0.05 mm: face 1 lies 3.391 mm off every face of the file it could be; its surface was evaluated wrongly, 5.000 mm off
- torus_ring at 0.1 mm: 5.000 mm off, 30 ms
- torus_ring at 0.5 mm: face 1 lies 5.000 mm off every face of the file it could be; its surface was evaluated wrongly, 11.352 mm off

**truck** passes 81 of 108.
- bowl_01 at 0.0125 mm: timed out at 60 s
- bowl_01 at 0.05 mm: 53.498 mm off, 4.21 s
- bowl_01 at 0.1 mm: 53.503 mm off, 3.88 s
- bowl_01 at 0.5 mm: 53.546 mm off, 6.4 ms
- freeform_panel at 0.0125 mm: timed out at 60 s
- freeform_panel at 0.05 mm: 12.172 mm off, 22.7 s
- freeform_panel at 0.1 mm: 12.172 mm off, 6.08 s
- freeform_panel at 0.5 mm: 12.172 mm off, 1.30 s
- sled_base at 0.0125 mm: 0.069 mm off, 147 ms
- sled_base at 0.1 mm: timed out at 60 s
- sled_base at 0.5 mm: 4.953 mm off, 45 ms
- cone_point at 0.0125 mm: outside the file by 0.3 mm on +Y, 30.000 mm off
- cone_point at 0.05 mm: outside the file by 0.3 mm on +Y, 30.000 mm off
- cone_point at 0.1 mm: outside the file by 0.2 mm on +Y, 30.000 mm off
- cone_point at 0.5 mm: 30.000 mm off, 2.2 ms
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
- bowl_01 at 0.0125 mm: 10.000 mm off, 244 ms
- bowl_01 at 0.05 mm: 0.218 mm off, 14 ms
- bowl_01 at 0.1 mm: 0.430 mm off, 7.8 ms
- bowl_01 at 0.5 mm: 2.111 mm off, 5.8 ms
- freeform_panel at 0.0125 mm: 12.172 mm off, 10.3 s
- freeform_panel at 0.05 mm: 12.172 mm off, 1.73 s
- freeform_panel at 0.1 mm: 12.172 mm off, 711 ms
- freeform_panel at 0.5 mm: 12.172 mm off, 126 ms
- sled_base at 0.05 mm: 0.818 mm off, 52 ms
- cone_point at 0.0125 mm: outside the file by 0.3 mm on +Y, 30.000 mm off
- cone_point at 0.05 mm: outside the file by 0.3 mm on +Y, 30.000 mm off
- cone_point at 0.1 mm: outside the file by 0.2 mm on +Y, 30.000 mm off
- cone_point at 0.5 mm: 30.000 mm off, 2.4 ms
- cone_point_vertex at 0.0125 mm: outside the file by 0.3 mm on +Y, 30.000 mm off
- cone_point_vertex at 0.05 mm: outside the file by 0.3 mm on +Y, 30.000 mm off
- cone_point_vertex at 0.1 mm: outside the file by 0.2 mm on +Y, 30.000 mm off
- cone_point_vertex at 0.5 mm: short of the vertices by 29.5 mm on +Z, 30.000 mm off

**brepkit** passes 50 of 108.
- bowl_01 at 0.0125 mm: could not read: read_step found no solid (it reads MANIFOLD_SOLID_BREP and BREP_WITH_VOIDS only)
- bowl_01 at 0.05 mm: could not read: read_step found no solid (it reads MANIFOLD_SOLID_BREP and BREP_WITH_VOIDS only)
- bowl_01 at 0.1 mm: could not read: read_step found no solid (it reads MANIFOLD_SOLID_BREP and BREP_WITH_VOIDS only)
- bowl_01 at 0.5 mm: could not read: read_step found no solid (it reads MANIFOLD_SOLID_BREP and BREP_WITH_VOIDS only)
- freeform_panel at 0.0125 mm: 14.374 mm off, 605 ms
- freeform_panel at 0.05 mm: 14.374 mm off, 440 ms
- freeform_panel at 0.1 mm: 14.374 mm off, 427 ms
- freeform_panel at 0.5 mm: nothing of the mesh lies on the file's face #2114; a missing face is a hole in the part, 31.325 mm off
- sled_base at 0.0125 mm: 3.792 mm off, 53 ms
- sled_base at 0.05 mm: 4.149 mm off, 30 ms
- sled_base at 0.1 mm: 4.111 mm off, 24 ms
- sled_base at 0.5 mm: face 357 empty, 5.959 mm off
- cone_point at 0.0125 mm: 14.813 mm off, 1.5 ms
- cone_point at 0.05 mm: 14.813 mm off, 1.5 ms
- cone_point at 0.1 mm: face 1 empty, 30.000 mm off
- cone_point at 0.5 mm: face 1 empty, 30.000 mm off
- cone_point_vertex at 0.0125 mm: could not read: parse error: failed to create wire from edge loop #44: empty wire — at least one
- cone_point_vertex at 0.05 mm: could not read: parse error: failed to create wire from edge loop #44: empty wire — at least one
- cone_point_vertex at 0.1 mm: could not read: parse error: failed to create wire from edge loop #44: empty wire — at least one
- cone_point_vertex at 0.5 mm: could not read: parse error: failed to create wire from edge loop #44: empty wire — at least one
- cylinder at 0.0125 mm: 14.469 mm off, 2.3 ms
- cylinder at 0.05 mm: 14.469 mm off, 1.7 ms
- cylinder at 0.1 mm: 14.469 mm off, 1.6 ms
- cylinder at 0.5 mm: 14.470 mm off, 1.4 ms
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
- dome_nurbs at 0.0125 mm: 3.906 mm off, 70 ms
- dome_nurbs at 0.05 mm: 3.907 mm off, 17 ms
- dome_nurbs at 0.1 mm: 3.913 mm off, 5.7 ms
- dome_nurbs at 0.5 mm: 3.938 mm off, 2.8 ms
- far_cylinder at 0.0125 mm: 14.469 mm off, 2.2 ms
- far_cylinder at 0.05 mm: 14.469 mm off, 1.7 ms
- far_cylinder at 0.1 mm: 14.469 mm off, 1.6 ms
- far_cylinder at 0.5 mm: 14.470 mm off, 1.5 ms
- sheet_disc at 0.0125 mm: 8.906 mm off, 38 ms
- sheet_disc at 0.05 mm: 8.906 mm off, 15 ms
- sheet_disc at 0.1 mm: 8.907 mm off, 8.2 ms
- sheet_disc at 0.5 mm: 8.920 mm off, 4.1 ms
- tiny_pin at 0.0125 mm: outside the file by 0.7 mm on -Z, 1.000 mm off
- tiny_pin at 0.05 mm: face 0 empty, 1000.000 mm off
- tiny_pin at 0.1 mm: face 0 empty, 1000.000 mm off
- tiny_pin at 0.5 mm: face 0 empty, 1000.000 mm off
- torus_ring_nurbs at 0.0125 mm: 0.132 mm off, 65 ms
- torus_ring_nurbs at 0.05 mm: 0.362 mm off, 17 ms
- torus_ring_nurbs at 0.1 mm: 0.630 mm off, 5.7 ms
- washer at 0.0125 mm: 1.000 mm off, 5.8 ms
- washer at 0.05 mm: 1.000 mm off, 3.7 ms
- washer at 0.1 mm: 1.000 mm off, 3.0 ms

## Speed and memory

Every run a reader passes is repeated, up to 5 times or for 10 s, and the median
taken. Time runs from starting the reader to its exit, CPU time is across all its
threads, reading and meshing are the reader's own account of them, and memory is
its peak resident memory.

Against opencascade, on the runs both pass: each figure is the reader's divided by
opencascade's, averaged geometrically, so 0.5× is twice as fast, or half the memory.

| reader | runs both pass | time | CPU time | reading | meshing | peak memory | triangles |
|---|---|---|---|---|---|---|---|
| ogeom | 87 | 0.47× | 0.81× | 0.74× | 0.25× | 0.37× | 1.3× |
| solidcraft | 92 | 1.1× | 2.1× | 1.3× | 1.1× | 0.42× | 1.4× |
| truck | 81 | 0.41× | 0.72× | 0.20× | 0.51× | 0.31× | 0.70× |
| truck-master | 91 | 0.40× | 0.68× | 0.21× | 0.49× | 0.32× | 0.64× |
| brepkit | 50 | 0.26× | 0.39× | 0.031× | 0.50× | 0.27× | 0.87× |

Every run that passes: the time, the reader's own reading and meshing, peak memory,
and triangles.

| part | tolerance mm | opencascade | ogeom | solidcraft | truck | truck-master | brepkit |
|---|---|---|---|---|---|---|---|
| bowl_01 | 0.0125 | 1.88 s (read 3.6 ms, mesh 1.87 s), 145 MB, 178,896 triangles | 76 ms (read 1.1 ms, mesh 73 ms), 15 MB, 42,680 triangles | 645 ms (read 13 ms, mesh 620 ms), 108 MB, 151,644 triangles | – | – | – |
| bowl_01 | 0.05 | 71 ms (read 4.0 ms, mesh 64 ms), 25 MB, 13,200 triangles | 17 ms (read 1.1 ms, mesh 15 ms), 7.8 MB, 11,022 triangles | 188 ms (read 13 ms, mesh 170 ms), 53 MB, 38,948 triangles | – | – | – |
| bowl_01 | 0.1 | 137 ms (read 3.5 ms, mesh 131 ms), 33 MB, 22,838 triangles | 9.3 ms (read 1.0 ms, mesh 7.3 ms), 6.7 MB, 6,670 triangles | 68 ms (read 12 ms, mesh 54 ms), 26 MB, 14,576 triangles | – | – | – |
| bowl_01 | 0.5 | 20 ms (read 3.5 ms, mesh 14 ms), 18 MB, 3,332 triangles | 4.4 ms (read 1.1 ms, mesh 2.6 ms), 5.5 MB, 1,806 triangles | 22 ms (read 13 ms, mesh 7.5 ms), 6.3 MB, 1,792 triangles | – | – | – |
| door_handle_guide | 0.0125 | 45 ms (read 28 ms, mesh 14 ms), 20 MB, 1,948 triangles | 22 ms (read 17 ms, mesh 4.2 ms), 9.4 MB, 7,588 triangles | 51 ms (read 29 ms, mesh 20 ms), 13 MB, 10,116 triangles | 15 ms (read 9.1 ms, mesh 4.6 ms), 6.4 MB, 1,152 triangles | 15 ms (read 9.2 ms, mesh 4.2 ms), 6.7 MB, 1,152 triangles | 7.9 ms (read 1.1 ms, mesh 5.5 ms), 4.9 MB, 1,202 triangles |
| door_handle_guide | 0.05 | 42 ms (read 28 ms, mesh 11 ms), 19 MB, 960 triangles | 21 ms (read 17 ms, mesh 2.7 ms), 7.4 MB, 2,524 triangles | 39 ms (read 29 ms, mesh 9.1 ms), 7.1 MB, 2,640 triangles | 15 ms (read 9.6 ms, mesh 3.9 ms), 6.0 MB, 696 triangles | 15 ms (read 9.4 ms, mesh 4.3 ms), 6.4 MB, 696 triangles | 7.7 ms (read 1.1 ms, mesh 4.8 ms), 4.6 MB, 742 triangles |
| door_handle_guide | 0.1 | 39 ms (read 26 ms, mesh 10 ms), 19 MB, 956 triangles | 18 ms (read 16 ms, mesh 1.8 ms), 7.1 MB, 1,052 triangles | 35 ms (read 27 ms, mesh 6.6 ms), 6.4 MB, 1,036 triangles | 14 ms (read 8.5 ms, mesh 4.2 ms), 6.0 MB, 568 triangles | 14 ms (read 8.5 ms, mesh 3.8 ms), 6.2 MB, 568 triangles | 6.3 ms (read 1.0 ms, mesh 4.2 ms), 4.4 MB, 734 triangles |
| door_handle_guide | 0.5 | 38 ms (read 26 ms, mesh 9.8 ms), 20 MB, 956 triangles | 18 ms (read 16 ms, mesh 1.7 ms), 7.1 MB, 1,052 triangles | 34 ms (read 26 ms, mesh 6.3 ms), 6.0 MB, 488 triangles | 13 ms (read 8.6 ms, mesh 3.7 ms), 6.0 MB, 396 triangles | 14 ms (read 8.7 ms, mesh 3.9 ms), 6.3 MB, 396 triangles | 6.8 ms (read 1.0 ms, mesh 4.6 ms), 4.6 MB, 734 triangles |
| freeform_panel | 0.0125 | 1.40 s (read 38 ms, mesh 1.36 s), 179 MB, 93,476 triangles | – | 5.72 s (read 928 ms, mesh 4.77 s), 164 MB, 254,420 triangles | – | – | – |
| freeform_panel | 0.05 | 234 ms (read 36 ms, mesh 194 ms), 48 MB, 23,106 triangles | – | – | – | – | – |
| freeform_panel | 0.1 | 136 ms (read 40 ms, mesh 94 ms), 33 MB, 11,626 triangles | – | – | – | – | – |
| freeform_panel | 0.5 | 62 ms (read 39 ms, mesh 20 ms), 21 MB, 2,566 triangles | 104 ms (read 85 ms, mesh 19 ms), 12 MB, 3,200 triangles | – | – | – | – |
| hinge_jig | 0.0125 | 15 ms (read 10 ms, mesh 3.0 ms), 17 MB, 74 triangles | 6.5 ms (read 4.7 ms, mesh 1.2 ms), 5.6 MB, 74 triangles | 13 ms (read 6.8 ms, mesh 5.0 ms), 4.7 MB, 74 triangles | 7.5 ms (read 3.4 ms, mesh 2.9 ms), 5.0 MB, 74 triangles | 7.5 ms (read 3.5 ms, mesh 3.0 ms), 5.3 MB, 74 triangles | 3.4 ms (read 0.4 ms, mesh 2.0 ms), 3.7 MB, 74 triangles |
| hinge_jig | 0.05 | 15 ms (read 10 ms, mesh 3.0 ms), 17 MB, 74 triangles | 6.5 ms (read 4.4 ms, mesh 1.5 ms), 5.5 MB, 74 triangles | 15 ms (read 7.6 ms, mesh 5.6 ms), 4.6 MB, 74 triangles | 7.6 ms (read 3.4 ms, mesh 3.2 ms), 5.1 MB, 74 triangles | 8.0 ms (read 3.5 ms, mesh 3.6 ms), 5.4 MB, 74 triangles | 3.7 ms (read 0.4 ms, mesh 2.3 ms), 3.7 MB, 74 triangles |
| hinge_jig | 0.1 | 16 ms (read 11 ms, mesh 3.1 ms), 17 MB, 74 triangles | 6.7 ms (read 4.8 ms, mesh 1.3 ms), 5.5 MB, 74 triangles | 15 ms (read 7.9 ms, mesh 5.4 ms), 4.8 MB, 74 triangles | 7.9 ms (read 3.4 ms, mesh 3.4 ms), 5.2 MB, 74 triangles | 7.8 ms (read 3.4 ms, mesh 3.0 ms), 5.4 MB, 74 triangles | 3.8 ms (read 0.4 ms, mesh 2.5 ms), 3.9 MB, 74 triangles |
| hinge_jig | 0.5 | 15 ms (read 10 ms, mesh 3.1 ms), 17 MB, 74 triangles | 6.8 ms (read 4.5 ms, mesh 1.3 ms), 5.6 MB, 74 triangles | 14 ms (read 7.3 ms, mesh 5.1 ms), 4.6 MB, 74 triangles | 7.4 ms (read 3.5 ms, mesh 3.0 ms), 5.2 MB, 74 triangles | 7.5 ms (read 3.4 ms, mesh 3.2 ms), 5.4 MB, 74 triangles | 3.7 ms (read 0.5 ms, mesh 2.3 ms), 3.7 MB, 74 triangles |
| lego_3_pins | 0.0125 | 183 ms (read 55 ms, mesh 126 ms), 26 MB, 27,592 triangles | 72 ms (read 60 ms, mesh 9.2 ms), 16 MB, 34,284 triangles | 203 ms (read 107 ms, mesh 90 ms), 25 MB, 49,134 triangles | 42 ms (read 20 ms, mesh 20 ms), 14 MB, 13,348 triangles | 43 ms (read 21 ms, mesh 20 ms), 15 MB, 13,348 triangles | 42 ms (read 2.3 ms, mesh 38 ms), 7.6 MB, 13,930 triangles |
| lego_3_pins | 0.05 | 124 ms (read 61 ms, mesh 59 ms), 24 MB, 11,786 triangles | 69 ms (read 61 ms, mesh 4.7 ms), 12 MB, 14,452 triangles | 150 ms (read 108 ms, mesh 40 ms), 12 MB, 13,368 triangles | 39 ms (read 20 ms, mesh 16 ms), 11 MB, 6,814 triangles | 37 ms (read 20 ms, mesh 15 ms), 12 MB, 6,814 triangles | 29 ms (read 2.2 ms, mesh 25 ms), 6.0 MB, 7,126 triangles |
| lego_3_pins | 0.1 | 101 ms (read 56 ms, mesh 42 ms), 24 MB, 8,570 triangles | 62 ms (read 57 ms, mesh 4.4 ms), 12 MB, 12,540 triangles | 130 ms (read 98 ms, mesh 31 ms), 11 MB, 10,056 triangles | 32 ms (read 19 ms, mesh 13 ms), 10 MB, 4,962 triangles | 31 ms (read 18 ms, mesh 11 ms), 10 MB, 4,962 triangles | 24 ms (read 2.2 ms, mesh 21 ms), 5.5 MB, 5,344 triangles |
| lego_3_pins | 0.5 | 97 ms (read 57 ms, mesh 37 ms), 24 MB, 6,680 triangles | 62 ms (read 59 ms, mesh 3.3 ms), 12 MB, 6,780 triangles | 121 ms (read 98 ms, mesh 22 ms), 9.2 MB, 3,552 triangles | 33 ms (read 20 ms, mesh 12 ms), 9.1 MB, 2,374 triangles | 32 ms (read 20 ms, mesh 10 ms), 9.3 MB, 2,374 triangles | 25 ms (read 2.3 ms, mesh 22 ms), 5.4 MB, 4,696 triangles |
| lego_4x2_01 | 0.0125 | 67 ms (read 21 ms, mesh 44 ms), 22 MB, 9,064 triangles | 28 ms (read 23 ms, mesh 4.8 ms), 9.5 MB, 10,336 triangles | 66 ms (read 35 ms, mesh 29 ms), 13 MB, 14,950 triangles | 15 ms (read 6.8 ms, mesh 6.8 ms), 8.0 MB, 4,356 triangles | 15 ms (read 7.0 ms, mesh 7.1 ms), 8.5 MB, 4,356 triangles | 15 ms (read 0.9 ms, mesh 13 ms), 6.9 MB, 4,550 triangles |
| lego_4x2_01 | 0.05 | 43 ms (read 21 ms, mesh 20 ms), 19 MB, 3,862 triangles | 29 ms (read 25 ms, mesh 3.0 ms), 8.0 MB, 4,528 triangles | 49 ms (read 34 ms, mesh 12 ms), 7.3 MB, 4,084 triangles | 14 ms (read 6.7 ms, mesh 5.7 ms), 6.7 MB, 2,226 triangles | 14 ms (read 6.9 ms, mesh 6.2 ms), 6.9 MB, 2,226 triangles | 11 ms (read 0.8 ms, mesh 9.0 ms), 5.5 MB, 2,330 triangles |
| lego_4x2_01 | 0.1 | 38 ms (read 21 ms, mesh 15 ms), 19 MB, 2,790 triangles | 30 ms (read 27 ms, mesh 3.0 ms), 8.1 MB, 4,080 triangles | 49 ms (read 36 ms, mesh 12 ms), 7.3 MB, 3,252 triangles | 13 ms (read 6.7 ms, mesh 4.8 ms), 6.4 MB, 1,622 triangles | 13 ms (read 6.7 ms, mesh 5.0 ms), 6.9 MB, 1,622 triangles | 9.7 ms (read 0.8 ms, mesh 7.7 ms), 4.9 MB, 1,736 triangles |
| lego_4x2_01 | 0.5 | 36 ms (read 21 ms, mesh 13 ms), 19 MB, 2,160 triangles | 26 ms (read 23 ms, mesh 2.4 ms), 7.9 MB, 2,160 triangles | 43 ms (read 35 ms, mesh 7.2 ms), 6.0 MB, 1,156 triangles | 12 ms (read 7.0 ms, mesh 4.1 ms), 5.7 MB, 778 triangles | 12 ms (read 6.9 ms, mesh 4.2 ms), 6.1 MB, 778 triangles | 9.4 ms (read 0.9 ms, mesh 7.5 ms), 5.0 MB, 1,520 triangles |
| rail_guide | 0.0125 | 71 ms (read 36 ms, mesh 32 ms), 23 MB, 6,484 triangles | 42 ms (read 37 ms, mesh 4.3 ms), 11 MB, 8,744 triangles | 74 ms (read 44 ms, mesh 25 ms), 13 MB, 11,808 triangles | 18 ms (read 12 ms, mesh 4.7 ms), 8.0 MB, 2,596 triangles | 18 ms (read 12 ms, mesh 4.9 ms), 8.3 MB, 2,596 triangles | 11 ms (read 1.6 ms, mesh 8.1 ms), 5.9 MB, 2,948 triangles |
| rail_guide | 0.05 | 58 ms (read 36 ms, mesh 19 ms), 22 MB, 2,884 triangles | 37 ms (read 34 ms, mesh 2.5 ms), 8.4 MB, 3,216 triangles | 63 ms (read 49 ms, mesh 13 ms), 8.5 MB, 4,076 triangles | 18 ms (read 13 ms, mesh 3.8 ms), 7.4 MB, 1,404 triangles | 19 ms (read 13 ms, mesh 4.3 ms), 7.6 MB, 1,404 triangles | 10.0 ms (read 1.8 ms, mesh 6.6 ms), 5.1 MB, 1,604 triangles |
| rail_guide | 0.1 | 60 ms (read 40 ms, mesh 18 ms), 22 MB, 2,412 triangles | 33 ms (read 30 ms, mesh 2.4 ms), 8.9 MB, 2,448 triangles | 61 ms (read 49 ms, mesh 11 ms), 7.4 MB, 2,648 triangles | 19 ms (read 13 ms, mesh 4.8 ms), 7.2 MB, 1,100 triangles | 21 ms (read 13 ms, mesh 4.8 ms), 7.4 MB, 1,100 triangles | 9.5 ms (read 1.6 ms, mesh 6.5 ms), 5.1 MB, 1,492 triangles |
| rail_guide | 0.5 | 60 ms (read 40 ms, mesh 18 ms), 22 MB, 2,244 triangles | 40 ms (read 37 ms, mesh 2.2 ms), 8.7 MB, 2,208 triangles | 57 ms (read 48 ms, mesh 7.8 ms), 6.8 MB, 804 triangles | 19 ms (read 13 ms, mesh 4.4 ms), 6.8 MB, 652 triangles | 19 ms (read 13 ms, mesh 4.6 ms), 7.4 MB, 652 triangles | 9.2 ms (read 1.7 ms, mesh 6.6 ms), 5.2 MB, 1,492 triangles |
| sled_base | 0.0125 | 268 ms (read 110 ms, mesh 154 ms), 35 MB, 33,822 triangles | 107 ms (read 90 ms, mesh 14 ms), 26 MB, 54,166 triangles | 345 ms (read 216 ms, mesh 118 ms), 46 MB, 67,572 triangles | – | 79 ms (read 36 ms, mesh 37 ms), 25 MB, 30,858 triangles | – |
| sled_base | 0.05 | 185 ms (read 110 ms, mesh 71 ms), 32 MB, 12,750 triangles | 88 ms (read 81 ms, mesh 5.3 ms), 20 MB, 17,434 triangles | 239 ms (read 200 ms, mesh 37 ms), 19 MB, 20,904 triangles | 70 ms (read 36 ms, mesh 31 ms), 22 MB, 22,127 triangles | – | – |
| sled_base | 0.1 | 197 ms (read 120 ms, mesh 70 ms), 32 MB, 9,320 triangles | 101 ms (read 93 ms, mesh 5.5 ms), 20 MB, 15,386 triangles | 252 ms (read 219 ms, mesh 32 ms), 17 MB, 16,112 triangles | – | 44 ms (read 36 ms, mesh 6.6 ms), 14 MB, 5,210 triangles | – |
| sled_base | 0.5 | 176 ms (read 116 ms, mesh 57 ms), 32 MB, 7,968 triangles | 92 ms (read 85 ms, mesh 5.3 ms), 20 MB, 13,610 triangles | 224 ms (read 209 ms, mesh 13 ms), 13 MB, 3,206 triangles | – | 43 ms (read 35 ms, mesh 6.1 ms), 12 MB, 2,194 triangles | – |
| cone_point | 0.0125 | 11 ms (read 2.0 ms, mesh 7.5 ms), 19 MB, 1,563 triangles | – | – | – | – | – |
| cone_point | 0.05 | 5.2 ms (read 1.8 ms, mesh 2.0 ms), 16 MB, 473 triangles | 3.2 ms (read 1.5 ms, mesh 1.1 ms), 5.3 MB, 653 triangles | – | – | – | – |
| cone_point | 0.1 | 4.6 ms (read 1.9 ms, mesh 1.2 ms), 16 MB, 293 triangles | 2.7 ms (read 1.4 ms, mesh 0.7 ms), 5.1 MB, 361 triangles | – | – | – | – |
| cone_point | 0.5 | 4.0 ms (read 2.0 ms, mesh 0.7 ms), 16 MB, 129 triangles | 2.4 ms (read 1.4 ms, mesh 0.4 ms), 5.3 MB, 131 triangles | – | – | – | – |
| cone_point_vertex | 0.0125 | 11 ms (read 2.0 ms, mesh 7.5 ms), 19 MB, 1,563 triangles | – | – | – | – | – |
| cone_point_vertex | 0.05 | 5.6 ms (read 1.9 ms, mesh 2.1 ms), 16 MB, 473 triangles | 3.4 ms (read 1.5 ms, mesh 1.2 ms), 5.4 MB, 653 triangles | – | – | – | – |
| cone_point_vertex | 0.1 | 4.6 ms (read 1.9 ms, mesh 1.2 ms), 16 MB, 293 triangles | 2.8 ms (read 1.4 ms, mesh 0.6 ms), 5.4 MB, 361 triangles | – | – | – | – |
| cone_point_vertex | 0.5 | 3.9 ms (read 1.8 ms, mesh 0.6 ms), 16 MB, 129 triangles | 2.5 ms (read 1.4 ms, mesh 0.4 ms), 5.2 MB, 131 triangles | – | – | – | – |
| cylinder | 0.0125 | 5.6 ms (read 2.1 ms, mesh 2.0 ms), 16 MB, 352 triangles | 3.0 ms (read 1.1 ms, mesh 1.2 ms), 5.3 MB, 882 triangles | 7.0 ms (read 2.3 ms, mesh 3.9 ms), 6.0 MB, 1,262 triangles | 3.1 ms (read 0.3 ms, mesh 1.3 ms), 4.4 MB, 248 triangles | 2.4 ms (read 0.4 ms, mesh 1.2 ms), 4.7 MB, 248 triangles | – |
| cylinder | 0.05 | 4.4 ms (read 2.0 ms, mesh 1.0 ms), 16 MB, 176 triangles | 2.3 ms (read 1.1 ms, mesh 0.5 ms), 5.0 MB, 310 triangles | 4.9 ms (read 2.3 ms, mesh 1.7 ms), 5.0 MB, 374 triangles | 2.2 ms (read 0.3 ms, mesh 1.1 ms), 4.4 MB, 124 triangles | 2.8 ms (read 0.3 ms, mesh 1.0 ms), 4.7 MB, 124 triangles | – |
| cylinder | 0.1 | 4.6 ms (read 2.1 ms, mesh 0.8 ms), 16 MB, 124 triangles | 2.4 ms (read 1.2 ms, mesh 0.6 ms), 5.1 MB, 310 triangles | 4.8 ms (read 2.4 ms, mesh 1.5 ms), 4.6 MB, 310 triangles | 2.3 ms (read 0.3 ms, mesh 0.9 ms), 4.3 MB, 88 triangles | 2.3 ms (read 0.3 ms, mesh 1.0 ms), 4.6 MB, 88 triangles | – |
| cylinder | 0.5 | 4.1 ms (read 2.0 ms, mesh 0.7 ms), 16 MB, 100 triangles | 2.0 ms (read 1.1 ms, mesh 0.3 ms), 5.0 MB, 90 triangles | 4.5 ms (read 2.4 ms, mesh 1.1 ms), 4.2 MB, 90 triangles | 2.3 ms (read 0.3 ms, mesh 0.8 ms), 4.4 MB, 36 triangles | 2.1 ms (read 0.4 ms, mesh 0.9 ms), 4.6 MB, 36 triangles | – |
| cylinder_inch | 0.0125 | 5.3 ms (read 1.9 ms, mesh 1.9 ms), 16 MB, 352 triangles | 3.6 ms (read 1.6 ms, mesh 1.2 ms), 5.3 MB, 882 triangles | 7.1 ms (read 2.3 ms, mesh 3.7 ms), 6.0 MB, 1,262 triangles | 2.8 ms (read 0.3 ms, mesh 1.3 ms), 4.3 MB, 248 triangles | 2.6 ms (read 0.4 ms, mesh 1.2 ms), 4.9 MB, 248 triangles | – |
| cylinder_inch | 0.05 | 4.4 ms (read 2.0 ms, mesh 1.0 ms), 16 MB, 176 triangles | 2.3 ms (read 1.1 ms, mesh 0.5 ms), 5.0 MB, 310 triangles | 4.8 ms (read 2.3 ms, mesh 1.9 ms), 4.9 MB, 374 triangles | 2.3 ms (read 0.3 ms, mesh 1.0 ms), 4.5 MB, 124 triangles | 2.7 ms (read 0.4 ms, mesh 1.0 ms), 4.7 MB, 124 triangles | – |
| cylinder_inch | 0.1 | 4.1 ms (read 2.0 ms, mesh 0.8 ms), 16 MB, 124 triangles | 2.3 ms (read 1.1 ms, mesh 0.5 ms), 5.1 MB, 310 triangles | 5.2 ms (read 2.4 ms, mesh 1.5 ms), 4.7 MB, 310 triangles | 2.8 ms (read 0.3 ms, mesh 1.1 ms), 4.4 MB, 88 triangles | 2.2 ms (read 0.4 ms, mesh 1.1 ms), 4.7 MB, 88 triangles | – |
| cylinder_inch | 0.5 | 4.0 ms (read 2.0 ms, mesh 0.7 ms), 16 MB, 100 triangles | 2.0 ms (read 1.1 ms, mesh 0.3 ms), 5.0 MB, 90 triangles | 4.9 ms (read 2.3 ms, mesh 1.1 ms), 4.2 MB, 90 triangles | 2.5 ms (read 0.3 ms, mesh 0.9 ms), 4.4 MB, 36 triangles | 2.2 ms (read 0.4 ms, mesh 0.9 ms), 4.6 MB, 36 triangles | – |
| cylinder_nurbs | 0.0125 | 9.0 ms (read 3.3 ms, mesh 4.2 ms), 18 MB, 372 triangles | – | 15 ms (read 2.7 ms, mesh 11 ms), 9.7 MB, 3,040 triangles | 55 ms (read 0.5 ms, mesh 52 ms), 13 MB, 24,508 triangles | 31 ms (read 0.6 ms, mesh 28 ms), 9.8 MB, 15,008 triangles | 4.2 ms (read 0.1 ms, mesh 3.5 ms), 4.0 MB, 322 triangles |
| cylinder_nurbs | 0.05 | 6.2 ms (read 3.2 ms, mesh 1.6 ms), 17 MB, 184 triangles | – | 7.8 ms (read 2.8 ms, mesh 4.2 ms), 5.6 MB, 846 triangles | 15 ms (read 0.5 ms, mesh 13 ms), 7.0 MB, 6,104 triangles | 7.4 ms (read 0.5 ms, mesh 5.9 ms), 6.2 MB, 3,098 triangles | 2.6 ms (read 0.1 ms, mesh 1.9 ms), 4.0 MB, 178 triangles |
| cylinder_nurbs | 0.1 | 5.8 ms (read 3.3 ms, mesh 1.1 ms), 17 MB, 132 triangles | – | 5.8 ms (read 2.5 ms, mesh 2.3 ms), 4.5 MB, 310 triangles | 6.7 ms (read 0.5 ms, mesh 4.9 ms), 5.5 MB, 2,046 triangles | 6.1 ms (read 0.6 ms, mesh 4.0 ms), 5.8 MB, 2,046 triangles | 2.7 ms (read 0.1 ms, mesh 1.9 ms), 4.0 MB, 178 triangles |
| cylinder_nurbs | 0.5 | 5.6 ms (read 3.1 ms, mesh 1.0 ms), 17 MB, 120 triangles | – | 5.6 ms (read 2.7 ms, mesh 1.7 ms), 4.1 MB, 90 triangles | 3.8 ms (read 0.5 ms, mesh 2.3 ms), 4.7 MB, 510 triangles | 4.0 ms (read 0.6 ms, mesh 2.1 ms), 5.1 MB, 510 triangles | 2.5 ms (read 0.1 ms, mesh 1.7 ms), 4.0 MB, 178 triangles |
| cylinder_placed | 0.0125 | 5.5 ms (read 2.0 ms, mesh 1.9 ms), 16 MB, 352 triangles | 3.4 ms (read 1.3 ms, mesh 1.3 ms), 5.3 MB, 882 triangles | 6.8 ms (read 2.2 ms, mesh 3.7 ms), 6.0 MB, 1,262 triangles | – | 2.5 ms (read 0.5 ms, mesh 1.2 ms), 4.9 MB, 248 triangles | – |
| cylinder_placed | 0.05 | 4.8 ms (read 2.1 ms, mesh 1.1 ms), 16 MB, 176 triangles | 2.3 ms (read 1.2 ms, mesh 0.6 ms), 5.1 MB, 310 triangles | 5.4 ms (read 2.7 ms, mesh 2.0 ms), 4.8 MB, 374 triangles | – | 2.4 ms (read 0.4 ms, mesh 1.1 ms), 4.7 MB, 124 triangles | – |
| cylinder_placed | 0.1 | 4.4 ms (read 2.1 ms, mesh 0.8 ms), 16 MB, 124 triangles | 2.3 ms (read 1.2 ms, mesh 0.5 ms), 5.1 MB, 310 triangles | 4.6 ms (read 2.2 ms, mesh 1.5 ms), 4.7 MB, 310 triangles | – | 2.5 ms (read 0.4 ms, mesh 1.1 ms), 4.9 MB, 88 triangles | – |
| cylinder_placed | 0.5 | 4.1 ms (read 2.0 ms, mesh 0.7 ms), 16 MB, 100 triangles | 2.1 ms (read 1.1 ms, mesh 0.3 ms), 5.0 MB, 90 triangles | 4.3 ms (read 2.4 ms, mesh 1.1 ms), 4.2 MB, 90 triangles | – | 2.5 ms (read 0.4 ms, mesh 0.9 ms), 4.7 MB, 36 triangles | – |
| cylinder_seam | 0.0125 | 5.3 ms (read 1.9 ms, mesh 2.0 ms), 16 MB, 352 triangles | 2.9 ms (read 1.1 ms, mesh 1.2 ms), 5.1 MB, 882 triangles | 7.5 ms (read 2.4 ms, mesh 4.0 ms), 6.0 MB, 1,262 triangles | 2.7 ms (read 0.4 ms, mesh 1.2 ms), 4.4 MB, 248 triangles | 2.6 ms (read 0.4 ms, mesh 1.3 ms), 4.9 MB, 248 triangles | 1.7 ms (read 0.1 ms, mesh 1.0 ms), 4.0 MB, 248 triangles |
| cylinder_seam | 0.05 | 4.3 ms (read 1.9 ms, mesh 1.0 ms), 16 MB, 176 triangles | 2.2 ms (read 1.2 ms, mesh 0.5 ms), 5.0 MB, 310 triangles | 5.5 ms (read 2.6 ms, mesh 1.8 ms), 4.8 MB, 374 triangles | 2.7 ms (read 0.3 ms, mesh 1.1 ms), 4.3 MB, 124 triangles | 2.5 ms (read 0.4 ms, mesh 1.0 ms), 4.7 MB, 124 triangles | 1.4 ms (read 0.1 ms, mesh 0.7 ms), 4.0 MB, 124 triangles |
| cylinder_seam | 0.1 | 4.1 ms (read 1.9 ms, mesh 0.8 ms), 16 MB, 124 triangles | 2.3 ms (read 1.2 ms, mesh 0.5 ms), 5.2 MB, 310 triangles | 5.0 ms (read 2.4 ms, mesh 1.7 ms), 4.7 MB, 310 triangles | 2.6 ms (read 0.3 ms, mesh 1.2 ms), 4.4 MB, 88 triangles | 2.2 ms (read 0.4 ms, mesh 1.0 ms), 4.7 MB, 88 triangles | 1.3 ms (read 0.1 ms, mesh 0.7 ms), 4.0 MB, 88 triangles |
| cylinder_seam | 0.5 | 3.9 ms (read 1.9 ms, mesh 0.7 ms), 16 MB, 100 triangles | 2.0 ms (read 1.1 ms, mesh 0.3 ms), 5.1 MB, 90 triangles | 4.6 ms (read 2.6 ms, mesh 1.1 ms), 4.2 MB, 90 triangles | 2.7 ms (read 0.4 ms, mesh 1.0 ms), 4.4 MB, 36 triangles | 2.2 ms (read 0.4 ms, mesh 0.9 ms), 4.7 MB, 36 triangles | 1.3 ms (read 0.1 ms, mesh 0.6 ms), 4.1 MB, 68 triangles |
| cylinder_twice | 0.0125 | 5.5 ms (read 2.0 ms, mesh 2.0 ms), 16 MB, 704 triangles | 4.3 ms (read 1.2 ms, mesh 2.4 ms), 5.5 MB, 1,764 triangles | 13 ms (read 4.0 ms, mesh 7.7 ms), 6.5 MB, 2,524 triangles | – | 3.0 ms (read 0.5 ms, mesh 1.6 ms), 5.1 MB, 496 triangles | – |
| cylinder_twice | 0.05 | 4.7 ms (read 2.1 ms, mesh 1.1 ms), 16 MB, 352 triangles | 2.8 ms (read 1.2 ms, mesh 1.0 ms), 5.0 MB, 620 triangles | 9.6 ms (read 3.7 ms, mesh 4.7 ms), 6.0 MB, 1,432 triangles | – | 3.3 ms (read 0.5 ms, mesh 1.6 ms), 4.9 MB, 248 triangles | – |
| cylinder_twice | 0.1 | 4.3 ms (read 2.0 ms, mesh 0.8 ms), 16 MB, 248 triangles | 2.9 ms (read 1.1 ms, mesh 1.1 ms), 5.1 MB, 620 triangles | 9.0 ms (read 4.5 ms, mesh 3.0 ms), 4.6 MB, 620 triangles | – | 2.6 ms (read 0.5 ms, mesh 1.2 ms), 4.7 MB, 176 triangles | – |
| cylinder_twice | 0.5 | 4.2 ms (read 2.1 ms, mesh 0.7 ms), 16 MB, 200 triangles | 2.3 ms (read 1.1 ms, mesh 0.6 ms), 5.2 MB, 180 triangles | 7.1 ms (read 3.8 ms, mesh 2.0 ms), 4.1 MB, 180 triangles | – | 2.7 ms (read 0.5 ms, mesh 1.3 ms), 4.7 MB, 72 triangles | – |
| dome | 0.0125 | 45 ms (read 2.1 ms, mesh 41 ms), 29 MB, 8,157 triangles | – | 56 ms (read 3.5 ms, mesh 50 ms), 28 MB, 14,571 triangles | 7.7 ms (read 0.3 ms, mesh 6.4 ms), 6.3 MB, 4,111 triangles | 7.9 ms (read 0.3 ms, mesh 6.1 ms), 6.5 MB, 4,111 triangles | 49 ms (read 0.1 ms, mesh 48 ms), 22 MB, 31,344 triangles |
| dome | 0.05 | 9.8 ms (read 1.9 ms, mesh 6.5 ms), 18 MB, 2,061 triangles | – | 16 ms (read 3.2 ms, mesh 12 ms), 11 MB, 3,639 triangles | 3.5 ms (read 0.3 ms, mesh 2.2 ms), 5.1 MB, 1,087 triangles | 3.3 ms (read 0.3 ms, mesh 2.1 ms), 5.4 MB, 1,087 triangles | 11 ms (read 0.0 ms, mesh 10 ms), 8.3 MB, 7,860 triangles |
| dome | 0.1 | 6.7 ms (read 1.9 ms, mesh 3.2 ms), 17 MB, 1,091 triangles | – | 13 ms (read 3.2 ms, mesh 8.9 ms), 9.5 MB, 2,485 triangles | 3.0 ms (read 0.2 ms, mesh 1.6 ms), 4.7 MB, 511 triangles | 3.0 ms (read 0.3 ms, mesh 1.5 ms), 4.9 MB, 511 triangles | 6.0 ms (read 0.0 ms, mesh 5.2 ms), 5.9 MB, 3,948 triangles |
| dome | 0.5 | 4.2 ms (read 1.9 ms, mesh 0.9 ms), 16 MB, 269 triangles | – | 6.2 ms (read 3.4 ms, mesh 1.8 ms), 5.0 MB, 403 triangles | 2.2 ms (read 0.3 ms, mesh 1.0 ms), 4.4 MB, 119 triangles | 2.3 ms (read 0.3 ms, mesh 1.0 ms), 4.8 MB, 119 triangles | 2.1 ms (read 0.0 ms, mesh 1.3 ms), 4.3 MB, 744 triangles |
| dome_nurbs | 0.0125 | 129 ms (read 3.2 ms, mesh 123 ms), 34 MB, 20,385 triangles | – | 62 ms (read 3.3 ms, mesh 56 ms), 28 MB, 13,623 triangles | 65 ms (read 0.6 ms, mesh 61 ms), 13 MB, 26,987 triangles | 32 ms (read 0.6 ms, mesh 30 ms), 10 MB, 16,827 triangles | – |
| dome_nurbs | 0.05 | 31 ms (read 3.2 ms, mesh 26 ms), 20 MB, 4,925 triangles | – | 20 ms (read 3.4 ms, mesh 14 ms), 10 MB, 3,385 triangles | 17 ms (read 0.5 ms, mesh 15 ms), 7.1 MB, 6,709 triangles | 8.9 ms (read 0.5 ms, mesh 7.1 ms), 6.6 MB, 4,189 triangles | – |
| dome_nurbs | 0.1 | 18 ms (read 3.1 ms, mesh 13 ms), 18 MB, 2,467 triangles | – | 13 ms (read 3.2 ms, mesh 9.0 ms), 7.6 MB, 2,385 triangles | 12 ms (read 0.5 ms, mesh 10 ms), 6.3 MB, 4,295 triangles | 8.2 ms (read 0.5 ms, mesh 6.0 ms), 6.2 MB, 3,388 triangles | – |
| dome_nurbs | 0.5 | 7.7 ms (read 3.1 ms, mesh 2.9 ms), 17 MB, 565 triangles | – | 6.9 ms (read 3.3 ms, mesh 2.3 ms), 5.0 MB, 409 triangles | 4.3 ms (read 0.5 ms, mesh 2.0 ms), 4.7 MB, 427 triangles | 3.0 ms (read 0.5 ms, mesh 1.5 ms), 4.9 MB, 277 triangles | – |
| far_cylinder | 0.0125 | 6.1 ms (read 2.1 ms, mesh 2.5 ms), 16 MB, 352 triangles | 3.3 ms (read 1.2 ms, mesh 1.3 ms), 5.5 MB, 882 triangles | 7.6 ms (read 2.5 ms, mesh 4.1 ms), 6.0 MB, 1,262 triangles | 2.4 ms (read 0.3 ms, mesh 1.2 ms), 4.5 MB, 248 triangles | 2.5 ms (read 0.4 ms, mesh 1.2 ms), 4.9 MB, 248 triangles | – |
| far_cylinder | 0.05 | 4.7 ms (read 2.1 ms, mesh 1.1 ms), 16 MB, 176 triangles | 2.4 ms (read 1.2 ms, mesh 0.6 ms), 4.9 MB, 310 triangles | 7.3 ms (read 2.5 ms, mesh 3.4 ms), 5.8 MB, 1,058 triangles | 2.9 ms (read 0.4 ms, mesh 1.3 ms), 4.5 MB, 124 triangles | 2.9 ms (read 0.4 ms, mesh 1.2 ms), 4.7 MB, 124 triangles | – |
| far_cylinder | 0.1 | 4.4 ms (read 2.1 ms, mesh 0.8 ms), 16 MB, 124 triangles | 2.4 ms (read 1.2 ms, mesh 0.5 ms), 5.2 MB, 310 triangles | 4.8 ms (read 2.4 ms, mesh 1.6 ms), 4.7 MB, 310 triangles | 2.7 ms (read 0.3 ms, mesh 1.2 ms), 4.3 MB, 88 triangles | 2.7 ms (read 0.4 ms, mesh 1.0 ms), 4.8 MB, 88 triangles | – |
| far_cylinder | 0.5 | 4.3 ms (read 2.1 ms, mesh 0.7 ms), 16 MB, 100 triangles | 2.2 ms (read 1.2 ms, mesh 0.3 ms), 5.1 MB, 90 triangles | 4.4 ms (read 2.4 ms, mesh 1.1 ms), 4.2 MB, 90 triangles | 2.5 ms (read 0.3 ms, mesh 1.1 ms), 4.4 MB, 36 triangles | 3.1 ms (read 0.4 ms, mesh 1.1 ms), 4.7 MB, 36 triangles | – |
| grooved_shaft | 0.0125 | 728 ms (read 158 ms, mesh 557 ms), 51 MB, 132,852 triangles | 440 ms (read 397 ms, mesh 33 ms), 58 MB, 190,878 triangles | 651 ms (read 372 ms, mesh 264 ms), 94 MB, 168,380 triangles | 112 ms (read 61 ms, mesh 42 ms), 55 MB, 94,248 triangles | 120 ms (read 63 ms, mesh 50 ms), 56 MB, 94,248 triangles | 143 ms (read 7.5 ms, mesh 130 ms), 41 MB, 94,748 triangles |
| grooved_shaft | 0.05 | 391 ms (read 152 ms, mesh 233 ms), 40 MB, 67,176 triangles | 415 ms (read 394 ms, mesh 17 ms), 58 MB, 94,686 triangles | 508 ms (read 372 ms, mesh 127 ms), 57 MB, 76,188 triangles | 93 ms (read 61 ms, mesh 27 ms), 41 MB, 47,624 triangles | 98 ms (read 64 ms, mesh 27 ms), 42 MB, 47,624 triangles | 85 ms (read 7.6 ms, mesh 72 ms), 23 MB, 48,124 triangles |
| grooved_shaft | 0.1 | 328 ms (read 153 ms, mesh 170 ms), 39 MB, 47,624 triangles | 430 ms (read 403 ms, mesh 19 ms), 58 MB, 94,686 triangles | 460 ms (read 379 ms, mesh 73 ms), 42 MB, 48,124 triangles | 85 ms (read 62 ms, mesh 20 ms), 35 MB, 34,088 triangles | 87 ms (read 64 ms, mesh 17 ms), 36 MB, 34,088 triangles | 64 ms (read 7.8 ms, mesh 54 ms), 22 MB, 34,588 triangles |
| grooved_shaft | 0.5 | 312 ms (read 155 ms, mesh 149 ms), 38 MB, 39,100 triangles | 423 ms (read 403 ms, mesh 15 ms), 57 MB, 70,590 triangles | 427 ms (read 380 ms, mesh 44 ms), 30 MB, 24,060 triangles | 78 ms (read 62 ms, mesh 12 ms), 23 MB, 15,036 triangles | 79 ms (read 64 ms, mesh 12 ms), 24 MB, 15,036 triangles | 54 ms (read 7.6 ms, mesh 44 ms), 15 MB, 27,068 triangles |
| plate_100_holes | 0.0125 | 308 ms (read 61 ms, mesh 243 ms), 52 MB, 20,252 triangles | 99 ms (read 56 ms, mesh 36 ms), 28 MB, 89,868 triangles | 453 ms (read 72 ms, mesh 368 ms), 68 MB, 127,788 triangles | 114 ms (read 14 ms, mesh 98 ms), 14 MB, 14,572 triangles | 118 ms (read 14 ms, mesh 101 ms), 15 MB, 14,572 triangles | 61 ms (read 1.7 ms, mesh 58 ms), 15 MB, 14,596 triangles |
| plate_100_holes | 0.05 | 159 ms (read 61 ms, mesh 95 ms), 34 MB, 10,940 triangles | 79 ms (read 61 ms, mesh 16 ms), 17 MB, 32,108 triangles | 202 ms (read 71 ms, mesh 123 ms), 26 MB, 38,308 triangles | 48 ms (read 13 ms, mesh 32 ms), 12 MB, 7,692 triangles | 49 ms (read 15 ms, mesh 31 ms), 13 MB, 7,692 triangles | 34 ms (read 1.8 ms, mesh 30 ms), 10 MB, 7,820 triangles |
| plate_100_holes | 0.1 | 160 ms (read 61 ms, mesh 96 ms), 35 MB, 10,924 triangles | 68 ms (read 60 ms, mesh 6.9 ms), 13 MB, 9,900 triangles | 115 ms (read 72 ms, mesh 39 ms), 14 MB, 12,900 triangles | 34 ms (read 14 ms, mesh 19 ms), 9.4 MB, 5,676 triangles | 35 ms (read 14 ms, mesh 20 ms), 10 MB, 5,676 triangles | 31 ms (read 1.7 ms, mesh 29 ms), 11 MB, 7,820 triangles |
| plate_100_holes | 0.5 | 163 ms (read 61 ms, mesh 99 ms), 34 MB, 10,924 triangles | 67 ms (read 59 ms, mesh 6.9 ms), 13 MB, 9,900 triangles | 89 ms (read 71 ms, mesh 16 ms), 8.9 MB, 5,044 triangles | 22 ms (read 14 ms, mesh 7.7 ms), 8.1 MB, 2,844 triangles | 23 ms (read 15 ms, mesh 7.1 ms), 8.3 MB, 2,844 triangles | 32 ms (read 1.7 ms, mesh 29 ms), 11 MB, 7,820 triangles |
| plate_900_holes | 0.0125 | 14.8 s (read 1.67 s, mesh 13.1 s), 1000 MB, 180,252 triangles | 879 ms (read 534 ms, mesh 319 ms), 148 MB, 801,868 triangles | 19.3 s (read 554 ms, mesh 18.7 s), 535 MB, 1,142,188 triangles | 5.26 s (read 117 ms, mesh 5.13 s), 98 MB, 129,772 triangles | 5.66 s (read 122 ms, mesh 5.52 s), 106 MB, 129,772 triangles | 2.13 s (read 15 ms, mesh 2.11 s), 102 MB, 129,796 triangles |
| plate_900_holes | 0.05 | 6.22 s (read 1.65 s, mesh 4.55 s), 395 MB, 97,340 triangles | 681 ms (read 531 ms, mesh 142 ms), 88 MB, 286,508 triangles | 5.53 s (read 562 ms, mesh 4.93 s), 171 MB, 342,308 triangles | 1.38 s (read 116 ms, mesh 1.25 s), 61 MB, 68,492 triangles | 1.48 s (read 123 ms, mesh 1.35 s), 65 MB, 68,492 triangles | 880 ms (read 16 ms, mesh 859 ms), 60 MB, 68,620 triangles |
| plate_900_holes | 0.1 | 6.18 s (read 1.68 s, mesh 4.48 s), 395 MB, 97,324 triangles | 592 ms (read 529 ms, mesh 59 ms), 79 MB, 88,300 triangles | 1.66 s (read 555 ms, mesh 1.09 s), 81 MB, 115,300 triangles | 799 ms (read 115 ms, mesh 674 ms), 59 MB, 50,476 triangles | 851 ms (read 120 ms, mesh 717 ms), 62 MB, 50,476 triangles | 853 ms (read 15 ms, mesh 834 ms), 60 MB, 68,620 triangles |
| plate_900_holes | 0.5 | 6.00 s (read 1.64 s, mesh 4.35 s), 395 MB, 97,324 triangles | 582 ms (read 523 ms, mesh 55 ms), 79 MB, 88,300 triangles | 858 ms (read 556 ms, mesh 294 ms), 54 MB, 45,044 triangles | 266 ms (read 117 ms, mesh 144 ms), 39 MB, 25,244 triangles | 280 ms (read 120 ms, mesh 153 ms), 40 MB, 25,244 triangles | 854 ms (read 15 ms, mesh 834 ms), 60 MB, 68,620 triangles |
| sheet_disc | 0.0125 | 81 ms (read 2.2 ms, mesh 77 ms), 31 MB, 4,768 triangles | 25 ms (read 8.3 ms, mesh 16 ms), 7.6 MB, 5,112 triangles | 39 ms (read 4.9 ms, mesh 33 ms), 11 MB, 4,092 triangles | 18 ms (read 0.3 ms, mesh 17 ms), 6.4 MB, 3,368 triangles | 19 ms (read 0.4 ms, mesh 17 ms), 6.9 MB, 3,368 triangles | – |
| sheet_disc | 0.05 | 27 ms (read 2.1 ms, mesh 23 ms), 20 MB, 2,384 triangles | 15 ms (read 8.3 ms, mesh 6.4 ms), 6.5 MB, 3,064 triangles | 17 ms (read 4.5 ms, mesh 11 ms), 7.4 MB, 2,044 triangles | 6.8 ms (read 0.3 ms, mesh 5.4 ms), 5.2 MB, 1,684 triangles | 7.5 ms (read 0.4 ms, mesh 5.6 ms), 5.7 MB, 1,684 triangles | – |
| sheet_disc | 0.1 | 18 ms (read 2.2 ms, mesh 14 ms), 19 MB, 1,684 triangles | 15 ms (read 8.2 ms, mesh 6.4 ms), 6.8 MB, 3,064 triangles | 17 ms (read 4.9 ms, mesh 11 ms), 7.4 MB, 2,044 triangles | 5.2 ms (read 0.3 ms, mesh 3.6 ms), 5.2 MB, 1,192 triangles | 4.9 ms (read 0.3 ms, mesh 3.5 ms), 5.4 MB, 1,192 triangles | – |
| sheet_disc | 0.5 | 8.1 ms (read 2.0 ms, mesh 4.7 ms), 17 MB, 752 triangles | 11 ms (read 8.3 ms, mesh 2.3 ms), 6.6 MB, 1,530 triangles | 9.8 ms (read 4.5 ms, mesh 4.3 ms), 5.8 MB, 956 triangles | 2.7 ms (read 0.3 ms, mesh 1.6 ms), 4.8 MB, 532 triangles | 3.4 ms (read 0.4 ms, mesh 1.6 ms), 5.2 MB, 532 triangles | – |
| tiny_pin | 0.0125 | 4.4 ms (read 2.1 ms, mesh 0.8 ms), 16 MB, 100 triangles | 1.9 ms (read 0.9 ms, mesh 0.4 ms), 5.1 MB, 150 triangles | 4.5 ms (read 2.3 ms, mesh 1.3 ms), 4.5 MB, 180 triangles | 2.1 ms (read 0.3 ms, mesh 1.0 ms), 4.4 MB, 36 triangles | 2.3 ms (read 0.3 ms, mesh 1.0 ms), 4.7 MB, 36 triangles | – |
| tiny_pin | 0.05 | 4.3 ms (read 2.0 ms, mesh 0.8 ms), 16 MB, 100 triangles | 2.0 ms (read 1.0 ms, mesh 0.4 ms), 5.1 MB, 150 triangles | 4.5 ms (read 2.5 ms, mesh 1.3 ms), 4.2 MB, 56 triangles | 2.4 ms (read 0.3 ms, mesh 1.0 ms), 4.5 MB, 16 triangles | 2.8 ms (read 0.4 ms, mesh 1.2 ms), 4.7 MB, 16 triangles | – |
| tiny_pin | 0.1 | 4.6 ms (read 2.1 ms, mesh 0.9 ms), 16 MB, 100 triangles | 2.1 ms (read 1.0 ms, mesh 0.4 ms), 4.9 MB, 150 triangles | 5.2 ms (read 2.4 ms, mesh 1.1 ms), 4.2 MB, 36 triangles | 2.3 ms (read 0.3 ms, mesh 0.9 ms), 4.3 MB, 12 triangles | 2.2 ms (read 0.4 ms, mesh 0.9 ms), 4.9 MB, 12 triangles | – |
| tiny_pin | 0.5 | 4.3 ms (read 2.0 ms, mesh 0.8 ms), 16 MB, 100 triangles | 2.1 ms (read 0.9 ms, mesh 0.4 ms), 5.1 MB, 150 triangles | – | 2.5 ms (read 0.3 ms, mesh 1.0 ms), 4.6 MB, 8 triangles | 2.2 ms (read 0.4 ms, mesh 0.9 ms), 4.7 MB, 8 triangles | – |
| torus_ring | 0.0125 | 53 ms (read 2.3 ms, mesh 48 ms), 27 MB, 12,218 triangles | 14 ms (read 2.0 ms, mesh 11 ms), 7.4 MB, 8,448 triangles | – | 9.7 ms (read 0.3 ms, mesh 8.2 ms), 6.5 MB, 4,772 triangles | 9.8 ms (read 0.3 ms, mesh 8.1 ms), 6.7 MB, 4,772 triangles | 15 ms (read 0.1 ms, mesh 14 ms), 11 MB, 15,978 triangles |
| torus_ring | 0.05 | 13 ms (read 2.1 ms, mesh 9.5 ms), 19 MB, 3,102 triangles | 5.9 ms (read 2.1 ms, mesh 3.1 ms), 5.9 MB, 2,176 triangles | – | 4.0 ms (read 0.3 ms, mesh 2.7 ms), 5.0 MB, 1,286 triangles | 4.0 ms (read 0.3 ms, mesh 2.5 ms), 5.2 MB, 1,286 triangles | 4.5 ms (read 0.1 ms, mesh 3.8 ms), 5.8 MB, 4,084 triangles |
| torus_ring | 0.1 | 9.2 ms (read 2.1 ms, mesh 5.6 ms), 17 MB, 1,894 triangles | 3.8 ms (read 2.0 ms, mesh 1.2 ms), 5.6 MB, 832 triangles | – | 3.3 ms (read 0.3 ms, mesh 1.8 ms), 4.7 MB, 632 triangles | 3.1 ms (read 0.3 ms, mesh 1.8 ms), 4.9 MB, 632 triangles | 3.0 ms (read 0.1 ms, mesh 2.2 ms), 4.8 MB, 2,108 triangles |
| torus_ring | 0.5 | 6.0 ms (read 2.1 ms, mesh 2.5 ms), 16 MB, 780 triangles | 3.3 ms (read 2.0 ms, mesh 0.6 ms), 5.5 MB, 288 triangles | – | 2.3 ms (read 0.3 ms, mesh 1.1 ms), 4.4 MB, 154 triangles | 2.5 ms (read 0.3 ms, mesh 1.0 ms), 4.9 MB, 154 triangles | 1.5 ms (read 0.1 ms, mesh 0.9 ms), 4.1 MB, 432 triangles |
| torus_ring_nurbs | 0.0125 | 97 ms (read 4.0 ms, mesh 90 ms), 29 MB, 14,690 triangles | – | 103 ms (read 4.0 ms, mesh 95 ms), 43 MB, 20,974 triangles | 87 ms (read 0.7 ms, mesh 83 ms), 14 MB, 32,832 triangles | 39 ms (read 0.7 ms, mesh 35 ms), 10 MB, 16,384 triangles | – |
| torus_ring_nurbs | 0.05 | 23 ms (read 3.8 ms, mesh 17 ms), 19 MB, 3,692 triangles | – | 28 ms (read 3.7 ms, mesh 23 ms), 14 MB, 5,366 triangles | 22 ms (read 0.7 ms, mesh 19 ms), 7.5 MB, 8,224 triangles | 10 ms (read 0.7 ms, mesh 8.1 ms), 6.6 MB, 4,096 triangles | – |
| torus_ring_nurbs | 0.1 | 18 ms (read 3.8 ms, mesh 12 ms), 18 MB, 2,344 triangles | – | 21 ms (read 3.7 ms, mesh 16 ms), 10 MB, 4,024 triangles | 16 ms (read 0.7 ms, mesh 15 ms), 7.0 MB, 6,514 triangles | 9.6 ms (read 0.7 ms, mesh 7.7 ms), 6.4 MB, 3,770 triangles | – |
| torus_ring_nurbs | 0.5 | 7.6 ms (read 3.8 ms, mesh 2.4 ms), 17 MB, 500 triangles | – | 7.7 ms (read 3.8 ms, mesh 3.1 ms), 4.8 MB, 396 triangles | 5.1 ms (read 0.6 ms, mesh 3.6 ms), 4.8 MB, 960 triangles | 3.8 ms (read 0.6 ms, mesh 2.2 ms), 5.0 MB, 576 triangles | 4.1 ms (read 0.1 ms, mesh 3.2 ms), 4.4 MB, 1,082 triangles |
| washer | 0.0125 | 9.9 ms (read 3.0 ms, mesh 4.9 ms), 17 MB, 756 triangles | 3.4 ms (read 1.3 ms, mesh 1.4 ms), 5.2 MB, 1,148 triangles | 9.0 ms (read 3.8 ms, mesh 3.7 ms), 5.1 MB, 894 triangles | 3.2 ms (read 0.4 ms, mesh 1.6 ms), 4.9 MB, 536 triangles | 3.3 ms (read 0.5 ms, mesh 1.8 ms), 5.2 MB, 536 triangles | – |
| washer | 0.05 | 5.9 ms (read 2.5 ms, mesh 2.1 ms), 16 MB, 380 triangles | 2.7 ms (read 1.3 ms, mesh 0.7 ms), 5.1 MB, 572 triangles | 7.1 ms (read 3.7 ms, mesh 2.5 ms), 4.5 MB, 384 triangles | 2.7 ms (read 0.4 ms, mesh 1.2 ms), 4.5 MB, 272 triangles | 2.6 ms (read 0.5 ms, mesh 1.3 ms), 4.7 MB, 272 triangles | – |
| washer | 0.1 | 5.7 ms (read 2.5 ms, mesh 1.6 ms), 16 MB, 284 triangles | 2.5 ms (read 1.3 ms, mesh 0.5 ms), 5.1 MB, 284 triangles | 7.2 ms (read 3.9 ms, mesh 2.6 ms), 4.3 MB, 288 triangles | 2.6 ms (read 0.4 ms, mesh 1.3 ms), 4.4 MB, 192 triangles | 2.7 ms (read 0.5 ms, mesh 1.3 ms), 4.7 MB, 192 triangles | – |
| washer | 0.5 | 5.2 ms (read 2.5 ms, mesh 1.3 ms), 16 MB, 208 triangles | 2.5 ms (read 1.4 ms, mesh 0.5 ms), 5.0 MB, 188 triangles | 7.2 ms (read 3.9 ms, mesh 2.0 ms), 4.2 MB, 96 triangles | 2.4 ms (read 0.4 ms, mesh 1.1 ms), 4.4 MB, 88 triangles | 2.6 ms (read 0.5 ms, mesh 1.1 ms), 4.7 MB, 88 triangles | 3.0 ms (read 0.1 ms, mesh 1.9 ms), 4.2 MB, 144 triangles |
