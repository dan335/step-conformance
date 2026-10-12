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
| bowl_01 | 0.0125 | ok, 0.042 mm, 1.84 s | ok, 0.021 mm, 73 ms | ok, 0.012 mm, 632 ms | timed out at 60 s | 10.000 mm off, 272 ms | could not read: read_step found no solid (it reads MANIFOLD_SOLID_BREP and BREP_WITH_VOIDS only) |
| bowl_01 | 0.05 | ok, 0.050 mm, 75 ms | ok, 0.082 mm, 17 ms | ok, 0.047 mm, 192 ms | 53.498 mm off, 4.32 s | 0.218 mm off, 13 ms | could not read: read_step found no solid (it reads MANIFOLD_SOLID_BREP and BREP_WITH_VOIDS only) |
| bowl_01 | 0.1 | ok, 0.098 mm, 154 ms | ok, 0.105 mm, 11 ms | ok, 0.075 mm, 73 ms | 53.503 mm off, 4.23 s | 0.430 mm off, 9.0 ms | could not read: read_step found no solid (it reads MANIFOLD_SOLID_BREP and BREP_WITH_VOIDS only) |
| bowl_01 | 0.5 | ok, 0.464 mm, 22 ms | ok, 0.416 mm, 5.0 ms | ok, 0.401 mm, 23 ms | 53.546 mm off, 9.0 ms | 2.111 mm off, 4.3 ms | could not read: read_step found no solid (it reads MANIFOLD_SOLID_BREP and BREP_WITH_VOIDS only) |
| door_handle_guide | 0.0125 | ok, 0.008 mm, 48 ms | ok, 0.008 mm, 25 ms | ok, 0.008 mm, 54 ms | ok, 0.012 mm, 15 ms | ok, 0.012 mm, 15 ms | ok, 0.017 mm, 8.0 ms |
| door_handle_guide | 0.05 | ok, 0.020 mm, 44 ms | ok, 0.031 mm, 20 ms | ok, 0.034 mm, 40 ms | ok, 0.044 mm, 15 ms | ok, 0.044 mm, 15 ms | ok, 0.064 mm, 7.1 ms |
| door_handle_guide | 0.1 | ok, 0.020 mm, 45 ms | ok, 0.061 mm, 20 ms | ok, 0.066 mm, 39 ms | ok, 0.089 mm, 15 ms | ok, 0.089 mm, 15 ms | ok, 0.039 mm, 7.6 ms |
| door_handle_guide | 0.5 | ok, 0.020 mm, 44 ms | ok, 0.061 mm, 21 ms | ok, 0.242 mm, 36 ms | ok, 0.353 mm, 14 ms | ok, 0.353 mm, 14 ms | ok, 0.039 mm, 7.9 ms |
| freeform_panel | 0.0125 | 0.605 mm off, 1.58 s | 5.468 mm off, 403 ms | 0.473 mm off, 5.98 s | timed out at 60 s | 12.172 mm off, 11.1 s | 14.374 mm off, 752 ms |
| freeform_panel | 0.05 | 0.586 mm off, 257 ms | ok, 0.156 mm, 215 ms | 5.306 mm off, 2.69 s | 12.172 mm off, 25.0 s | 12.172 mm off, 1.73 s | 14.374 mm off, 493 ms |
| freeform_panel | 0.1 | 0.602 mm off, 164 ms | ok, 0.294 mm, 144 ms | 5.306 mm off, 2.09 s | 12.172 mm off, 6.55 s | 12.172 mm off, 667 ms | 14.374 mm off, 458 ms |
| freeform_panel | 0.5 | ok, 0.481 mm, 59 ms | ok, 1.187 mm, 112 ms | outside the file by 123.5 mm on +Y, 149.165 mm off | 12.172 mm off, 1.50 s | 12.172 mm off, 132 ms | nothing of the mesh lies on the file's face #2114; a missing face is a hole in the part, 31.436 mm off |
| hinge_jig | 0.0125 | ok, 0.000 mm, 17 ms | ok, 0.000 mm, 6.9 ms | ok, 0.000 mm, 16 ms | ok, 0.000 mm, 8.3 ms | ok, 0.000 mm, 9.4 ms | ok, 0.000 mm, 4.6 ms |
| hinge_jig | 0.05 | ok, 0.000 mm, 17 ms | ok, 0.000 mm, 6.1 ms | ok, 0.000 mm, 13 ms | ok, 0.000 mm, 7.9 ms | ok, 0.000 mm, 8.2 ms | ok, 0.000 mm, 3.8 ms |
| hinge_jig | 0.1 | ok, 0.000 mm, 15 ms | ok, 0.000 mm, 6.0 ms | ok, 0.000 mm, 13 ms | ok, 0.000 mm, 7.5 ms | ok, 0.000 mm, 7.7 ms | ok, 0.000 mm, 3.6 ms |
| hinge_jig | 0.5 | ok, 0.000 mm, 15 ms | ok, 0.000 mm, 8.3 ms | ok, 0.000 mm, 15 ms | ok, 0.000 mm, 7.1 ms | ok, 0.000 mm, 7.4 ms | ok, 0.000 mm, 3.4 ms |
| lego_3_pins | 0.0125 | ok, 0.008 mm, 215 ms | ok, 0.011 mm, 78 ms | ok, 0.012 mm, 214 ms | ok, 0.012 mm, 44 ms | ok, 0.012 mm, 44 ms | ok, 0.017 mm, 46 ms |
| lego_3_pins | 0.05 | ok, 0.030 mm, 147 ms | ok, 0.044 mm, 68 ms | ok, 0.049 mm, 151 ms | ok, 0.048 mm, 35 ms | ok, 0.048 mm, 36 ms | ok, 0.096 mm, 27 ms |
| lego_3_pins | 0.1 | ok, 0.058 mm, 120 ms | ok, 0.061 mm, 69 ms | ok, 0.066 mm, 144 ms | ok, 0.094 mm, 35 ms | ok, 0.094 mm, 35 ms | ok, 0.183 mm, 29 ms |
| lego_3_pins | 0.5 | ok, 0.067 mm, 107 ms | ok, 0.177 mm, 71 ms | ok, 0.298 mm, 137 ms | ok, 0.498 mm, 34 ms | ok, 0.498 mm, 33 ms | ok, 0.140 mm, 26 ms |
| lego_4x2_01 | 0.0125 | ok, 0.008 mm, 69 ms | ok, 0.011 mm, 33 ms | ok, 0.012 mm, 66 ms | ok, 0.012 mm, 14 ms | ok, 0.012 mm, 16 ms | ok, 0.017 mm, 15 ms |
| lego_4x2_01 | 0.05 | ok, 0.030 mm, 41 ms | ok, 0.044 mm, 30 ms | ok, 0.049 mm, 47 ms | ok, 0.048 mm, 13 ms | ok, 0.048 mm, 13 ms | ok, 0.096 mm, 11 ms |
| lego_4x2_01 | 0.1 | ok, 0.058 mm, 35 ms | ok, 0.061 mm, 32 ms | ok, 0.066 mm, 44 ms | ok, 0.094 mm, 14 ms | ok, 0.094 mm, 13 ms | ok, 0.183 mm, 11 ms |
| lego_4x2_01 | 0.5 | ok, 0.067 mm, 43 ms | ok, 0.177 mm, 31 ms | ok, 0.298 mm, 43 ms | ok, 0.498 mm, 14 ms | ok, 0.498 mm, 13 ms | ok, 0.140 mm, 9.9 ms |
| rail_guide | 0.0125 | ok, 0.007 mm, 78 ms | ok, 0.013 mm, 41 ms | ok, 0.012 mm, 80 ms | ok, 0.012 mm, 21 ms | ok, 0.012 mm, 21 ms | ok, 0.014 mm, 12 ms |
| rail_guide | 0.05 | ok, 0.028 mm, 68 ms | ok, 0.048 mm, 42 ms | ok, 0.048 mm, 63 ms | ok, 0.048 mm, 18 ms | ok, 0.048 mm, 19 ms | ok, 0.039 mm, 10 ms |
| rail_guide | 0.1 | ok, 0.047 mm, 67 ms | ok, 0.055 mm, 44 ms | ok, 0.064 mm, 58 ms | ok, 0.095 mm, 19 ms | ok, 0.095 mm, 20 ms | ok, 0.078 mm, 9.7 ms |
| rail_guide | 0.5 | ok, 0.069 mm, 62 ms | ok, 0.122 mm, 38 ms | ok, 0.483 mm, 55 ms | ok, 0.483 mm, 17 ms | ok, 0.483 mm, 17 ms | ok, 0.078 mm, 8.7 ms |
| sled_base | 0.0125 | ok, 0.014 mm, 344 ms | ok, 0.017 mm, 112 ms | ok, 0.012 mm, 354 ms | 0.069 mm off, 155 ms | ok, 0.021 mm, 74 ms | 3.792 mm off, 58 ms |
| sled_base | 0.05 | ok, 0.052 mm, 202 ms | ok, 0.067 mm, 99 ms | ok, 0.049 mm, 264 ms | ok, 0.084 mm, 76 ms | 0.818 mm off, 51 ms | 4.149 mm off, 29 ms |
| sled_base | 0.1 | ok, 0.094 mm, 190 ms | ok, 0.119 mm, 101 ms | ok, 0.098 mm, 241 ms | timed out at 60 s | ok, 0.223 mm, 42 ms | 4.111 mm off, 24 ms |
| sled_base | 0.5 | ok, 0.349 mm, 173 ms | ok, 0.252 mm, 98 ms | ok, 0.483 mm, 237 ms | 4.953 mm off, 46 ms | ok, 0.621 mm, 45 ms | face 357 empty, 5.959 mm off |
| cone_point | 0.0125 | ok, 0.011 mm, 12 ms | face 1 lies 0.080 mm off every face of the file it could be; its surface was evaluated wrongly, 0.092 mm off | 30.000 mm off, 5.1 ms | outside the file by 0.3 mm on +Y, 30.000 mm off | outside the file by 0.3 mm on +Y, 30.000 mm off | 14.813 mm off, 1.6 ms |
| cone_point | 0.05 | ok, 0.039 mm, 5.5 ms | ok, 0.137 mm, 3.4 ms | 30.000 mm off, 3.6 ms | outside the file by 0.3 mm on +Y, 30.000 mm off | outside the file by 0.3 mm on +Y, 30.000 mm off | 14.813 mm off, 1.3 ms |
| cone_point | 0.1 | ok, 0.073 mm, 4.8 ms | ok, 0.235 mm, 2.9 ms | 30.000 mm off, 3.5 ms | outside the file by 0.2 mm on +Y, 30.000 mm off | outside the file by 0.2 mm on +Y, 30.000 mm off | face 1 empty, 30.000 mm off |
| cone_point | 0.5 | ok, 0.205 mm, 4.9 ms | ok, 0.256 mm, 2.6 ms | 30.000 mm off, 3.7 ms | 30.000 mm off, 2.4 ms | 30.000 mm off, 2.2 ms | face 1 empty, 30.000 mm off |
| cone_point_vertex | 0.0125 | ok, 0.011 mm, 12 ms | face 1 lies 0.080 mm off every face of the file it could be; its surface was evaluated wrongly, 0.092 mm off | short of the vertices by 30.0 mm on +Z, 30.000 mm off | outside the file by 0.3 mm on +Y, 30.000 mm off | outside the file by 0.3 mm on +Y, 30.000 mm off | could not read: parse error: failed to create wire from edge loop #44: empty wire — at least one |
| cone_point_vertex | 0.05 | ok, 0.039 mm, 5.9 ms | ok, 0.137 mm, 3.3 ms | short of the vertices by 29.9 mm on +Z, 30.000 mm off | outside the file by 0.3 mm on +Y, 30.000 mm off | outside the file by 0.3 mm on +Y, 30.000 mm off | could not read: parse error: failed to create wire from edge loop #44: empty wire — at least one |
| cone_point_vertex | 0.1 | ok, 0.073 mm, 4.9 ms | ok, 0.235 mm, 3.1 ms | short of the vertices by 29.9 mm on +Z, 30.000 mm off | outside the file by 0.2 mm on +Y, 30.000 mm off | outside the file by 0.2 mm on +Y, 30.000 mm off | could not read: parse error: failed to create wire from edge loop #44: empty wire — at least one |
| cone_point_vertex | 0.5 | ok, 0.205 mm, 4.1 ms | ok, 0.256 mm, 2.7 ms | short of the vertices by 29.5 mm on +Z, 30.000 mm off | short of the vertices by 29.5 mm on +Z, 30.000 mm off | short of the vertices by 29.5 mm on +Z, 30.000 mm off | could not read: parse error: failed to create wire from edge loop #44: empty wire — at least one |
| cylinder | 0.0125 | ok, 0.006 mm, 5.8 ms | ok, 0.012 mm, 3.1 ms | ok, 0.012 mm, 8.2 ms | ok, 0.012 mm, 2.5 ms | ok, 0.012 mm, 2.8 ms | 14.469 mm off, 2.4 ms |
| cylinder | 0.05 | ok, 0.024 mm, 5.0 ms | ok, 0.048 mm, 2.4 ms | ok, 0.050 mm, 5.1 ms | ok, 0.048 mm, 2.2 ms | ok, 0.048 mm, 2.4 ms | 14.469 mm off, 1.8 ms |
| cylinder | 0.1 | ok, 0.048 mm, 4.4 ms | ok, 0.048 mm, 2.4 ms | ok, 0.053 mm, 5.0 ms | ok, 0.093 mm, 2.7 ms | ok, 0.093 mm, 2.8 ms | 14.469 mm off, 1.9 ms |
| cylinder | 0.5 | ok, 0.073 mm, 4.9 ms | ok, 0.192 mm, 2.6 ms | ok, 0.218 mm, 4.3 ms | ok, 0.489 mm, 2.3 ms | ok, 0.489 mm, 2.3 ms | 14.470 mm off, 1.5 ms |
| cylinder_inch | 0.0125 | ok, 0.006 mm, 6.1 ms | ok, 0.012 mm, 3.3 ms | ok, 0.012 mm, 7.1 ms | ok, 0.012 mm, 2.6 ms | ok, 0.012 mm, 2.8 ms | outside the file by 15.4 mm on -Z, 25.400 mm off |
| cylinder_inch | 0.05 | ok, 0.024 mm, 4.9 ms | ok, 0.048 mm, 2.4 ms | ok, 0.050 mm, 5.5 ms | ok, 0.048 mm, 2.5 ms | ok, 0.048 mm, 3.0 ms | outside the file by 15.3 mm on -Z, 25.400 mm off |
| cylinder_inch | 0.1 | ok, 0.048 mm, 4.4 ms | ok, 0.048 mm, 2.6 ms | ok, 0.053 mm, 5.3 ms | ok, 0.093 mm, 2.5 ms | ok, 0.093 mm, 2.5 ms | outside the file by 15.3 mm on -Z, 25.400 mm off |
| cylinder_inch | 0.5 | ok, 0.073 mm, 4.3 ms | ok, 0.192 mm, 2.2 ms | ok, 0.218 mm, 4.2 ms | ok, 0.489 mm, 2.5 ms | ok, 0.489 mm, 2.5 ms | outside the file by 14.9 mm on -Z, 25.400 mm off |
| cylinder_nurbs | 0.0125 | ok, 0.006 mm, 9.4 ms | face 1 empty, 15.000 mm off | ok, 0.012 mm, 17 ms | ok, 0.012 mm, 59 ms | ok, 0.012 mm, 31 ms | ok, 0.013 mm, 4.6 ms |
| cylinder_nurbs | 0.05 | ok, 0.025 mm, 7.1 ms | face 1 empty, 15.000 mm off | ok, 0.046 mm, 7.1 ms | ok, 0.050 mm, 16 ms | ok, 0.050 mm, 8.4 ms | ok, 0.040 mm, 2.8 ms |
| cylinder_nurbs | 0.1 | ok, 0.049 mm, 7.1 ms | face 1 empty, 15.000 mm off | ok, 0.053 mm, 6.4 ms | ok, 0.053 mm, 7.1 ms | ok, 0.053 mm, 6.0 ms | ok, 0.040 mm, 2.7 ms |
| cylinder_nurbs | 0.5 | ok, 0.077 mm, 5.8 ms | face 1 empty, 15.000 mm off | ok, 0.207 mm, 5.0 ms | ok, 0.207 mm, 4.0 ms | ok, 0.207 mm, 4.1 ms | ok, 0.040 mm, 2.3 ms |
| cylinder_placed | 0.0125 | ok, 0.006 mm, 5.6 ms | ok, 0.012 mm, 2.9 ms | ok, 0.012 mm, 7.2 ms | outside the file by 40.0 mm on -X, 56.630 mm off | ok, 0.012 mm, 3.0 ms | outside the file by 40.0 mm on -X, 56.957 mm off |
| cylinder_placed | 0.05 | ok, 0.024 mm, 4.7 ms | ok, 0.048 mm, 2.2 ms | ok, 0.050 mm, 4.6 ms | outside the file by 39.9 mm on -X, 56.630 mm off | ok, 0.048 mm, 2.6 ms | outside the file by 39.9 mm on -X, 56.948 mm off |
| cylinder_placed | 0.1 | ok, 0.048 mm, 4.2 ms | ok, 0.048 mm, 2.4 ms | ok, 0.053 mm, 4.5 ms | outside the file by 39.8 mm on -X, 56.667 mm off | ok, 0.093 mm, 2.2 ms | outside the file by 39.9 mm on -X, 57.025 mm off |
| cylinder_placed | 0.5 | ok, 0.073 mm, 4.3 ms | ok, 0.192 mm, 2.6 ms | ok, 0.218 mm, 4.6 ms | outside the file by 39.5 mm on -X, 56.851 mm off | ok, 0.489 mm, 2.5 ms | outside the file by 39.3 mm on -X, 56.947 mm off |
| cylinder_seam | 0.0125 | ok, 0.006 mm, 5.2 ms | ok, 0.012 mm, 3.1 ms | ok, 0.012 mm, 7.8 ms | ok, 0.012 mm, 2.7 ms | ok, 0.012 mm, 2.6 ms | ok, 0.012 mm, 1.8 ms |
| cylinder_seam | 0.05 | ok, 0.024 mm, 4.6 ms | ok, 0.048 mm, 2.7 ms | ok, 0.048 mm, 5.3 ms | ok, 0.048 mm, 2.5 ms | ok, 0.048 mm, 2.7 ms | ok, 0.048 mm, 1.4 ms |
| cylinder_seam | 0.1 | ok, 0.048 mm, 4.2 ms | ok, 0.048 mm, 2.5 ms | ok, 0.053 mm, 5.5 ms | ok, 0.093 mm, 2.6 ms | ok, 0.093 mm, 2.8 ms | ok, 0.093 mm, 1.4 ms |
| cylinder_seam | 0.5 | ok, 0.073 mm, 4.3 ms | ok, 0.192 mm, 2.2 ms | ok, 0.207 mm, 5.1 ms | ok, 0.489 mm, 2.7 ms | ok, 0.489 mm, 2.7 ms | ok, 0.152 mm, 1.4 ms |
| cylinder_twice | 0.0125 | ok, 0.006 mm, 6.1 ms | ok, 0.012 mm, 4.5 ms | ok, 0.012 mm, 14 ms | short of the vertices by 80.0 mm on +X, 80.555 mm off | ok, 0.012 mm, 3.4 ms | short of the vertices by 80.0 mm on +X, 81.677 mm off |
| cylinder_twice | 0.05 | ok, 0.024 mm, 4.8 ms | ok, 0.048 mm, 3.1 ms | ok, 0.050 mm, 11 ms | short of the vertices by 79.9 mm on +X, 80.595 mm off | ok, 0.048 mm, 3.3 ms | short of the vertices by 79.9 mm on +X, 81.716 mm off |
| cylinder_twice | 0.1 | ok, 0.048 mm, 5.2 ms | ok, 0.048 mm, 3.5 ms | ok, 0.053 mm, 8.8 ms | short of the vertices by 79.9 mm on +X, 80.623 mm off | ok, 0.093 mm, 3.9 ms | short of the vertices by 79.9 mm on +X, 81.741 mm off |
| cylinder_twice | 0.5 | ok, 0.073 mm, 4.9 ms | ok, 0.192 mm, 2.6 ms | ok, 0.218 mm, 6.9 ms | short of the vertices by 79.5 mm on +X, 80.623 mm off | ok, 0.489 mm, 2.9 ms | short of the vertices by 79.7 mm on +X, 81.708 mm off |
| dome | 0.0125 | ok, 0.012 mm, 48 ms | face 1 lies 0.064 mm off every face of the file it could be; its surface was evaluated wrongly, 20.000 mm off | ok, 0.012 mm, 58 ms | ok, 0.024 mm, 8.6 ms | ok, 0.024 mm, 8.5 ms | ok, 0.012 mm, 53 ms |
| dome | 0.05 | ok, 0.048 mm, 11 ms | 20.000 mm off, 6.6 ms | ok, 0.047 mm, 18 ms | ok, 0.091 mm, 4.2 ms | ok, 0.091 mm, 3.7 ms | ok, 0.049 mm, 12 ms |
| dome | 0.1 | ok, 0.083 mm, 6.9 ms | 20.000 mm off, 4.7 ms | ok, 0.093 mm, 15 ms | ok, 0.191 mm, 3.0 ms | ok, 0.191 mm, 2.9 ms | ok, 0.096 mm, 6.8 ms |
| dome | 0.5 | ok, 0.345 mm, 4.5 ms | 20.000 mm off, 3.5 ms | ok, 0.415 mm, 5.9 ms | ok, 0.797 mm, 2.4 ms | ok, 0.797 mm, 2.4 ms | ok, 0.304 mm, 2.1 ms |
| dome_nurbs | 0.0125 | ok, 0.013 mm, 144 ms | face 1 empty, 20.000 mm off | ok, 0.012 mm, 62 ms | ok, 0.008 mm, 66 ms | ok, 0.008 mm, 33 ms | 3.906 mm off, 62 ms |
| dome_nurbs | 0.05 | ok, 0.050 mm, 29 ms | face 1 empty, 20.000 mm off | ok, 0.047 mm, 17 ms | ok, 0.032 mm, 15 ms | ok, 0.032 mm, 8.5 ms | 3.907 mm off, 16 ms |
| dome_nurbs | 0.1 | ok, 0.087 mm, 16 ms | face 1 empty, 20.000 mm off | ok, 0.093 mm, 12 ms | ok, 0.131 mm, 11 ms | ok, 0.100 mm, 7.5 ms | 3.913 mm off, 5.3 ms |
| dome_nurbs | 0.5 | ok, 0.400 mm, 6.7 ms | face 1 empty, 20.000 mm off | ok, 0.415 mm, 6.5 ms | ok, 0.538 mm, 3.5 ms | ok, 0.538 mm, 3.1 ms | 3.938 mm off, 2.6 ms |
| far_cylinder | 0.0125 | ok, 0.006 mm, 5.9 ms | ok, 0.012 mm, 3.0 ms | ok, 0.012 mm, 7.2 ms | ok, 0.012 mm, 2.3 ms | ok, 0.012 mm, 2.5 ms | 14.469 mm off, 2.0 ms |
| far_cylinder | 0.05 | ok, 0.024 mm, 4.5 ms | ok, 0.048 mm, 2.4 ms | ok, 0.046 mm, 6.7 ms | ok, 0.048 mm, 2.2 ms | ok, 0.048 mm, 2.2 ms | 14.469 mm off, 1.6 ms |
| far_cylinder | 0.1 | ok, 0.048 mm, 4.2 ms | ok, 0.048 mm, 2.3 ms | ok, 0.053 mm, 5.2 ms | ok, 0.093 mm, 2.2 ms | ok, 0.093 mm, 2.3 ms | 14.469 mm off, 1.5 ms |
| far_cylinder | 0.5 | ok, 0.073 mm, 4.1 ms | ok, 0.192 mm, 2.1 ms | ok, 0.216 mm, 3.7 ms | ok, 0.489 mm, 2.1 ms | ok, 0.489 mm, 2.3 ms | 14.470 mm off, 1.4 ms |
| grooved_shaft | 0.0125 | ok, 0.006 mm, 759 ms | ok, 0.012 mm, 538 ms | ok, 0.012 mm, 692 ms | ok, 0.012 mm, 109 ms | ok, 0.012 mm, 119 ms | ok, 0.012 mm, 160 ms |
| grooved_shaft | 0.05 | ok, 0.024 mm, 441 ms | ok, 0.048 mm, 725 ms | ok, 0.046 mm, 520 ms | ok, 0.048 mm, 95 ms | ok, 0.048 mm, 101 ms | ok, 0.048 mm, 95 ms |
| grooved_shaft | 0.1 | ok, 0.048 mm, 482 ms | ok, 0.048 mm, 877 ms | ok, 0.053 mm, 544 ms | ok, 0.093 mm, 125 ms | ok, 0.093 mm, 161 ms | ok, 0.093 mm, 78 ms |
| grooved_shaft | 0.5 | ok, 0.073 mm, 370 ms | ok, 0.189 mm, 510 ms | ok, 0.207 mm, 466 ms | ok, 0.489 mm, 84 ms | ok, 0.489 mm, 91 ms | ok, 0.152 mm, 62 ms |
| plate_100_holes | 0.0125 | ok, 0.005 mm, 393 ms | ok, 0.005 mm, 146 ms | ok, 0.005 mm, 501 ms | ok, 0.012 mm, 119 ms | ok, 0.012 mm, 123 ms | ok, 0.012 mm, 63 ms |
| plate_100_holes | 0.05 | ok, 0.022 mm, 177 ms | ok, 0.019 mm, 78 ms | ok, 0.020 mm, 203 ms | ok, 0.049 mm, 47 ms | ok, 0.049 mm, 49 ms | ok, 0.046 mm, 32 ms |
| plate_100_holes | 0.1 | ok, 0.025 mm, 172 ms | ok, 0.077 mm, 69 ms | ok, 0.082 mm, 114 ms | ok, 0.087 mm, 35 ms | ok, 0.087 mm, 36 ms | ok, 0.046 mm, 33 ms |
| plate_100_holes | 0.5 | ok, 0.025 mm, 167 ms | ok, 0.077 mm, 69 ms | ok, 0.303 mm, 94 ms | ok, 0.402 mm, 23 ms | ok, 0.402 mm, 23 ms | ok, 0.046 mm, 32 ms |
| plate_900_holes | 0.0125 | ok, 0.006 mm, 17.4 s | ok, 0.004 mm, 948 ms | ok, 0.005 mm, 21.3 s | ok, 0.012 mm, 6.61 s | ok, 0.012 mm, 6.80 s | ok, 0.012 mm, 2.29 s |
| plate_900_holes | 0.05 | ok, 0.022 mm, 6.93 s | ok, 0.017 mm, 731 ms | ok, 0.018 mm, 5.93 s | ok, 0.049 mm, 1.47 s | ok, 0.049 mm, 1.53 s | ok, 0.046 mm, 896 ms |
| plate_900_holes | 0.1 | ok, 0.025 mm, 6.79 s | ok, 0.077 mm, 601 ms | ok, 0.083 mm, 1.75 s | ok, 0.087 mm, 840 ms | ok, 0.087 mm, 881 ms | ok, 0.046 mm, 911 ms |
| plate_900_holes | 0.5 | ok, 0.025 mm, 6.72 s | ok, 0.077 mm, 598 ms | ok, 0.304 mm, 877 ms | ok, 0.401 mm, 265 ms | ok, 0.401 mm, 280 ms | ok, 0.046 mm, 854 ms |
| sheet_disc | 0.0125 | ok, 0.006 mm, 91 ms | ok, 0.008 mm, 26 ms | ok, 0.009 mm, 40 ms | ok, 0.012 mm, 19 ms | ok, 0.012 mm, 20 ms | 8.906 mm off, 40 ms |
| sheet_disc | 0.05 | ok, 0.025 mm, 28 ms | ok, 0.030 mm, 16 ms | ok, 0.037 mm, 18 ms | ok, 0.050 mm, 7.7 ms | ok, 0.050 mm, 8.1 ms | 8.906 mm off, 15 ms |
| sheet_disc | 0.1 | ok, 0.050 mm, 18 ms | ok, 0.030 mm, 16 ms | ok, 0.037 mm, 18 ms | ok, 0.099 mm, 5.1 ms | ok, 0.099 mm, 5.6 ms | 8.907 mm off, 8.7 ms |
| sheet_disc | 0.5 | ok, 0.249 mm, 8.4 ms | ok, 0.120 mm, 12 ms | ok, 0.443 mm, 11 ms | ok, 0.495 mm, 3.2 ms | ok, 0.495 mm, 3.3 ms | 8.920 mm off, 4.1 ms |
| tiny_pin | 0.0125 | ok, 0.002 mm, 4.4 ms | ok, 0.005 mm, 2.1 ms | ok, 0.005 mm, 4.6 ms | ok, 0.012 mm, 2.3 ms | ok, 0.012 mm, 2.6 ms | outside the file by 0.7 mm on -Z, 1.000 mm off |
| tiny_pin | 0.05 | ok, 0.002 mm, 5.2 ms | ok, 0.005 mm, 2.5 ms | ok, 0.024 mm, 4.4 ms | ok, 0.048 mm, 2.3 ms | ok, 0.048 mm, 2.3 ms | face 0 empty, 1000.000 mm off |
| tiny_pin | 0.1 | ok, 0.002 mm, 4.7 ms | ok, 0.005 mm, 2.1 ms | ok, 0.073 mm, 4.5 ms | ok, 0.073 mm, 2.6 ms | ok, 0.073 mm, 2.6 ms | face 0 empty, 1000.000 mm off |
| tiny_pin | 0.5 | ok, 0.002 mm, 4.4 ms | ok, 0.005 mm, 2.1 ms | outside the file by 0.2 mm on +Y, 1.164 mm off | ok, 0.125 mm, 2.3 ms | ok, 0.125 mm, 2.4 ms | face 0 empty, 1000.000 mm off |
| torus_ring | 0.0125 | ok, 0.016 mm, 57 ms | ok, 0.014 mm, 15 ms | face 1 lies 3.482 mm off every face of the file it could be; its surface was evaluated wrongly, 5.000 mm off | ok, 0.024 mm, 11 ms | ok, 0.024 mm, 10 ms | ok, 0.012 mm, 15 ms |
| torus_ring | 0.05 | ok, 0.062 mm, 14 ms | ok, 0.054 mm, 6.0 ms | face 1 lies 3.391 mm off every face of the file it could be; its surface was evaluated wrongly, 5.000 mm off | ok, 0.091 mm, 4.1 ms | ok, 0.091 mm, 4.2 ms | ok, 0.053 mm, 4.8 ms |
| torus_ring | 0.1 | ok, 0.108 mm, 10 ms | ok, 0.125 mm, 4.1 ms | 5.000 mm off, 30 ms | ok, 0.189 mm, 3.1 ms | ok, 0.189 mm, 3.2 ms | ok, 0.105 mm, 3.0 ms |
| torus_ring | 0.5 | ok, 0.213 mm, 6.4 ms | ok, 0.564 mm, 3.5 ms | face 1 lies 5.000 mm off every face of the file it could be; its surface was evaluated wrongly, 11.352 mm off | ok, 0.813 mm, 2.4 ms | ok, 0.813 mm, 2.6 ms | ok, 0.390 mm, 1.6 ms |
| torus_ring_nurbs | 0.0125 | ok, 0.011 mm, 106 ms | face 1 empty, 5.000 mm off | ok, 0.008 mm, 108 ms | ok, 0.009 mm, 88 ms | ok, 0.010 mm, 40 ms | 0.132 mm off, 68 ms |
| torus_ring_nurbs | 0.05 | ok, 0.050 mm, 23 ms | face 1 empty, 5.000 mm off | ok, 0.033 mm, 28 ms | ok, 0.037 mm, 21 ms | ok, 0.041 mm, 10 ms | 0.362 mm off, 18 ms |
| torus_ring_nurbs | 0.1 | ok, 0.094 mm, 18 ms | face 1 empty, 5.000 mm off | ok, 0.088 mm, 22 ms | ok, 0.088 mm, 17 ms | ok, 0.088 mm, 10 ms | 0.630 mm off, 6.1 ms |
| torus_ring_nurbs | 0.5 | ok, 0.463 mm, 7.5 ms | face 1 empty, 5.000 mm off | ok, 0.441 mm, 7.6 ms | ok, 0.614 mm, 5.8 ms | ok, 0.441 mm, 4.5 ms | ok, 0.675 mm, 4.3 ms |
| washer | 0.0125 | ok, 0.006 mm, 8.9 ms | ok, 0.006 mm, 3.3 ms | ok, 0.007 mm, 8.8 ms | ok, 0.012 mm, 3.1 ms | ok, 0.012 mm, 3.2 ms | 1.000 mm off, 4.6 ms |
| washer | 0.05 | ok, 0.025 mm, 6.4 ms | ok, 0.024 mm, 2.8 ms | ok, 0.027 mm, 8.2 ms | ok, 0.049 mm, 2.7 ms | ok, 0.049 mm, 2.8 ms | 1.000 mm off, 3.4 ms |
| washer | 0.1 | ok, 0.049 mm, 5.8 ms | ok, 0.096 mm, 2.4 ms | ok, 0.093 mm, 7.1 ms | ok, 0.096 mm, 2.7 ms | ok, 0.096 mm, 2.9 ms | 1.000 mm off, 3.2 ms |
| washer | 0.5 | ok, 0.146 mm, 5.5 ms | ok, 0.384 mm, 2.4 ms | ok, 0.415 mm, 7.4 ms | ok, 0.495 mm, 2.6 ms | ok, 0.495 mm, 2.7 ms | ok, 1.030 mm, 3.9 ms |

**opencascade** passes 105 of 108.
- freeform_panel at 0.0125 mm: 0.605 mm off, 1.58 s
- freeform_panel at 0.05 mm: 0.586 mm off, 257 ms
- freeform_panel at 0.1 mm: 0.602 mm off, 164 ms

**ogeom** passes 89 of 108.
- freeform_panel at 0.0125 mm: 5.468 mm off, 403 ms
- cone_point at 0.0125 mm: face 1 lies 0.080 mm off every face of the file it could be; its surface was evaluated wrongly, 0.092 mm off
- cone_point_vertex at 0.0125 mm: face 1 lies 0.080 mm off every face of the file it could be; its surface was evaluated wrongly, 0.092 mm off
- cylinder_nurbs at 0.0125 mm: face 1 empty, 15.000 mm off
- cylinder_nurbs at 0.05 mm: face 1 empty, 15.000 mm off
- cylinder_nurbs at 0.1 mm: face 1 empty, 15.000 mm off
- cylinder_nurbs at 0.5 mm: face 1 empty, 15.000 mm off
- dome at 0.0125 mm: face 1 lies 0.064 mm off every face of the file it could be; its surface was evaluated wrongly, 20.000 mm off
- dome at 0.05 mm: 20.000 mm off, 6.6 ms
- dome at 0.1 mm: 20.000 mm off, 4.7 ms
- dome at 0.5 mm: 20.000 mm off, 3.5 ms
- dome_nurbs at 0.0125 mm: face 1 empty, 20.000 mm off
- dome_nurbs at 0.05 mm: face 1 empty, 20.000 mm off
- dome_nurbs at 0.1 mm: face 1 empty, 20.000 mm off
- dome_nurbs at 0.5 mm: face 1 empty, 20.000 mm off
- torus_ring_nurbs at 0.0125 mm: face 1 empty, 5.000 mm off
- torus_ring_nurbs at 0.05 mm: face 1 empty, 5.000 mm off
- torus_ring_nurbs at 0.1 mm: face 1 empty, 5.000 mm off
- torus_ring_nurbs at 0.5 mm: face 1 empty, 5.000 mm off

**solidcraft** passes 91 of 108.
- freeform_panel at 0.0125 mm: 0.473 mm off, 5.98 s
- freeform_panel at 0.05 mm: 5.306 mm off, 2.69 s
- freeform_panel at 0.1 mm: 5.306 mm off, 2.09 s
- freeform_panel at 0.5 mm: outside the file by 123.5 mm on +Y, 149.165 mm off
- cone_point at 0.0125 mm: 30.000 mm off, 5.1 ms
- cone_point at 0.05 mm: 30.000 mm off, 3.6 ms
- cone_point at 0.1 mm: 30.000 mm off, 3.5 ms
- cone_point at 0.5 mm: 30.000 mm off, 3.7 ms
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
- bowl_01 at 0.05 mm: 53.498 mm off, 4.32 s
- bowl_01 at 0.1 mm: 53.503 mm off, 4.23 s
- bowl_01 at 0.5 mm: 53.546 mm off, 9.0 ms
- freeform_panel at 0.0125 mm: timed out at 60 s
- freeform_panel at 0.05 mm: 12.172 mm off, 25.0 s
- freeform_panel at 0.1 mm: 12.172 mm off, 6.55 s
- freeform_panel at 0.5 mm: 12.172 mm off, 1.50 s
- sled_base at 0.0125 mm: 0.069 mm off, 155 ms
- sled_base at 0.1 mm: timed out at 60 s
- sled_base at 0.5 mm: 4.953 mm off, 46 ms
- cone_point at 0.0125 mm: outside the file by 0.3 mm on +Y, 30.000 mm off
- cone_point at 0.05 mm: outside the file by 0.3 mm on +Y, 30.000 mm off
- cone_point at 0.1 mm: outside the file by 0.2 mm on +Y, 30.000 mm off
- cone_point at 0.5 mm: 30.000 mm off, 2.4 ms
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
- bowl_01 at 0.0125 mm: 10.000 mm off, 272 ms
- bowl_01 at 0.05 mm: 0.218 mm off, 13 ms
- bowl_01 at 0.1 mm: 0.430 mm off, 9.0 ms
- bowl_01 at 0.5 mm: 2.111 mm off, 4.3 ms
- freeform_panel at 0.0125 mm: 12.172 mm off, 11.1 s
- freeform_panel at 0.05 mm: 12.172 mm off, 1.73 s
- freeform_panel at 0.1 mm: 12.172 mm off, 667 ms
- freeform_panel at 0.5 mm: 12.172 mm off, 132 ms
- sled_base at 0.05 mm: 0.818 mm off, 51 ms
- cone_point at 0.0125 mm: outside the file by 0.3 mm on +Y, 30.000 mm off
- cone_point at 0.05 mm: outside the file by 0.3 mm on +Y, 30.000 mm off
- cone_point at 0.1 mm: outside the file by 0.2 mm on +Y, 30.000 mm off
- cone_point at 0.5 mm: 30.000 mm off, 2.2 ms
- cone_point_vertex at 0.0125 mm: outside the file by 0.3 mm on +Y, 30.000 mm off
- cone_point_vertex at 0.05 mm: outside the file by 0.3 mm on +Y, 30.000 mm off
- cone_point_vertex at 0.1 mm: outside the file by 0.2 mm on +Y, 30.000 mm off
- cone_point_vertex at 0.5 mm: short of the vertices by 29.5 mm on +Z, 30.000 mm off

**brepkit** passes 50 of 108.
- bowl_01 at 0.0125 mm: could not read: read_step found no solid (it reads MANIFOLD_SOLID_BREP and BREP_WITH_VOIDS only)
- bowl_01 at 0.05 mm: could not read: read_step found no solid (it reads MANIFOLD_SOLID_BREP and BREP_WITH_VOIDS only)
- bowl_01 at 0.1 mm: could not read: read_step found no solid (it reads MANIFOLD_SOLID_BREP and BREP_WITH_VOIDS only)
- bowl_01 at 0.5 mm: could not read: read_step found no solid (it reads MANIFOLD_SOLID_BREP and BREP_WITH_VOIDS only)
- freeform_panel at 0.0125 mm: 14.374 mm off, 752 ms
- freeform_panel at 0.05 mm: 14.374 mm off, 493 ms
- freeform_panel at 0.1 mm: 14.374 mm off, 458 ms
- freeform_panel at 0.5 mm: nothing of the mesh lies on the file's face #2114; a missing face is a hole in the part, 31.436 mm off
- sled_base at 0.0125 mm: 3.792 mm off, 58 ms
- sled_base at 0.05 mm: 4.149 mm off, 29 ms
- sled_base at 0.1 mm: 4.111 mm off, 24 ms
- sled_base at 0.5 mm: face 357 empty, 5.959 mm off
- cone_point at 0.0125 mm: 14.813 mm off, 1.6 ms
- cone_point at 0.05 mm: 14.813 mm off, 1.3 ms
- cone_point at 0.1 mm: face 1 empty, 30.000 mm off
- cone_point at 0.5 mm: face 1 empty, 30.000 mm off
- cone_point_vertex at 0.0125 mm: could not read: parse error: failed to create wire from edge loop #44: empty wire — at least one
- cone_point_vertex at 0.05 mm: could not read: parse error: failed to create wire from edge loop #44: empty wire — at least one
- cone_point_vertex at 0.1 mm: could not read: parse error: failed to create wire from edge loop #44: empty wire — at least one
- cone_point_vertex at 0.5 mm: could not read: parse error: failed to create wire from edge loop #44: empty wire — at least one
- cylinder at 0.0125 mm: 14.469 mm off, 2.4 ms
- cylinder at 0.05 mm: 14.469 mm off, 1.8 ms
- cylinder at 0.1 mm: 14.469 mm off, 1.9 ms
- cylinder at 0.5 mm: 14.470 mm off, 1.5 ms
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
- dome_nurbs at 0.0125 mm: 3.906 mm off, 62 ms
- dome_nurbs at 0.05 mm: 3.907 mm off, 16 ms
- dome_nurbs at 0.1 mm: 3.913 mm off, 5.3 ms
- dome_nurbs at 0.5 mm: 3.938 mm off, 2.6 ms
- far_cylinder at 0.0125 mm: 14.469 mm off, 2.0 ms
- far_cylinder at 0.05 mm: 14.469 mm off, 1.6 ms
- far_cylinder at 0.1 mm: 14.469 mm off, 1.5 ms
- far_cylinder at 0.5 mm: 14.470 mm off, 1.4 ms
- sheet_disc at 0.0125 mm: 8.906 mm off, 40 ms
- sheet_disc at 0.05 mm: 8.906 mm off, 15 ms
- sheet_disc at 0.1 mm: 8.907 mm off, 8.7 ms
- sheet_disc at 0.5 mm: 8.920 mm off, 4.1 ms
- tiny_pin at 0.0125 mm: outside the file by 0.7 mm on -Z, 1.000 mm off
- tiny_pin at 0.05 mm: face 0 empty, 1000.000 mm off
- tiny_pin at 0.1 mm: face 0 empty, 1000.000 mm off
- tiny_pin at 0.5 mm: face 0 empty, 1000.000 mm off
- torus_ring_nurbs at 0.0125 mm: 0.132 mm off, 68 ms
- torus_ring_nurbs at 0.05 mm: 0.362 mm off, 18 ms
- torus_ring_nurbs at 0.1 mm: 0.630 mm off, 6.1 ms
- washer at 0.0125 mm: 1.000 mm off, 4.6 ms
- washer at 0.05 mm: 1.000 mm off, 3.4 ms
- washer at 0.1 mm: 1.000 mm off, 3.2 ms

## Speed and memory

Every run a reader passes is repeated, up to 5 times or for 10 s, and the median
taken. Time runs from starting the reader to its exit, CPU time is across all its
threads, reading and meshing are the reader's own account of them, and memory is
its peak resident memory.

Against opencascade, on the runs both pass: each figure is the reader's divided by
opencascade's, averaged geometrically, so 0.5× is twice as fast, or half the memory.

| reader | runs both pass | time | CPU time | reading | meshing | peak memory | triangles |
|---|---|---|---|---|---|---|---|
| ogeom | 87 | 0.47× | 0.78× | 0.74× | 0.25× | 0.37× | 1.3× |
| solidcraft | 91 | 1.0× | 2.0× | 1.2× | 1.0× | 0.41× | 1.4× |
| truck | 81 | 0.39× | 0.70× | 0.20× | 0.48× | 0.31× | 0.70× |
| truck-master | 91 | 0.39× | 0.67× | 0.21× | 0.46× | 0.32× | 0.64× |
| brepkit | 50 | 0.26× | 0.38× | 0.030× | 0.48× | 0.27× | 0.87× |

Every run that passes: the time, the reader's own reading and meshing, peak memory,
and triangles.

| part | tolerance mm | opencascade | ogeom | solidcraft | truck | truck-master | brepkit |
|---|---|---|---|---|---|---|---|
| bowl_01 | 0.0125 | 1.84 s (read 3.4 ms, mesh 1.82 s), 145 MB, 178,896 triangles | 73 ms (read 1.0 ms, mesh 71 ms), 15 MB, 42,680 triangles | 632 ms (read 12 ms, mesh 609 ms), 110 MB, 151,644 triangles | – | – | – |
| bowl_01 | 0.05 | 75 ms (read 3.9 ms, mesh 68 ms), 25 MB, 13,200 triangles | 17 ms (read 1.1 ms, mesh 15 ms), 8.0 MB, 11,022 triangles | 192 ms (read 13 ms, mesh 175 ms), 53 MB, 38,948 triangles | – | – | – |
| bowl_01 | 0.1 | 154 ms (read 3.7 ms, mesh 148 ms), 33 MB, 22,838 triangles | 11 ms (read 1.2 ms, mesh 8.1 ms), 6.7 MB, 6,670 triangles | 73 ms (read 13 ms, mesh 58 ms), 26 MB, 14,576 triangles | – | – | – |
| bowl_01 | 0.5 | 22 ms (read 4.1 ms, mesh 16 ms), 18 MB, 3,332 triangles | 5.0 ms (read 1.2 ms, mesh 2.7 ms), 5.6 MB, 1,806 triangles | 23 ms (read 14 ms, mesh 7.7 ms), 6.3 MB, 1,792 triangles | – | – | – |
| door_handle_guide | 0.0125 | 48 ms (read 30 ms, mesh 16 ms), 20 MB, 1,948 triangles | 25 ms (read 19 ms, mesh 4.5 ms), 9.2 MB, 7,588 triangles | 54 ms (read 30 ms, mesh 21 ms), 12 MB, 10,116 triangles | 15 ms (read 9.5 ms, mesh 3.8 ms), 6.3 MB, 1,152 triangles | 15 ms (read 9.5 ms, mesh 4.3 ms), 6.5 MB, 1,152 triangles | 8.0 ms (read 1.1 ms, mesh 5.8 ms), 4.8 MB, 1,202 triangles |
| door_handle_guide | 0.05 | 44 ms (read 29 ms, mesh 12 ms), 19 MB, 960 triangles | 20 ms (read 17 ms, mesh 2.5 ms), 7.4 MB, 2,524 triangles | 40 ms (read 30 ms, mesh 9.2 ms), 7.4 MB, 2,640 triangles | 15 ms (read 9.4 ms, mesh 4.6 ms), 6.1 MB, 696 triangles | 15 ms (read 9.4 ms, mesh 4.0 ms), 6.3 MB, 696 triangles | 7.1 ms (read 1.1 ms, mesh 4.9 ms), 4.6 MB, 742 triangles |
| door_handle_guide | 0.1 | 45 ms (read 29 ms, mesh 12 ms), 19 MB, 956 triangles | 20 ms (read 17 ms, mesh 1.9 ms), 7.1 MB, 1,052 triangles | 39 ms (read 30 ms, mesh 7.1 ms), 6.1 MB, 1,036 triangles | 15 ms (read 9.9 ms, mesh 4.3 ms), 6.0 MB, 568 triangles | 15 ms (read 9.5 ms, mesh 4.0 ms), 6.3 MB, 568 triangles | 7.6 ms (read 1.2 ms, mesh 5.1 ms), 4.7 MB, 734 triangles |
| door_handle_guide | 0.5 | 44 ms (read 29 ms, mesh 11 ms), 19 MB, 956 triangles | 21 ms (read 18 ms, mesh 2.1 ms), 7.1 MB, 1,052 triangles | 36 ms (read 29 ms, mesh 6.5 ms), 6.1 MB, 488 triangles | 14 ms (read 9.3 ms, mesh 3.8 ms), 6.0 MB, 396 triangles | 14 ms (read 9.3 ms, mesh 3.6 ms), 6.4 MB, 396 triangles | 7.9 ms (read 1.1 ms, mesh 5.0 ms), 4.6 MB, 734 triangles |
| freeform_panel | 0.0125 | – | – | – | – | – | – |
| freeform_panel | 0.05 | – | 215 ms (read 87 ms, mesh 126 ms), 15 MB, 16,415 triangles | – | – | – | – |
| freeform_panel | 0.1 | – | 144 ms (read 85 ms, mesh 58 ms), 13 MB, 9,323 triangles | – | – | – | – |
| freeform_panel | 0.5 | 59 ms (read 38 ms, mesh 20 ms), 21 MB, 2,566 triangles | 112 ms (read 91 ms, mesh 20 ms), 12 MB, 3,200 triangles | – | – | – | – |
| hinge_jig | 0.0125 | 17 ms (read 11 ms, mesh 3.3 ms), 17 MB, 74 triangles | 6.9 ms (read 4.6 ms, mesh 1.6 ms), 5.3 MB, 74 triangles | 16 ms (read 8.0 ms, mesh 5.9 ms), 4.6 MB, 74 triangles | 8.3 ms (read 3.7 ms, mesh 3.6 ms), 5.1 MB, 74 triangles | 9.4 ms (read 3.8 ms, mesh 4.2 ms), 5.3 MB, 74 triangles | 4.6 ms (read 0.5 ms, mesh 3.1 ms), 3.7 MB, 74 triangles |
| hinge_jig | 0.05 | 17 ms (read 12 ms, mesh 3.3 ms), 17 MB, 74 triangles | 6.1 ms (read 4.1 ms, mesh 1.2 ms), 5.4 MB, 74 triangles | 13 ms (read 6.9 ms, mesh 5.1 ms), 4.6 MB, 74 triangles | 7.9 ms (read 3.6 ms, mesh 3.3 ms), 5.0 MB, 74 triangles | 8.2 ms (read 3.6 ms, mesh 3.5 ms), 5.4 MB, 74 triangles | 3.8 ms (read 0.4 ms, mesh 2.4 ms), 3.7 MB, 74 triangles |
| hinge_jig | 0.1 | 15 ms (read 10 ms, mesh 3.1 ms), 17 MB, 74 triangles | 6.0 ms (read 4.0 ms, mesh 1.2 ms), 5.6 MB, 74 triangles | 13 ms (read 7.2 ms, mesh 5.0 ms), 4.6 MB, 74 triangles | 7.5 ms (read 3.4 ms, mesh 3.0 ms), 5.0 MB, 74 triangles | 7.7 ms (read 3.4 ms, mesh 3.2 ms), 5.4 MB, 74 triangles | 3.6 ms (read 0.4 ms, mesh 2.3 ms), 3.7 MB, 74 triangles |
| hinge_jig | 0.5 | 15 ms (read 10 ms, mesh 3.0 ms), 17 MB, 74 triangles | 8.3 ms (read 4.9 ms, mesh 2.1 ms), 5.4 MB, 74 triangles | 15 ms (read 8.6 ms, mesh 5.4 ms), 4.6 MB, 74 triangles | 7.1 ms (read 3.4 ms, mesh 2.9 ms), 5.1 MB, 74 triangles | 7.4 ms (read 3.2 ms, mesh 3.2 ms), 5.3 MB, 74 triangles | 3.4 ms (read 0.4 ms, mesh 2.1 ms), 3.8 MB, 74 triangles |
| lego_3_pins | 0.0125 | 215 ms (read 67 ms, mesh 148 ms), 26 MB, 27,592 triangles | 78 ms (read 65 ms, mesh 11 ms), 16 MB, 34,284 triangles | 214 ms (read 110 ms, mesh 93 ms), 25 MB, 49,134 triangles | 44 ms (read 22 ms, mesh 19 ms), 14 MB, 13,348 triangles | 44 ms (read 21 ms, mesh 20 ms), 15 MB, 13,348 triangles | 46 ms (read 2.4 ms, mesh 42 ms), 7.7 MB, 13,930 triangles |
| lego_3_pins | 0.05 | 147 ms (read 72 ms, mesh 71 ms), 24 MB, 11,786 triangles | 68 ms (read 61 ms, mesh 4.8 ms), 12 MB, 14,452 triangles | 151 ms (read 108 ms, mesh 41 ms), 12 MB, 13,368 triangles | 35 ms (read 21 ms, mesh 12 ms), 11 MB, 6,814 triangles | 36 ms (read 21 ms, mesh 13 ms), 12 MB, 6,814 triangles | 27 ms (read 2.3 ms, mesh 24 ms), 6.0 MB, 7,126 triangles |
| lego_3_pins | 0.1 | 120 ms (read 65 ms, mesh 53 ms), 24 MB, 8,570 triangles | 69 ms (read 62 ms, mesh 4.7 ms), 12 MB, 12,540 triangles | 144 ms (read 105 ms, mesh 33 ms), 10 MB, 10,056 triangles | 35 ms (read 21 ms, mesh 13 ms), 10 MB, 4,962 triangles | 35 ms (read 21 ms, mesh 11 ms), 10 MB, 4,962 triangles | 29 ms (read 2.4 ms, mesh 24 ms), 5.4 MB, 5,344 triangles |
| lego_3_pins | 0.5 | 107 ms (read 64 ms, mesh 41 ms), 23 MB, 6,680 triangles | 71 ms (read 66 ms, mesh 3.6 ms), 12 MB, 6,780 triangles | 137 ms (read 110 ms, mesh 26 ms), 9.1 MB, 3,552 triangles | 34 ms (read 21 ms, mesh 11 ms), 8.9 MB, 2,374 triangles | 33 ms (read 21 ms, mesh 10 ms), 9.4 MB, 2,374 triangles | 26 ms (read 2.5 ms, mesh 22 ms), 5.3 MB, 4,696 triangles |
| lego_4x2_01 | 0.0125 | 69 ms (read 21 ms, mesh 46 ms), 22 MB, 9,064 triangles | 33 ms (read 27 ms, mesh 4.8 ms), 9.6 MB, 10,336 triangles | 66 ms (read 36 ms, mesh 29 ms), 13 MB, 14,950 triangles | 14 ms (read 6.6 ms, mesh 5.8 ms), 8.0 MB, 4,356 triangles | 16 ms (read 6.5 ms, mesh 7.4 ms), 8.4 MB, 4,356 triangles | 15 ms (read 0.8 ms, mesh 13 ms), 6.9 MB, 4,550 triangles |
| lego_4x2_01 | 0.05 | 41 ms (read 20 ms, mesh 19 ms), 19 MB, 3,862 triangles | 30 ms (read 26 ms, mesh 2.8 ms), 8.3 MB, 4,528 triangles | 47 ms (read 33 ms, mesh 12 ms), 7.8 MB, 4,084 triangles | 13 ms (read 7.0 ms, mesh 4.8 ms), 6.4 MB, 2,226 triangles | 13 ms (read 6.6 ms, mesh 4.9 ms), 6.9 MB, 2,226 triangles | 11 ms (read 0.8 ms, mesh 8.5 ms), 5.4 MB, 2,330 triangles |
| lego_4x2_01 | 0.1 | 35 ms (read 20 ms, mesh 14 ms), 19 MB, 2,790 triangles | 32 ms (read 28 ms, mesh 3.0 ms), 7.9 MB, 4,080 triangles | 44 ms (read 33 ms, mesh 11 ms), 6.9 MB, 3,252 triangles | 14 ms (read 7.7 ms, mesh 4.4 ms), 6.2 MB, 1,622 triangles | 13 ms (read 7.0 ms, mesh 4.9 ms), 6.6 MB, 1,622 triangles | 11 ms (read 1.0 ms, mesh 8.9 ms), 4.8 MB, 1,736 triangles |
| lego_4x2_01 | 0.5 | 43 ms (read 24 ms, mesh 16 ms), 19 MB, 2,160 triangles | 31 ms (read 28 ms, mesh 2.4 ms), 8.3 MB, 2,160 triangles | 43 ms (read 34 ms, mesh 6.9 ms), 6.1 MB, 1,156 triangles | 14 ms (read 8.0 ms, mesh 4.4 ms), 5.8 MB, 778 triangles | 13 ms (read 7.6 ms, mesh 4.3 ms), 6.1 MB, 778 triangles | 9.9 ms (read 0.9 ms, mesh 7.7 ms), 4.9 MB, 1,520 triangles |
| rail_guide | 0.0125 | 78 ms (read 41 ms, mesh 35 ms), 23 MB, 6,484 triangles | 41 ms (read 36 ms, mesh 4.1 ms), 11 MB, 8,744 triangles | 80 ms (read 48 ms, mesh 28 ms), 13 MB, 11,808 triangles | 21 ms (read 14 ms, mesh 5.3 ms), 7.8 MB, 2,596 triangles | 21 ms (read 14 ms, mesh 5.5 ms), 8.4 MB, 2,596 triangles | 12 ms (read 1.7 ms, mesh 8.9 ms), 6.0 MB, 2,948 triangles |
| rail_guide | 0.05 | 68 ms (read 44 ms, mesh 23 ms), 22 MB, 2,884 triangles | 42 ms (read 38 ms, mesh 2.9 ms), 8.7 MB, 3,216 triangles | 63 ms (read 48 ms, mesh 12 ms), 8.3 MB, 4,076 triangles | 18 ms (read 13 ms, mesh 4.1 ms), 7.2 MB, 1,404 triangles | 19 ms (read 13 ms, mesh 4.5 ms), 7.6 MB, 1,404 triangles | 10 ms (read 1.8 ms, mesh 7.1 ms), 5.1 MB, 1,604 triangles |
| rail_guide | 0.1 | 67 ms (read 44 ms, mesh 20 ms), 21 MB, 2,412 triangles | 44 ms (read 40 ms, mesh 2.4 ms), 8.5 MB, 2,448 triangles | 58 ms (read 46 ms, mesh 9.8 ms), 7.5 MB, 2,648 triangles | 19 ms (read 13 ms, mesh 3.9 ms), 7.1 MB, 1,100 triangles | 20 ms (read 14 ms, mesh 3.9 ms), 7.5 MB, 1,100 triangles | 9.7 ms (read 1.7 ms, mesh 6.5 ms), 5.1 MB, 1,492 triangles |
| rail_guide | 0.5 | 62 ms (read 40 ms, mesh 19 ms), 21 MB, 2,244 triangles | 38 ms (read 35 ms, mesh 2.2 ms), 8.9 MB, 2,208 triangles | 55 ms (read 47 ms, mesh 6.9 ms), 6.9 MB, 804 triangles | 17 ms (read 12 ms, mesh 3.8 ms), 7.0 MB, 652 triangles | 17 ms (read 12 ms, mesh 4.0 ms), 7.2 MB, 652 triangles | 8.7 ms (read 1.5 ms, mesh 6.1 ms), 5.1 MB, 1,492 triangles |
| sled_base | 0.0125 | 344 ms (read 142 ms, mesh 199 ms), 35 MB, 33,822 triangles | 112 ms (read 95 ms, mesh 14 ms), 26 MB, 54,166 triangles | 354 ms (read 224 ms, mesh 131 ms), 47 MB, 67,572 triangles | – | 74 ms (read 36 ms, mesh 33 ms), 26 MB, 30,858 triangles | – |
| sled_base | 0.05 | 202 ms (read 120 ms, mesh 77 ms), 32 MB, 12,750 triangles | 99 ms (read 91 ms, mesh 5.9 ms), 21 MB, 17,434 triangles | 264 ms (read 218 ms, mesh 40 ms), 19 MB, 20,904 triangles | 76 ms (read 38 ms, mesh 34 ms), 22 MB, 22,127 triangles | – | – |
| sled_base | 0.1 | 190 ms (read 118 ms, mesh 64 ms), 32 MB, 9,320 triangles | 101 ms (read 93 ms, mesh 5.5 ms), 20 MB, 15,386 triangles | 241 ms (read 208 ms, mesh 29 ms), 17 MB, 16,112 triangles | – | 42 ms (read 35 ms, mesh 6.6 ms), 14 MB, 5,210 triangles | – |
| sled_base | 0.5 | 173 ms (read 112 ms, mesh 57 ms), 32 MB, 7,968 triangles | 98 ms (read 91 ms, mesh 5.1 ms), 20 MB, 13,610 triangles | 237 ms (read 220 ms, mesh 14 ms), 13 MB, 3,206 triangles | – | 45 ms (read 37 ms, mesh 6.1 ms), 12 MB, 2,194 triangles | – |
| cone_point | 0.0125 | 12 ms (read 2.0 ms, mesh 7.9 ms), 18 MB, 1,563 triangles | – | – | – | – | – |
| cone_point | 0.05 | 5.5 ms (read 2.0 ms, mesh 2.1 ms), 16 MB, 473 triangles | 3.4 ms (read 1.5 ms, mesh 1.2 ms), 5.3 MB, 653 triangles | – | – | – | – |
| cone_point | 0.1 | 4.8 ms (read 1.9 ms, mesh 1.3 ms), 16 MB, 293 triangles | 2.9 ms (read 1.5 ms, mesh 0.7 ms), 5.3 MB, 361 triangles | – | – | – | – |
| cone_point | 0.5 | 4.9 ms (read 2.3 ms, mesh 0.7 ms), 16 MB, 129 triangles | 2.6 ms (read 1.5 ms, mesh 0.4 ms), 5.4 MB, 131 triangles | – | – | – | – |
| cone_point_vertex | 0.0125 | 12 ms (read 2.1 ms, mesh 8.0 ms), 19 MB, 1,563 triangles | – | – | – | – | – |
| cone_point_vertex | 0.05 | 5.9 ms (read 2.1 ms, mesh 2.1 ms), 16 MB, 473 triangles | 3.3 ms (read 1.5 ms, mesh 1.2 ms), 5.2 MB, 653 triangles | – | – | – | – |
| cone_point_vertex | 0.1 | 4.9 ms (read 2.0 ms, mesh 1.2 ms), 16 MB, 293 triangles | 3.1 ms (read 1.6 ms, mesh 0.7 ms), 5.3 MB, 361 triangles | – | – | – | – |
| cone_point_vertex | 0.5 | 4.1 ms (read 1.9 ms, mesh 0.7 ms), 16 MB, 129 triangles | 2.7 ms (read 1.5 ms, mesh 0.4 ms), 5.3 MB, 131 triangles | – | – | – | – |
| cylinder | 0.0125 | 5.8 ms (read 2.1 ms, mesh 2.0 ms), 16 MB, 352 triangles | 3.1 ms (read 1.2 ms, mesh 1.2 ms), 5.2 MB, 882 triangles | 8.2 ms (read 2.3 ms, mesh 4.1 ms), 6.0 MB, 1,262 triangles | 2.5 ms (read 0.4 ms, mesh 1.2 ms), 4.4 MB, 248 triangles | 2.8 ms (read 0.4 ms, mesh 1.2 ms), 4.7 MB, 248 triangles | – |
| cylinder | 0.05 | 5.0 ms (read 2.3 ms, mesh 1.1 ms), 16 MB, 176 triangles | 2.4 ms (read 1.2 ms, mesh 0.5 ms), 5.1 MB, 310 triangles | 5.1 ms (read 2.4 ms, mesh 1.8 ms), 5.0 MB, 374 triangles | 2.2 ms (read 0.4 ms, mesh 1.0 ms), 4.3 MB, 124 triangles | 2.4 ms (read 0.4 ms, mesh 1.1 ms), 4.8 MB, 124 triangles | – |
| cylinder | 0.1 | 4.4 ms (read 2.1 ms, mesh 0.8 ms), 16 MB, 124 triangles | 2.4 ms (read 1.2 ms, mesh 0.5 ms), 5.0 MB, 310 triangles | 5.0 ms (read 2.5 ms, mesh 1.7 ms), 4.6 MB, 310 triangles | 2.7 ms (read 0.3 ms, mesh 1.1 ms), 4.3 MB, 88 triangles | 2.8 ms (read 0.4 ms, mesh 1.0 ms), 4.7 MB, 88 triangles | – |
| cylinder | 0.5 | 4.9 ms (read 2.3 ms, mesh 0.8 ms), 16 MB, 100 triangles | 2.6 ms (read 1.4 ms, mesh 0.4 ms), 5.1 MB, 90 triangles | 4.3 ms (read 2.4 ms, mesh 1.0 ms), 4.3 MB, 90 triangles | 2.3 ms (read 0.4 ms, mesh 1.1 ms), 4.3 MB, 36 triangles | 2.3 ms (read 0.4 ms, mesh 1.0 ms), 4.7 MB, 36 triangles | – |
| cylinder_inch | 0.0125 | 6.1 ms (read 2.2 ms, mesh 2.1 ms), 16 MB, 352 triangles | 3.3 ms (read 1.3 ms, mesh 1.3 ms), 5.3 MB, 882 triangles | 7.1 ms (read 2.3 ms, mesh 3.9 ms), 6.0 MB, 1,262 triangles | 2.6 ms (read 0.4 ms, mesh 1.4 ms), 4.4 MB, 248 triangles | 2.8 ms (read 0.4 ms, mesh 1.4 ms), 4.9 MB, 248 triangles | – |
| cylinder_inch | 0.05 | 4.9 ms (read 2.2 ms, mesh 1.1 ms), 16 MB, 176 triangles | 2.4 ms (read 1.2 ms, mesh 0.5 ms), 5.1 MB, 310 triangles | 5.5 ms (read 2.7 ms, mesh 1.7 ms), 4.9 MB, 374 triangles | 2.5 ms (read 0.4 ms, mesh 1.1 ms), 4.4 MB, 124 triangles | 3.0 ms (read 0.4 ms, mesh 1.1 ms), 4.7 MB, 124 triangles | – |
| cylinder_inch | 0.1 | 4.4 ms (read 2.1 ms, mesh 0.8 ms), 16 MB, 124 triangles | 2.6 ms (read 1.2 ms, mesh 0.6 ms), 5.0 MB, 310 triangles | 5.3 ms (read 2.6 ms, mesh 1.7 ms), 4.7 MB, 310 triangles | 2.5 ms (read 0.4 ms, mesh 1.1 ms), 4.6 MB, 88 triangles | 2.5 ms (read 0.4 ms, mesh 1.0 ms), 4.7 MB, 88 triangles | – |
| cylinder_inch | 0.5 | 4.3 ms (read 2.1 ms, mesh 0.7 ms), 16 MB, 100 triangles | 2.2 ms (read 1.2 ms, mesh 0.3 ms), 5.1 MB, 90 triangles | 4.2 ms (read 2.3 ms, mesh 1.0 ms), 4.2 MB, 90 triangles | 2.5 ms (read 0.4 ms, mesh 1.1 ms), 4.4 MB, 36 triangles | 2.5 ms (read 0.4 ms, mesh 1.0 ms), 4.8 MB, 36 triangles | – |
| cylinder_nurbs | 0.0125 | 9.4 ms (read 3.5 ms, mesh 4.3 ms), 18 MB, 372 triangles | – | 17 ms (read 2.9 ms, mesh 12 ms), 9.6 MB, 3,040 triangles | 59 ms (read 0.6 ms, mesh 56 ms), 13 MB, 24,508 triangles | 31 ms (read 0.6 ms, mesh 28 ms), 9.8 MB, 15,008 triangles | 4.6 ms (read 0.1 ms, mesh 3.6 ms), 4.2 MB, 322 triangles |
| cylinder_nurbs | 0.05 | 7.1 ms (read 3.6 ms, mesh 1.7 ms), 17 MB, 184 triangles | – | 7.1 ms (read 2.4 ms, mesh 3.8 ms), 5.6 MB, 846 triangles | 16 ms (read 0.6 ms, mesh 13 ms), 7.0 MB, 6,104 triangles | 8.4 ms (read 0.6 ms, mesh 6.3 ms), 6.2 MB, 3,098 triangles | 2.8 ms (read 0.1 ms, mesh 2.0 ms), 4.1 MB, 178 triangles |
| cylinder_nurbs | 0.1 | 7.1 ms (read 3.9 ms, mesh 1.2 ms), 17 MB, 132 triangles | – | 6.4 ms (read 2.6 ms, mesh 2.3 ms), 4.5 MB, 310 triangles | 7.1 ms (read 0.6 ms, mesh 5.4 ms), 5.4 MB, 2,046 triangles | 6.0 ms (read 0.6 ms, mesh 4.2 ms), 5.9 MB, 2,046 triangles | 2.7 ms (read 0.1 ms, mesh 2.0 ms), 4.0 MB, 178 triangles |
| cylinder_nurbs | 0.5 | 5.8 ms (read 3.3 ms, mesh 1.0 ms), 17 MB, 120 triangles | – | 5.0 ms (read 2.4 ms, mesh 1.5 ms), 4.0 MB, 90 triangles | 4.0 ms (read 0.5 ms, mesh 2.3 ms), 4.7 MB, 510 triangles | 4.1 ms (read 0.6 ms, mesh 1.9 ms), 4.9 MB, 510 triangles | 2.3 ms (read 0.1 ms, mesh 1.6 ms), 4.0 MB, 178 triangles |
| cylinder_placed | 0.0125 | 5.6 ms (read 2.0 ms, mesh 1.9 ms), 16 MB, 352 triangles | 2.9 ms (read 1.1 ms, mesh 1.2 ms), 5.3 MB, 882 triangles | 7.2 ms (read 2.3 ms, mesh 3.7 ms), 6.0 MB, 1,262 triangles | – | 3.0 ms (read 0.5 ms, mesh 1.2 ms), 4.8 MB, 248 triangles | – |
| cylinder_placed | 0.05 | 4.7 ms (read 2.0 ms, mesh 1.1 ms), 16 MB, 176 triangles | 2.2 ms (read 1.1 ms, mesh 0.5 ms), 5.1 MB, 310 triangles | 4.6 ms (read 2.2 ms, mesh 1.7 ms), 5.0 MB, 374 triangles | – | 2.6 ms (read 0.4 ms, mesh 1.0 ms), 4.7 MB, 124 triangles | – |
| cylinder_placed | 0.1 | 4.2 ms (read 2.0 ms, mesh 0.8 ms), 16 MB, 124 triangles | 2.4 ms (read 1.2 ms, mesh 0.5 ms), 5.1 MB, 310 triangles | 4.5 ms (read 2.2 ms, mesh 1.4 ms), 4.6 MB, 310 triangles | – | 2.2 ms (read 0.5 ms, mesh 0.9 ms), 4.7 MB, 88 triangles | – |
| cylinder_placed | 0.5 | 4.3 ms (read 2.1 ms, mesh 0.7 ms), 16 MB, 100 triangles | 2.6 ms (read 1.3 ms, mesh 0.4 ms), 5.1 MB, 90 triangles | 4.6 ms (read 2.3 ms, mesh 1.0 ms), 4.1 MB, 90 triangles | – | 2.5 ms (read 0.4 ms, mesh 1.1 ms), 4.7 MB, 36 triangles | – |
| cylinder_seam | 0.0125 | 5.2 ms (read 1.8 ms, mesh 1.9 ms), 16 MB, 352 triangles | 3.1 ms (read 1.2 ms, mesh 1.2 ms), 5.2 MB, 882 triangles | 7.8 ms (read 2.4 ms, mesh 4.1 ms), 6.0 MB, 1,262 triangles | 2.7 ms (read 0.4 ms, mesh 1.3 ms), 4.4 MB, 248 triangles | 2.6 ms (read 0.4 ms, mesh 1.3 ms), 4.7 MB, 248 triangles | 1.8 ms (read 0.1 ms, mesh 1.0 ms), 4.1 MB, 248 triangles |
| cylinder_seam | 0.05 | 4.6 ms (read 2.1 ms, mesh 1.1 ms), 16 MB, 176 triangles | 2.7 ms (read 1.2 ms, mesh 0.6 ms), 5.1 MB, 310 triangles | 5.3 ms (read 2.6 ms, mesh 1.9 ms), 4.8 MB, 374 triangles | 2.5 ms (read 0.4 ms, mesh 1.1 ms), 4.5 MB, 124 triangles | 2.7 ms (read 0.4 ms, mesh 1.1 ms), 4.7 MB, 124 triangles | 1.4 ms (read 0.1 ms, mesh 0.7 ms), 4.0 MB, 124 triangles |
| cylinder_seam | 0.1 | 4.2 ms (read 1.9 ms, mesh 0.8 ms), 16 MB, 124 triangles | 2.5 ms (read 1.2 ms, mesh 0.6 ms), 5.1 MB, 310 triangles | 5.5 ms (read 2.5 ms, mesh 1.8 ms), 4.6 MB, 310 triangles | 2.6 ms (read 0.4 ms, mesh 1.1 ms), 4.4 MB, 88 triangles | 2.8 ms (read 0.4 ms, mesh 1.1 ms), 4.7 MB, 88 triangles | 1.4 ms (read 0.1 ms, mesh 0.7 ms), 4.0 MB, 88 triangles |
| cylinder_seam | 0.5 | 4.3 ms (read 2.0 ms, mesh 0.7 ms), 16 MB, 100 triangles | 2.2 ms (read 1.2 ms, mesh 0.3 ms), 5.0 MB, 90 triangles | 5.1 ms (read 2.7 ms, mesh 1.4 ms), 4.1 MB, 90 triangles | 2.7 ms (read 0.5 ms, mesh 1.2 ms), 4.4 MB, 36 triangles | 2.7 ms (read 0.4 ms, mesh 1.1 ms), 4.7 MB, 36 triangles | 1.4 ms (read 0.1 ms, mesh 0.6 ms), 4.1 MB, 68 triangles |
| cylinder_twice | 0.0125 | 6.1 ms (read 2.2 ms, mesh 2.1 ms), 16 MB, 704 triangles | 4.5 ms (read 1.2 ms, mesh 2.5 ms), 5.4 MB, 1,764 triangles | 14 ms (read 3.8 ms, mesh 7.9 ms), 6.6 MB, 2,524 triangles | – | 3.4 ms (read 0.5 ms, mesh 1.7 ms), 4.9 MB, 496 triangles | – |
| cylinder_twice | 0.05 | 4.8 ms (read 2.2 ms, mesh 1.1 ms), 16 MB, 352 triangles | 3.1 ms (read 1.2 ms, mesh 1.2 ms), 5.1 MB, 620 triangles | 11 ms (read 4.4 ms, mesh 5.2 ms), 6.1 MB, 1,432 triangles | – | 3.3 ms (read 0.5 ms, mesh 1.6 ms), 4.7 MB, 248 triangles | – |
| cylinder_twice | 0.1 | 5.2 ms (read 2.6 ms, mesh 0.9 ms), 16 MB, 248 triangles | 3.5 ms (read 1.3 ms, mesh 1.2 ms), 5.0 MB, 620 triangles | 8.8 ms (read 4.3 ms, mesh 3.3 ms), 4.8 MB, 620 triangles | – | 3.9 ms (read 0.6 ms, mesh 1.6 ms), 4.7 MB, 176 triangles | – |
| cylinder_twice | 0.5 | 4.9 ms (read 2.5 ms, mesh 0.8 ms), 16 MB, 200 triangles | 2.6 ms (read 1.2 ms, mesh 0.6 ms), 5.1 MB, 180 triangles | 6.9 ms (read 3.7 ms, mesh 2.0 ms), 4.2 MB, 180 triangles | – | 2.9 ms (read 0.5 ms, mesh 1.2 ms), 4.8 MB, 72 triangles | – |
| dome | 0.0125 | 48 ms (read 2.1 ms, mesh 44 ms), 29 MB, 8,157 triangles | – | 58 ms (read 3.5 ms, mesh 53 ms), 28 MB, 14,571 triangles | 8.6 ms (read 0.3 ms, mesh 6.5 ms), 6.3 MB, 4,111 triangles | 8.5 ms (read 0.3 ms, mesh 6.6 ms), 6.5 MB, 4,111 triangles | 53 ms (read 0.1 ms, mesh 51 ms), 22 MB, 31,344 triangles |
| dome | 0.05 | 11 ms (read 2.2 ms, mesh 7.1 ms), 18 MB, 2,061 triangles | – | 18 ms (read 3.4 ms, mesh 13 ms), 10 MB, 3,639 triangles | 4.2 ms (read 0.3 ms, mesh 2.2 ms), 5.0 MB, 1,087 triangles | 3.7 ms (read 0.3 ms, mesh 2.1 ms), 5.4 MB, 1,087 triangles | 12 ms (read 0.1 ms, mesh 11 ms), 8.3 MB, 7,860 triangles |
| dome | 0.1 | 6.9 ms (read 1.9 ms, mesh 3.5 ms), 17 MB, 1,091 triangles | – | 15 ms (read 3.4 ms, mesh 9.4 ms), 9.5 MB, 2,485 triangles | 3.0 ms (read 0.3 ms, mesh 1.7 ms), 4.6 MB, 511 triangles | 2.9 ms (read 0.3 ms, mesh 1.6 ms), 4.9 MB, 511 triangles | 6.8 ms (read 0.1 ms, mesh 5.8 ms), 5.9 MB, 3,948 triangles |
| dome | 0.5 | 4.5 ms (read 2.0 ms, mesh 1.0 ms), 16 MB, 269 triangles | – | 5.9 ms (read 3.2 ms, mesh 1.8 ms), 5.0 MB, 403 triangles | 2.4 ms (read 0.3 ms, mesh 1.0 ms), 4.5 MB, 119 triangles | 2.4 ms (read 0.3 ms, mesh 1.0 ms), 4.8 MB, 119 triangles | 2.1 ms (read 0.0 ms, mesh 1.4 ms), 4.3 MB, 744 triangles |
| dome_nurbs | 0.0125 | 144 ms (read 3.5 ms, mesh 138 ms), 34 MB, 20,385 triangles | – | 62 ms (read 3.3 ms, mesh 55 ms), 28 MB, 13,623 triangles | 66 ms (read 0.5 ms, mesh 61 ms), 13 MB, 26,987 triangles | 33 ms (read 0.6 ms, mesh 31 ms), 10.0 MB, 16,827 triangles | – |
| dome_nurbs | 0.05 | 29 ms (read 3.0 ms, mesh 24 ms), 20 MB, 4,925 triangles | – | 17 ms (read 3.0 ms, mesh 13 ms), 10 MB, 3,385 triangles | 15 ms (read 0.5 ms, mesh 13 ms), 7.1 MB, 6,709 triangles | 8.5 ms (read 0.5 ms, mesh 6.6 ms), 6.5 MB, 4,189 triangles | – |
| dome_nurbs | 0.1 | 16 ms (read 2.9 ms, mesh 11 ms), 18 MB, 2,467 triangles | – | 12 ms (read 3.0 ms, mesh 8.2 ms), 7.5 MB, 2,385 triangles | 11 ms (read 0.5 ms, mesh 9.9 ms), 6.5 MB, 4,295 triangles | 7.5 ms (read 0.5 ms, mesh 5.9 ms), 6.4 MB, 3,388 triangles | – |
| dome_nurbs | 0.5 | 6.7 ms (read 2.7 ms, mesh 2.5 ms), 17 MB, 565 triangles | – | 6.5 ms (read 3.3 ms, mesh 2.2 ms), 4.8 MB, 409 triangles | 3.5 ms (read 0.5 ms, mesh 2.0 ms), 4.7 MB, 427 triangles | 3.1 ms (read 0.5 ms, mesh 1.5 ms), 4.9 MB, 277 triangles | – |
| far_cylinder | 0.0125 | 5.9 ms (read 1.9 ms, mesh 2.3 ms), 16 MB, 352 triangles | 3.0 ms (read 1.2 ms, mesh 1.2 ms), 5.2 MB, 882 triangles | 7.2 ms (read 2.3 ms, mesh 3.8 ms), 6.0 MB, 1,262 triangles | 2.3 ms (read 0.3 ms, mesh 1.1 ms), 4.5 MB, 248 triangles | 2.5 ms (read 0.3 ms, mesh 1.1 ms), 4.7 MB, 248 triangles | – |
| far_cylinder | 0.05 | 4.5 ms (read 2.1 ms, mesh 1.0 ms), 16 MB, 176 triangles | 2.4 ms (read 1.1 ms, mesh 0.5 ms), 5.0 MB, 310 triangles | 6.7 ms (read 2.4 ms, mesh 3.1 ms), 5.8 MB, 1,058 triangles | 2.2 ms (read 0.3 ms, mesh 1.0 ms), 4.5 MB, 124 triangles | 2.2 ms (read 0.3 ms, mesh 1.0 ms), 4.6 MB, 124 triangles | – |
| far_cylinder | 0.1 | 4.2 ms (read 2.0 ms, mesh 0.7 ms), 16 MB, 124 triangles | 2.3 ms (read 1.1 ms, mesh 0.6 ms), 5.1 MB, 310 triangles | 5.2 ms (read 2.3 ms, mesh 1.4 ms), 4.6 MB, 310 triangles | 2.2 ms (read 0.4 ms, mesh 1.0 ms), 4.3 MB, 88 triangles | 2.3 ms (read 0.3 ms, mesh 1.0 ms), 4.7 MB, 88 triangles | – |
| far_cylinder | 0.5 | 4.1 ms (read 2.1 ms, mesh 0.7 ms), 16 MB, 100 triangles | 2.1 ms (read 1.2 ms, mesh 0.3 ms), 5.1 MB, 90 triangles | 3.7 ms (read 2.0 ms, mesh 0.9 ms), 4.2 MB, 90 triangles | 2.1 ms (read 0.3 ms, mesh 0.9 ms), 4.3 MB, 36 triangles | 2.3 ms (read 0.4 ms, mesh 0.9 ms), 4.7 MB, 36 triangles | – |
| grooved_shaft | 0.0125 | 759 ms (read 165 ms, mesh 583 ms), 51 MB, 132,852 triangles | 538 ms (read 489 ms, mesh 45 ms), 59 MB, 190,878 triangles | 692 ms (read 390 ms, mesh 284 ms), 95 MB, 168,380 triangles | 109 ms (read 67 ms, mesh 33 ms), 55 MB, 94,248 triangles | 119 ms (read 72 ms, mesh 39 ms), 56 MB, 94,248 triangles | 160 ms (read 8.1 ms, mesh 147 ms), 41 MB, 94,748 triangles |
| grooved_shaft | 0.05 | 441 ms (read 167 ms, mesh 276 ms), 40 MB, 67,176 triangles | 725 ms (read 638 ms, mesh 58 ms), 59 MB, 94,686 triangles | 520 ms (read 380 ms, mesh 130 ms), 56 MB, 76,188 triangles | 95 ms (read 68 ms, mesh 20 ms), 41 MB, 47,624 triangles | 101 ms (read 71 ms, mesh 22 ms), 42 MB, 47,624 triangles | 95 ms (read 8.6 ms, mesh 84 ms), 23 MB, 48,124 triangles |
| grooved_shaft | 0.1 | 482 ms (read 220 ms, mesh 273 ms), 39 MB, 47,624 triangles | 877 ms (read 775 ms, mesh 84 ms), 59 MB, 94,686 triangles | 544 ms (read 403 ms, mesh 104 ms), 42 MB, 48,124 triangles | 125 ms (read 104 ms, mesh 16 ms), 36 MB, 34,088 triangles | 161 ms (read 126 ms, mesh 27 ms), 37 MB, 34,088 triangles | 78 ms (read 9.1 ms, mesh 66 ms), 22 MB, 34,588 triangles |
| grooved_shaft | 0.5 | 370 ms (read 176 ms, mesh 184 ms), 38 MB, 39,100 triangles | 510 ms (read 472 ms, mesh 21 ms), 59 MB, 70,590 triangles | 466 ms (read 412 ms, mesh 49 ms), 30 MB, 24,060 triangles | 84 ms (read 69 ms, mesh 12 ms), 24 MB, 15,036 triangles | 91 ms (read 75 ms, mesh 11 ms), 24 MB, 15,036 triangles | 62 ms (read 8.8 ms, mesh 49 ms), 15 MB, 27,068 triangles |
| plate_100_holes | 0.0125 | 393 ms (read 68 ms, mesh 321 ms), 52 MB, 20,252 triangles | 146 ms (read 75 ms, mesh 65 ms), 28 MB, 89,868 triangles | 501 ms (read 73 ms, mesh 405 ms), 67 MB, 127,788 triangles | 119 ms (read 14 ms, mesh 101 ms), 14 MB, 14,572 triangles | 123 ms (read 15 ms, mesh 103 ms), 15 MB, 14,572 triangles | 63 ms (read 1.8 ms, mesh 59 ms), 15 MB, 14,596 triangles |
| plate_100_holes | 0.05 | 177 ms (read 63 ms, mesh 111 ms), 35 MB, 10,940 triangles | 78 ms (read 59 ms, mesh 17 ms), 17 MB, 32,108 triangles | 203 ms (read 70 ms, mesh 125 ms), 27 MB, 38,308 triangles | 47 ms (read 14 ms, mesh 31 ms), 12 MB, 7,692 triangles | 49 ms (read 15 ms, mesh 32 ms), 12 MB, 7,692 triangles | 32 ms (read 1.7 ms, mesh 29 ms), 11 MB, 7,820 triangles |
| plate_100_holes | 0.1 | 172 ms (read 62 ms, mesh 107 ms), 35 MB, 10,924 triangles | 69 ms (read 61 ms, mesh 7.0 ms), 13 MB, 9,900 triangles | 114 ms (read 70 ms, mesh 41 ms), 14 MB, 12,900 triangles | 35 ms (read 14 ms, mesh 19 ms), 9.5 MB, 5,676 triangles | 36 ms (read 15 ms, mesh 20 ms), 10 MB, 5,676 triangles | 33 ms (read 1.7 ms, mesh 30 ms), 11 MB, 7,820 triangles |
| plate_100_holes | 0.5 | 167 ms (read 62 ms, mesh 103 ms), 35 MB, 10,924 triangles | 69 ms (read 60 ms, mesh 6.8 ms), 13 MB, 9,900 triangles | 94 ms (read 76 ms, mesh 17 ms), 9.6 MB, 5,044 triangles | 23 ms (read 14 ms, mesh 6.8 ms), 7.9 MB, 2,844 triangles | 23 ms (read 14 ms, mesh 7.3 ms), 8.5 MB, 2,844 triangles | 32 ms (read 1.8 ms, mesh 29 ms), 11 MB, 7,820 triangles |
| plate_900_holes | 0.0125 | 17.4 s (read 1.68 s, mesh 15.7 s), 1000 MB, 180,252 triangles | 948 ms (read 547 ms, mesh 358 ms), 148 MB, 801,868 triangles | 21.3 s (read 597 ms, mesh 20.6 s), 523 MB, 1,142,188 triangles | 6.61 s (read 130 ms, mesh 6.46 s), 98 MB, 129,772 triangles | 6.80 s (read 163 ms, mesh 6.62 s), 106 MB, 129,772 triangles | 2.29 s (read 15 ms, mesh 2.27 s), 102 MB, 129,796 triangles |
| plate_900_holes | 0.05 | 6.93 s (read 1.75 s, mesh 5.16 s), 394 MB, 97,340 triangles | 731 ms (read 568 ms, mesh 158 ms), 92 MB, 286,508 triangles | 5.93 s (read 614 ms, mesh 5.28 s), 177 MB, 342,308 triangles | 1.47 s (read 123 ms, mesh 1.33 s), 61 MB, 68,492 triangles | 1.53 s (read 127 ms, mesh 1.38 s), 65 MB, 68,492 triangles | 896 ms (read 15 ms, mesh 876 ms), 60 MB, 68,620 triangles |
| plate_900_holes | 0.1 | 6.79 s (read 1.69 s, mesh 5.09 s), 394 MB, 97,324 triangles | 601 ms (read 537 ms, mesh 59 ms), 79 MB, 88,300 triangles | 1.75 s (read 574 ms, mesh 1.16 s), 83 MB, 115,300 triangles | 840 ms (read 118 ms, mesh 712 ms), 59 MB, 50,476 triangles | 881 ms (read 125 ms, mesh 749 ms), 62 MB, 50,476 triangles | 911 ms (read 16 ms, mesh 890 ms), 60 MB, 68,620 triangles |
| plate_900_holes | 0.5 | 6.72 s (read 1.73 s, mesh 4.97 s), 395 MB, 97,324 triangles | 598 ms (read 534 ms, mesh 57 ms), 79 MB, 88,300 triangles | 877 ms (read 567 ms, mesh 302 ms), 55 MB, 45,044 triangles | 265 ms (read 114 ms, mesh 146 ms), 38 MB, 25,244 triangles | 280 ms (read 119 ms, mesh 155 ms), 40 MB, 25,244 triangles | 854 ms (read 15 ms, mesh 835 ms), 60 MB, 68,620 triangles |
| sheet_disc | 0.0125 | 91 ms (read 2.3 ms, mesh 87 ms), 31 MB, 4,768 triangles | 26 ms (read 8.8 ms, mesh 17 ms), 7.7 MB, 5,112 triangles | 40 ms (read 4.9 ms, mesh 35 ms), 11 MB, 4,092 triangles | 19 ms (read 0.4 ms, mesh 17 ms), 6.1 MB, 3,368 triangles | 20 ms (read 0.4 ms, mesh 18 ms), 6.6 MB, 3,368 triangles | – |
| sheet_disc | 0.05 | 28 ms (read 2.3 ms, mesh 24 ms), 20 MB, 2,384 triangles | 16 ms (read 8.6 ms, mesh 6.8 ms), 6.8 MB, 3,064 triangles | 18 ms (read 4.6 ms, mesh 12 ms), 7.6 MB, 2,044 triangles | 7.7 ms (read 0.4 ms, mesh 5.9 ms), 5.2 MB, 1,684 triangles | 8.1 ms (read 0.4 ms, mesh 6.1 ms), 5.7 MB, 1,684 triangles | – |
| sheet_disc | 0.1 | 18 ms (read 2.1 ms, mesh 15 ms), 19 MB, 1,684 triangles | 16 ms (read 8.6 ms, mesh 6.7 ms), 6.7 MB, 3,064 triangles | 18 ms (read 5.0 ms, mesh 12 ms), 7.4 MB, 2,044 triangles | 5.1 ms (read 0.4 ms, mesh 3.6 ms), 5.1 MB, 1,192 triangles | 5.6 ms (read 0.4 ms, mesh 3.6 ms), 5.4 MB, 1,192 triangles | – |
| sheet_disc | 0.5 | 8.4 ms (read 2.1 ms, mesh 4.8 ms), 17 MB, 752 triangles | 12 ms (read 8.9 ms, mesh 2.4 ms), 6.5 MB, 1,530 triangles | 11 ms (read 4.7 ms, mesh 4.6 ms), 5.8 MB, 956 triangles | 3.2 ms (read 0.4 ms, mesh 1.8 ms), 4.7 MB, 532 triangles | 3.3 ms (read 0.4 ms, mesh 1.7 ms), 5.1 MB, 532 triangles | – |
| tiny_pin | 0.0125 | 4.4 ms (read 2.0 ms, mesh 0.8 ms), 16 MB, 100 triangles | 2.1 ms (read 1.0 ms, mesh 0.4 ms), 5.0 MB, 150 triangles | 4.6 ms (read 2.4 ms, mesh 1.3 ms), 4.5 MB, 180 triangles | 2.3 ms (read 0.4 ms, mesh 1.0 ms), 4.3 MB, 36 triangles | 2.6 ms (read 0.4 ms, mesh 1.0 ms), 4.7 MB, 36 triangles | – |
| tiny_pin | 0.05 | 5.2 ms (read 2.5 ms, mesh 0.9 ms), 16 MB, 100 triangles | 2.5 ms (read 1.1 ms, mesh 0.4 ms), 5.1 MB, 150 triangles | 4.4 ms (read 2.4 ms, mesh 1.0 ms), 4.2 MB, 56 triangles | 2.3 ms (read 0.3 ms, mesh 1.0 ms), 4.4 MB, 16 triangles | 2.3 ms (read 0.4 ms, mesh 1.0 ms), 4.7 MB, 16 triangles | – |
| tiny_pin | 0.1 | 4.7 ms (read 2.2 ms, mesh 0.9 ms), 16 MB, 100 triangles | 2.1 ms (read 1.0 ms, mesh 0.4 ms), 5.1 MB, 150 triangles | 4.5 ms (read 2.4 ms, mesh 1.0 ms), 4.2 MB, 36 triangles | 2.6 ms (read 0.3 ms, mesh 1.2 ms), 4.3 MB, 12 triangles | 2.6 ms (read 0.4 ms, mesh 1.0 ms), 4.7 MB, 12 triangles | – |
| tiny_pin | 0.5 | 4.4 ms (read 2.1 ms, mesh 0.9 ms), 16 MB, 100 triangles | 2.1 ms (read 1.0 ms, mesh 0.5 ms), 5.0 MB, 150 triangles | – | 2.3 ms (read 0.4 ms, mesh 1.0 ms), 4.5 MB, 8 triangles | 2.4 ms (read 0.4 ms, mesh 0.9 ms), 4.7 MB, 8 triangles | – |
| torus_ring | 0.0125 | 57 ms (read 2.5 ms, mesh 52 ms), 27 MB, 12,218 triangles | 15 ms (read 2.2 ms, mesh 12 ms), 7.5 MB, 8,448 triangles | – | 11 ms (read 0.4 ms, mesh 9.3 ms), 6.5 MB, 4,772 triangles | 10 ms (read 0.4 ms, mesh 8.7 ms), 6.9 MB, 4,772 triangles | 15 ms (read 0.1 ms, mesh 14 ms), 11 MB, 15,978 triangles |
| torus_ring | 0.05 | 14 ms (read 2.4 ms, mesh 10 ms), 19 MB, 3,102 triangles | 6.0 ms (read 2.1 ms, mesh 3.2 ms), 5.8 MB, 2,176 triangles | – | 4.1 ms (read 0.3 ms, mesh 2.7 ms), 5.0 MB, 1,286 triangles | 4.2 ms (read 0.4 ms, mesh 2.8 ms), 5.2 MB, 1,286 triangles | 4.8 ms (read 0.1 ms, mesh 4.0 ms), 5.8 MB, 4,084 triangles |
| torus_ring | 0.1 | 10 ms (read 2.5 ms, mesh 5.9 ms), 17 MB, 1,894 triangles | 4.1 ms (read 2.0 ms, mesh 1.2 ms), 5.5 MB, 832 triangles | – | 3.1 ms (read 0.3 ms, mesh 1.8 ms), 4.5 MB, 632 triangles | 3.2 ms (read 0.4 ms, mesh 1.7 ms), 4.9 MB, 632 triangles | 3.0 ms (read 0.1 ms, mesh 2.3 ms), 4.8 MB, 2,108 triangles |
| torus_ring | 0.5 | 6.4 ms (read 2.2 ms, mesh 2.6 ms), 16 MB, 780 triangles | 3.5 ms (read 2.1 ms, mesh 0.7 ms), 5.4 MB, 288 triangles | – | 2.4 ms (read 0.3 ms, mesh 1.2 ms), 4.5 MB, 154 triangles | 2.6 ms (read 0.4 ms, mesh 1.1 ms), 4.7 MB, 154 triangles | 1.6 ms (read 0.1 ms, mesh 0.9 ms), 4.0 MB, 432 triangles |
| torus_ring_nurbs | 0.0125 | 106 ms (read 4.2 ms, mesh 98 ms), 29 MB, 14,690 triangles | – | 108 ms (read 3.9 ms, mesh 101 ms), 43 MB, 20,974 triangles | 88 ms (read 0.7 ms, mesh 84 ms), 15 MB, 32,832 triangles | 40 ms (read 0.7 ms, mesh 36 ms), 10 MB, 16,384 triangles | – |
| torus_ring_nurbs | 0.05 | 23 ms (read 3.9 ms, mesh 18 ms), 19 MB, 3,692 triangles | – | 28 ms (read 3.8 ms, mesh 23 ms), 14 MB, 5,366 triangles | 21 ms (read 0.7 ms, mesh 19 ms), 7.3 MB, 8,224 triangles | 10 ms (read 0.7 ms, mesh 8.0 ms), 6.7 MB, 4,096 triangles | – |
| torus_ring_nurbs | 0.1 | 18 ms (read 3.9 ms, mesh 12 ms), 18 MB, 2,344 triangles | – | 22 ms (read 3.6 ms, mesh 17 ms), 10 MB, 4,024 triangles | 17 ms (read 0.7 ms, mesh 15 ms), 6.9 MB, 6,514 triangles | 10 ms (read 0.7 ms, mesh 8.0 ms), 6.4 MB, 3,770 triangles | – |
| torus_ring_nurbs | 0.5 | 7.5 ms (read 3.7 ms, mesh 2.4 ms), 17 MB, 500 triangles | – | 7.6 ms (read 3.6 ms, mesh 3.2 ms), 5.1 MB, 396 triangles | 5.8 ms (read 0.7 ms, mesh 3.8 ms), 4.9 MB, 960 triangles | 4.5 ms (read 0.8 ms, mesh 2.5 ms), 4.9 MB, 576 triangles | 4.3 ms (read 0.1 ms, mesh 3.4 ms), 4.4 MB, 1,082 triangles |
| washer | 0.0125 | 8.9 ms (read 2.7 ms, mesh 4.7 ms), 17 MB, 756 triangles | 3.3 ms (read 1.3 ms, mesh 1.2 ms), 5.3 MB, 1,148 triangles | 8.8 ms (read 4.0 ms, mesh 3.9 ms), 5.1 MB, 894 triangles | 3.1 ms (read 0.5 ms, mesh 1.6 ms), 4.8 MB, 536 triangles | 3.2 ms (read 0.5 ms, mesh 1.6 ms), 5.2 MB, 536 triangles | – |
| washer | 0.05 | 6.4 ms (read 2.7 ms, mesh 2.1 ms), 16 MB, 380 triangles | 2.8 ms (read 1.4 ms, mesh 0.7 ms), 5.1 MB, 572 triangles | 8.2 ms (read 4.3 ms, mesh 2.5 ms), 4.6 MB, 384 triangles | 2.7 ms (read 0.5 ms, mesh 1.3 ms), 4.4 MB, 272 triangles | 2.8 ms (read 0.5 ms, mesh 1.3 ms), 4.7 MB, 272 triangles | – |
| washer | 0.1 | 5.8 ms (read 2.6 ms, mesh 1.6 ms), 16 MB, 284 triangles | 2.4 ms (read 1.3 ms, mesh 0.5 ms), 5.0 MB, 284 triangles | 7.1 ms (read 3.8 ms, mesh 2.3 ms), 4.4 MB, 288 triangles | 2.7 ms (read 0.5 ms, mesh 1.3 ms), 4.7 MB, 192 triangles | 2.9 ms (read 0.5 ms, mesh 1.2 ms), 4.8 MB, 192 triangles | – |
| washer | 0.5 | 5.5 ms (read 2.7 ms, mesh 1.3 ms), 16 MB, 208 triangles | 2.4 ms (read 1.3 ms, mesh 0.4 ms), 5.0 MB, 188 triangles | 7.4 ms (read 4.1 ms, mesh 2.4 ms), 4.1 MB, 96 triangles | 2.6 ms (read 0.5 ms, mesh 1.1 ms), 4.3 MB, 88 triangles | 2.7 ms (read 0.5 ms, mesh 1.2 ms), 4.7 MB, 88 triangles | 3.9 ms (read 0.1 ms, mesh 2.3 ms), 4.3 MB, 144 triangles |
