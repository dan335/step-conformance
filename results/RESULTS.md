# Results

Run 2026-10-12 on Darwin arm64, step-conformance as committed with these results.

- opencascade: opencascade-rs 0.3.0 with camcraft's patches, OCCT 7.8.1
- truck: truck-stepio 0.3.0, truck-meshalgo 0.4.0 (crates.io)
- truck-master: ricosjp/truck at 88ed005
- ogeom: 0.9.16 (crates.io)
- brepkit: 4.1.49 (crates.io)
- solidcraft: 113ded5 2026-10-12

Each cell: whether the reader passes, how far its mesh lies from the true surface
at worst, and how long it took. The true surface is the exact shape for the
generated parts, and OpenCascade's mesh at 0.001 mm for the rest. See the README
for how a reader is scored.

| part | tolerance mm | opencascade | ogeom | solidcraft | truck | truck-master | brepkit |
|---|---|---|---|---|---|---|---|
| bowl_01 | 0.0125 | ok, 0.042 mm, 2.5 s | ok, 0.021 mm, 0.6 s | ok, 0.012 mm, 1.2 s | timed out at 60 s | 10.000 mm off, 0.6 s | could not read: read_step found no solid (it reads MANIFOLD_SOLID_BREP and BREP_WITH_VOIDS only) |
| bowl_01 | 0.05 | ok, 0.050 mm, 0.1 s | ok, 0.082 mm, 0.0 s | ok, 0.047 mm, 0.3 s | 53.498 mm off, 5.5 s | 0.218 mm off, 0.1 s | could not read: read_step found no solid (it reads MANIFOLD_SOLID_BREP and BREP_WITH_VOIDS only) |
| bowl_01 | 0.1 | ok, 0.131 mm, 0.2 s | ok, 0.105 mm, 0.0 s | ok, 0.075 mm, 0.1 s | 53.503 mm off, 5.4 s | 0.430 mm off, 0.0 s | could not read: read_step found no solid (it reads MANIFOLD_SOLID_BREP and BREP_WITH_VOIDS only) |
| bowl_01 | 0.5 | ok, 0.667 mm, 0.1 s | ok, 0.416 mm, 0.0 s | ok, 0.401 mm, 0.1 s | 53.546 mm off, 0.0 s | 2.111 mm off, 0.0 s | could not read: read_step found no solid (it reads MANIFOLD_SOLID_BREP and BREP_WITH_VOIDS only) |
| door_handle_guide | 0.0125 | ok, 0.008 mm, 0.1 s | ok, 0.008 mm, 0.0 s | ok, 0.008 mm, 0.1 s | ok, 0.012 mm, 0.0 s | ok, 0.012 mm, 0.0 s | ok, 0.017 mm, 0.0 s |
| door_handle_guide | 0.05 | ok, 0.020 mm, 0.1 s | ok, 0.031 mm, 0.0 s | ok, 0.034 mm, 0.1 s | ok, 0.044 mm, 0.0 s | ok, 0.044 mm, 0.0 s | ok, 0.064 mm, 0.0 s |
| door_handle_guide | 0.1 | ok, 0.020 mm, 0.1 s | ok, 0.061 mm, 0.0 s | ok, 0.066 mm, 0.1 s | ok, 0.089 mm, 0.0 s | ok, 0.089 mm, 0.1 s | ok, 0.039 mm, 0.0 s |
| door_handle_guide | 0.5 | ok, 0.020 mm, 0.1 s | ok, 0.061 mm, 0.0 s | ok, 0.242 mm, 0.0 s | ok, 0.353 mm, 0.0 s | ok, 0.353 mm, 0.0 s | ok, 0.039 mm, 0.0 s |
| hinge_jig | 0.0125 | ok, 0.000 mm, 0.0 s | ok, 0.000 mm, 0.0 s | ok, 0.000 mm, 0.0 s | ok, 0.000 mm, 0.0 s | ok, 0.000 mm, 0.0 s | ok, 0.000 mm, 0.0 s |
| hinge_jig | 0.05 | ok, 0.000 mm, 0.0 s | ok, 0.000 mm, 0.0 s | ok, 0.000 mm, 0.0 s | ok, 0.000 mm, 0.0 s | ok, 0.000 mm, 0.0 s | ok, 0.000 mm, 0.0 s |
| hinge_jig | 0.1 | ok, 0.000 mm, 0.0 s | ok, 0.000 mm, 0.0 s | ok, 0.000 mm, 0.0 s | ok, 0.000 mm, 0.0 s | ok, 0.000 mm, 0.0 s | ok, 0.000 mm, 0.0 s |
| hinge_jig | 0.5 | ok, 0.000 mm, 0.0 s | ok, 0.000 mm, 0.0 s | ok, 0.000 mm, 0.0 s | ok, 0.000 mm, 0.0 s | ok, 0.000 mm, 0.0 s | ok, 0.000 mm, 0.0 s |
| lego_3_pins | 0.0125 | ok, 0.008 mm, 0.2 s | ok, 0.011 mm, 0.1 s | ok, 0.012 mm, 0.2 s | ok, 0.012 mm, 0.1 s | ok, 0.012 mm, 0.1 s | ok, 0.017 mm, 0.1 s |
| lego_3_pins | 0.05 | ok, 0.030 mm, 0.2 s | ok, 0.044 mm, 0.1 s | ok, 0.049 mm, 0.1 s | ok, 0.048 mm, 0.1 s | ok, 0.048 mm, 0.0 s | ok, 0.096 mm, 0.0 s |
| lego_3_pins | 0.1 | ok, 0.058 mm, 0.1 s | ok, 0.061 mm, 0.1 s | ok, 0.066 mm, 0.1 s | ok, 0.094 mm, 0.1 s | ok, 0.094 mm, 0.0 s | ok, 0.183 mm, 0.0 s |
| lego_3_pins | 0.5 | ok, 0.067 mm, 0.1 s | ok, 0.177 mm, 0.1 s | ok, 0.298 mm, 0.1 s | ok, 0.498 mm, 0.1 s | ok, 0.498 mm, 0.1 s | ok, 0.140 mm, 0.0 s |
| lego_4x2_01 | 0.0125 | ok, 0.008 mm, 0.1 s | ok, 0.011 mm, 0.0 s | ok, 0.012 mm, 0.1 s | ok, 0.012 mm, 0.0 s | ok, 0.012 mm, 0.0 s | ok, 0.017 mm, 0.0 s |
| lego_4x2_01 | 0.05 | ok, 0.030 mm, 0.1 s | ok, 0.044 mm, 0.0 s | ok, 0.049 mm, 0.1 s | ok, 0.048 mm, 0.0 s | ok, 0.048 mm, 0.0 s | ok, 0.096 mm, 0.0 s |
| lego_4x2_01 | 0.1 | ok, 0.058 mm, 0.1 s | ok, 0.061 mm, 0.1 s | ok, 0.066 mm, 0.0 s | ok, 0.094 mm, 0.0 s | ok, 0.094 mm, 0.0 s | ok, 0.183 mm, 0.0 s |
| lego_4x2_01 | 0.5 | ok, 0.067 mm, 0.1 s | ok, 0.177 mm, 0.0 s | ok, 0.298 mm, 0.1 s | ok, 0.498 mm, 0.0 s | ok, 0.498 mm, 0.0 s | ok, 0.140 mm, 0.0 s |
| rail_guide | 0.0125 | ok, 0.007 mm, 0.1 s | ok, 0.013 mm, 0.1 s | ok, 0.012 mm, 0.1 s | ok, 0.012 mm, 0.1 s | ok, 0.012 mm, 0.0 s | ok, 0.014 mm, 0.0 s |
| rail_guide | 0.05 | ok, 0.028 mm, 0.1 s | ok, 0.048 mm, 0.0 s | ok, 0.048 mm, 0.1 s | ok, 0.048 mm, 0.0 s | ok, 0.048 mm, 0.0 s | ok, 0.039 mm, 0.0 s |
| rail_guide | 0.1 | ok, 0.047 mm, 0.1 s | ok, 0.055 mm, 0.0 s | ok, 0.064 mm, 0.1 s | ok, 0.095 mm, 0.0 s | ok, 0.095 mm, 0.0 s | ok, 0.078 mm, 0.0 s |
| rail_guide | 0.5 | ok, 0.069 mm, 0.1 s | ok, 0.122 mm, 0.1 s | ok, 0.483 mm, 0.1 s | ok, 0.483 mm, 0.0 s | ok, 0.483 mm, 0.0 s | ok, 0.078 mm, 0.0 s |
| sled_base | 0.0125 | ok, 0.014 mm, 0.3 s | ok, 0.017 mm, 0.1 s | ok, 0.012 mm, 0.4 s | ok, 0.021 mm, 0.2 s | ok, 0.043 mm, 0.1 s | 3.792 mm off, 0.1 s |
| sled_base | 0.05 | ok, 0.052 mm, 0.2 s | ok, 0.067 mm, 0.1 s | ok, 0.049 mm, 0.3 s | ok, 0.084 mm, 0.1 s | 0.818 mm off, 0.1 s | 4.149 mm off, 0.0 s |
| sled_base | 0.1 | ok, 0.095 mm, 0.2 s | ok, 0.119 mm, 0.1 s | ok, 0.098 mm, 0.2 s | timed out at 60 s | ok, 0.223 mm, 0.1 s | 4.111 mm off, 0.0 s |
| sled_base | 0.5 | ok, 0.371 mm, 0.2 s | ok, 0.252 mm, 0.1 s | ok, 0.483 mm, 0.3 s | 4.953 mm off, 0.1 s | ok, 0.621 mm, 0.1 s | face 357 empty, 5.959 mm off |
| cone_point | 0.0125 | ok, 0.011 mm, 0.0 s | 0.092 mm off, 0.0 s | 30.000 mm off, 0.0 s | outside the file by 0.3 mm on +Y, 30.000 mm off | outside the file by 0.3 mm on +Y, 30.000 mm off | 14.813 mm off, 0.0 s |
| cone_point | 0.05 | ok, 0.039 mm, 0.0 s | ok, 0.137 mm, 0.0 s | 30.000 mm off, 0.0 s | outside the file by 0.3 mm on +Y, 30.000 mm off | outside the file by 0.3 mm on +Y, 30.000 mm off | 14.813 mm off, 0.0 s |
| cone_point | 0.1 | ok, 0.073 mm, 0.0 s | ok, 0.235 mm, 0.0 s | 30.000 mm off, 0.0 s | outside the file by 0.2 mm on +Y, 30.000 mm off | outside the file by 0.2 mm on +Y, 30.000 mm off | face 1 empty, 30.000 mm off |
| cone_point | 0.5 | ok, 0.205 mm, 0.0 s | ok, 0.256 mm, 0.0 s | 30.000 mm off, 0.0 s | 30.000 mm off, 0.0 s | 30.000 mm off, 0.0 s | face 1 empty, 30.000 mm off |
| cone_point_vertex | 0.0125 | ok, 0.011 mm, 0.0 s | 0.092 mm off, 0.0 s | short of the vertices by 30.0 mm on +Z, 30.000 mm off | outside the file by 0.3 mm on +Y, 30.000 mm off | outside the file by 0.3 mm on +Y, 30.000 mm off | could not read: parse error: failed to create wire from edge loop #44: empty wire — at least one |
| cone_point_vertex | 0.05 | ok, 0.039 mm, 0.0 s | ok, 0.137 mm, 0.0 s | short of the vertices by 29.9 mm on +Z, 30.000 mm off | outside the file by 0.3 mm on +Y, 30.000 mm off | outside the file by 0.3 mm on +Y, 30.000 mm off | could not read: parse error: failed to create wire from edge loop #44: empty wire — at least one |
| cone_point_vertex | 0.1 | ok, 0.073 mm, 0.0 s | ok, 0.235 mm, 0.0 s | short of the vertices by 29.9 mm on +Z, 30.000 mm off | outside the file by 0.2 mm on +Y, 30.000 mm off | outside the file by 0.2 mm on +Y, 30.000 mm off | could not read: parse error: failed to create wire from edge loop #44: empty wire — at least one |
| cone_point_vertex | 0.5 | ok, 0.205 mm, 0.0 s | ok, 0.256 mm, 0.0 s | short of the vertices by 29.5 mm on +Z, 30.000 mm off | short of the vertices by 29.5 mm on +Z, 30.000 mm off | short of the vertices by 29.5 mm on +Z, 30.000 mm off | could not read: parse error: failed to create wire from edge loop #44: empty wire — at least one |
| cylinder | 0.0125 | ok, 0.006 mm, 0.0 s | ok, 0.012 mm, 0.0 s | ok, 0.012 mm, 0.0 s | ok, 0.012 mm, 0.0 s | ok, 0.012 mm, 0.0 s | 14.469 mm off, 0.0 s |
| cylinder | 0.05 | ok, 0.024 mm, 0.0 s | ok, 0.048 mm, 0.0 s | ok, 0.050 mm, 0.0 s | ok, 0.048 mm, 0.0 s | ok, 0.048 mm, 0.0 s | 14.469 mm off, 0.0 s |
| cylinder | 0.1 | ok, 0.048 mm, 0.0 s | ok, 0.048 mm, 0.0 s | ok, 0.053 mm, 0.0 s | ok, 0.093 mm, 0.0 s | ok, 0.093 mm, 0.0 s | 14.469 mm off, 0.0 s |
| cylinder | 0.5 | ok, 0.073 mm, 0.0 s | ok, 0.192 mm, 0.0 s | ok, 0.218 mm, 0.0 s | ok, 0.489 mm, 0.0 s | ok, 0.489 mm, 0.0 s | 14.470 mm off, 0.0 s |
| cylinder_inch | 0.0125 | ok, 0.006 mm, 0.0 s | ok, 0.012 mm, 0.0 s | ok, 0.012 mm, 0.0 s | ok, 0.012 mm, 0.0 s | ok, 0.012 mm, 0.0 s | outside the file by 15.4 mm on -Z, 25.400 mm off |
| cylinder_inch | 0.05 | ok, 0.024 mm, 0.0 s | ok, 0.048 mm, 0.0 s | ok, 0.050 mm, 0.0 s | ok, 0.048 mm, 0.0 s | ok, 0.048 mm, 0.0 s | outside the file by 15.3 mm on -Z, 25.400 mm off |
| cylinder_inch | 0.1 | ok, 0.048 mm, 0.0 s | ok, 0.048 mm, 0.0 s | ok, 0.053 mm, 0.0 s | ok, 0.093 mm, 0.0 s | ok, 0.093 mm, 0.0 s | outside the file by 15.3 mm on -Z, 25.400 mm off |
| cylinder_inch | 0.5 | ok, 0.073 mm, 0.0 s | ok, 0.192 mm, 0.0 s | ok, 0.218 mm, 0.0 s | ok, 0.489 mm, 0.0 s | ok, 0.489 mm, 0.0 s | outside the file by 14.9 mm on -Z, 25.400 mm off |
| cylinder_nurbs | 0.0125 | ok, 0.006 mm, 0.0 s | face 1 empty, 15.000 mm off | ok, 0.012 mm, 0.0 s | ok, 0.012 mm, 0.1 s | ok, 0.012 mm, 0.0 s | ok, 0.013 mm, 0.0 s |
| cylinder_nurbs | 0.05 | ok, 0.025 mm, 0.0 s | face 1 empty, 15.000 mm off | ok, 0.046 mm, 0.0 s | ok, 0.050 mm, 0.0 s | ok, 0.050 mm, 0.0 s | ok, 0.040 mm, 0.0 s |
| cylinder_nurbs | 0.1 | ok, 0.049 mm, 0.0 s | face 1 empty, 15.000 mm off | ok, 0.053 mm, 0.0 s | ok, 0.053 mm, 0.0 s | ok, 0.053 mm, 0.0 s | ok, 0.040 mm, 0.0 s |
| cylinder_nurbs | 0.5 | ok, 0.077 mm, 0.0 s | face 1 empty, 15.000 mm off | ok, 0.207 mm, 0.0 s | ok, 0.207 mm, 0.0 s | ok, 0.207 mm, 0.0 s | ok, 0.040 mm, 0.0 s |
| cylinder_placed | 0.0125 | ok, 0.006 mm, 0.0 s | ok, 0.012 mm, 0.0 s | ok, 0.012 mm, 0.0 s | outside the file by 40.0 mm on -X, 56.630 mm off | ok, 0.012 mm, 0.0 s | outside the file by 40.0 mm on -X, 56.957 mm off |
| cylinder_placed | 0.05 | ok, 0.024 mm, 0.0 s | ok, 0.048 mm, 0.0 s | ok, 0.050 mm, 0.0 s | outside the file by 39.9 mm on -X, 56.630 mm off | ok, 0.048 mm, 0.0 s | outside the file by 39.9 mm on -X, 56.948 mm off |
| cylinder_placed | 0.1 | ok, 0.048 mm, 0.0 s | ok, 0.048 mm, 0.0 s | ok, 0.053 mm, 0.0 s | outside the file by 39.8 mm on -X, 56.667 mm off | ok, 0.093 mm, 0.0 s | outside the file by 39.9 mm on -X, 57.025 mm off |
| cylinder_placed | 0.5 | ok, 0.073 mm, 0.0 s | ok, 0.192 mm, 0.0 s | ok, 0.218 mm, 0.0 s | outside the file by 39.5 mm on -X, 56.851 mm off | ok, 0.489 mm, 0.0 s | outside the file by 39.3 mm on -X, 56.947 mm off |
| cylinder_seam | 0.0125 | ok, 0.006 mm, 0.0 s | ok, 0.012 mm, 0.0 s | ok, 0.012 mm, 0.0 s | ok, 0.012 mm, 0.0 s | ok, 0.012 mm, 0.0 s | ok, 0.012 mm, 0.0 s |
| cylinder_seam | 0.05 | ok, 0.024 mm, 0.0 s | ok, 0.048 mm, 0.0 s | ok, 0.048 mm, 0.0 s | ok, 0.048 mm, 0.0 s | ok, 0.048 mm, 0.0 s | ok, 0.048 mm, 0.0 s |
| cylinder_seam | 0.1 | ok, 0.048 mm, 0.0 s | ok, 0.048 mm, 0.0 s | ok, 0.053 mm, 0.0 s | ok, 0.093 mm, 0.0 s | ok, 0.093 mm, 0.0 s | ok, 0.093 mm, 0.0 s |
| cylinder_seam | 0.5 | ok, 0.073 mm, 0.0 s | ok, 0.192 mm, 0.0 s | ok, 0.207 mm, 0.0 s | ok, 0.489 mm, 0.0 s | ok, 0.489 mm, 0.0 s | ok, 0.152 mm, 0.0 s |
| cylinder_twice | 0.0125 | ok, 0.006 mm, 0.0 s | ok, 0.012 mm, 0.0 s | ok, 0.012 mm, 0.0 s | short of the vertices by 80.0 mm on +X, 80.555 mm off | ok, 0.012 mm, 0.0 s | short of the vertices by 80.0 mm on +X, 81.677 mm off |
| cylinder_twice | 0.05 | ok, 0.024 mm, 0.0 s | ok, 0.048 mm, 0.0 s | ok, 0.050 mm, 0.0 s | short of the vertices by 79.9 mm on +X, 80.595 mm off | ok, 0.048 mm, 0.0 s | short of the vertices by 79.9 mm on +X, 81.716 mm off |
| cylinder_twice | 0.1 | ok, 0.048 mm, 0.0 s | ok, 0.048 mm, 0.0 s | ok, 0.053 mm, 0.0 s | short of the vertices by 79.9 mm on +X, 80.623 mm off | ok, 0.093 mm, 0.0 s | short of the vertices by 79.9 mm on +X, 81.741 mm off |
| cylinder_twice | 0.5 | ok, 0.073 mm, 0.0 s | ok, 0.192 mm, 0.0 s | ok, 0.218 mm, 0.0 s | short of the vertices by 79.5 mm on +X, 80.623 mm off | ok, 0.489 mm, 0.0 s | short of the vertices by 79.7 mm on +X, 81.708 mm off |
| dome | 0.0125 | ok, 0.012 mm, 0.1 s | 20.000 mm off, 0.0 s | ok, 0.012 mm, 0.1 s | ok, 0.024 mm, 0.0 s | ok, 0.024 mm, 0.0 s | ok, 0.012 mm, 0.1 s |
| dome | 0.05 | ok, 0.048 mm, 0.0 s | 20.000 mm off, 0.0 s | ok, 0.047 mm, 0.0 s | ok, 0.091 mm, 0.0 s | ok, 0.091 mm, 0.0 s | ok, 0.049 mm, 0.0 s |
| dome | 0.1 | ok, 0.083 mm, 0.0 s | 20.000 mm off, 0.0 s | ok, 0.093 mm, 0.0 s | ok, 0.191 mm, 0.0 s | ok, 0.191 mm, 0.0 s | ok, 0.096 mm, 0.0 s |
| dome | 0.5 | ok, 0.345 mm, 0.0 s | 20.000 mm off, 0.0 s | ok, 0.415 mm, 0.0 s | ok, 0.797 mm, 0.0 s | ok, 0.797 mm, 0.0 s | ok, 0.304 mm, 0.0 s |
| dome_nurbs | 0.0125 | ok, 0.013 mm, 0.2 s | face 1 empty, 20.000 mm off | ok, 0.012 mm, 0.1 s | ok, 0.008 mm, 0.1 s | ok, 0.008 mm, 0.1 s | 3.906 mm off, 0.1 s |
| dome_nurbs | 0.05 | ok, 0.050 mm, 0.0 s | face 1 empty, 20.000 mm off | ok, 0.047 mm, 0.0 s | ok, 0.032 mm, 0.0 s | ok, 0.032 mm, 0.0 s | 3.907 mm off, 0.0 s |
| dome_nurbs | 0.1 | ok, 0.087 mm, 0.1 s | face 1 empty, 20.000 mm off | ok, 0.093 mm, 0.0 s | ok, 0.131 mm, 0.0 s | ok, 0.100 mm, 0.0 s | 3.913 mm off, 0.0 s |
| dome_nurbs | 0.5 | ok, 0.400 mm, 0.0 s | face 1 empty, 20.000 mm off | ok, 0.415 mm, 0.0 s | ok, 0.538 mm, 0.0 s | ok, 0.538 mm, 0.0 s | 3.938 mm off, 0.0 s |
| far_cylinder | 0.0125 | ok, 0.006 mm, 0.0 s | ok, 0.012 mm, 0.0 s | ok, 0.012 mm, 0.0 s | ok, 0.012 mm, 0.0 s | ok, 0.012 mm, 0.0 s | 14.469 mm off, 0.0 s |
| far_cylinder | 0.05 | ok, 0.024 mm, 0.0 s | ok, 0.048 mm, 0.0 s | ok, 0.046 mm, 0.0 s | ok, 0.048 mm, 0.0 s | ok, 0.048 mm, 0.0 s | 14.469 mm off, 0.0 s |
| far_cylinder | 0.1 | ok, 0.048 mm, 0.0 s | ok, 0.048 mm, 0.0 s | ok, 0.053 mm, 0.0 s | ok, 0.093 mm, 0.0 s | ok, 0.093 mm, 0.0 s | 14.469 mm off, 0.0 s |
| far_cylinder | 0.5 | ok, 0.073 mm, 0.0 s | ok, 0.192 mm, 0.0 s | ok, 0.216 mm, 0.0 s | ok, 0.489 mm, 0.0 s | ok, 0.489 mm, 0.0 s | 14.470 mm off, 0.0 s |
| sheet_disc | 0.0125 | ok, 0.006 mm, 0.1 s | ok, 0.008 mm, 0.0 s | ok, 0.009 mm, 0.0 s | ok, 0.012 mm, 0.0 s | ok, 0.012 mm, 0.0 s | 8.906 mm off, 0.1 s |
| sheet_disc | 0.05 | ok, 0.025 mm, 0.1 s | ok, 0.030 mm, 0.0 s | ok, 0.037 mm, 0.0 s | ok, 0.050 mm, 0.0 s | ok, 0.050 mm, 0.0 s | 8.906 mm off, 0.0 s |
| sheet_disc | 0.1 | ok, 0.050 mm, 0.0 s | ok, 0.030 mm, 0.0 s | ok, 0.037 mm, 0.0 s | ok, 0.099 mm, 0.0 s | ok, 0.099 mm, 0.0 s | 8.907 mm off, 0.0 s |
| sheet_disc | 0.5 | ok, 0.249 mm, 0.0 s | ok, 0.120 mm, 0.0 s | ok, 0.443 mm, 0.0 s | ok, 0.495 mm, 0.0 s | ok, 0.495 mm, 0.0 s | 8.920 mm off, 0.0 s |
| tiny_pin | 0.0125 | ok, 0.002 mm, 0.0 s | ok, 0.005 mm, 0.0 s | ok, 0.005 mm, 0.0 s | ok, 0.012 mm, 0.0 s | ok, 0.012 mm, 0.0 s | outside the file by 0.7 mm on -Z, 1.000 mm off |
| tiny_pin | 0.05 | ok, 0.002 mm, 0.0 s | ok, 0.005 mm, 0.0 s | ok, 0.024 mm, 0.0 s | ok, 0.048 mm, 0.0 s | ok, 0.048 mm, 0.0 s | face 0 empty, 1000.000 mm off |
| tiny_pin | 0.1 | ok, 0.002 mm, 0.0 s | ok, 0.005 mm, 0.0 s | ok, 0.073 mm, 0.0 s | ok, 0.073 mm, 0.0 s | ok, 0.073 mm, 0.0 s | face 0 empty, 1000.000 mm off |
| tiny_pin | 0.5 | ok, 0.002 mm, 0.0 s | ok, 0.005 mm, 0.0 s | outside the file by 0.2 mm on +Y, 1.164 mm off | ok, 0.125 mm, 0.0 s | ok, 0.125 mm, 0.0 s | face 0 empty, 1000.000 mm off |
| torus_ring | 0.0125 | ok, 0.016 mm, 0.1 s | ok, 0.014 mm, 0.0 s | 5.000 mm off, 0.2 s | ok, 0.024 mm, 0.0 s | ok, 0.024 mm, 0.0 s | ok, 0.012 mm, 0.0 s |
| torus_ring | 0.05 | ok, 0.062 mm, 0.0 s | ok, 0.054 mm, 0.0 s | 5.000 mm off, 0.1 s | ok, 0.091 mm, 0.0 s | ok, 0.091 mm, 0.0 s | ok, 0.053 mm, 0.0 s |
| torus_ring | 0.1 | ok, 0.108 mm, 0.0 s | ok, 0.125 mm, 0.0 s | 5.000 mm off, 0.0 s | ok, 0.189 mm, 0.0 s | ok, 0.189 mm, 0.0 s | ok, 0.105 mm, 0.0 s |
| torus_ring | 0.5 | ok, 0.213 mm, 0.0 s | ok, 0.564 mm, 0.0 s | 11.352 mm off, 0.0 s | ok, 0.813 mm, 0.0 s | ok, 0.813 mm, 0.0 s | ok, 0.390 mm, 0.0 s |
| torus_ring_nurbs | 0.0125 | ok, 0.017 mm, 0.2 s | face 1 empty, 5.000 mm off | ok, 0.008 mm, 0.1 s | ok, 0.009 mm, 0.1 s | ok, 0.010 mm, 0.1 s | 0.132 mm off, 0.1 s |
| torus_ring_nurbs | 0.05 | ok, 0.054 mm, 0.0 s | face 1 empty, 5.000 mm off | ok, 0.033 mm, 0.0 s | ok, 0.037 mm, 0.0 s | ok, 0.041 mm, 0.0 s | 0.362 mm off, 0.0 s |
| torus_ring_nurbs | 0.1 | ok, 0.101 mm, 0.1 s | face 1 empty, 5.000 mm off | ok, 0.088 mm, 0.0 s | ok, 0.088 mm, 0.0 s | ok, 0.088 mm, 0.0 s | 0.630 mm off, 0.0 s |
| torus_ring_nurbs | 0.5 | ok, 0.414 mm, 0.0 s | face 1 empty, 5.000 mm off | ok, 0.441 mm, 0.0 s | ok, 0.614 mm, 0.0 s | ok, 0.441 mm, 0.0 s | ok, 0.675 mm, 0.0 s |
| washer | 0.0125 | ok, 0.006 mm, 0.0 s | ok, 0.006 mm, 0.0 s | ok, 0.007 mm, 0.0 s | ok, 0.012 mm, 0.0 s | ok, 0.012 mm, 0.0 s | 1.000 mm off, 0.0 s |
| washer | 0.05 | ok, 0.025 mm, 0.0 s | ok, 0.024 mm, 0.0 s | ok, 0.027 mm, 0.0 s | ok, 0.049 mm, 0.0 s | ok, 0.049 mm, 0.0 s | 1.000 mm off, 0.0 s |
| washer | 0.1 | ok, 0.049 mm, 0.0 s | ok, 0.096 mm, 0.0 s | ok, 0.093 mm, 0.0 s | ok, 0.096 mm, 0.0 s | ok, 0.096 mm, 0.0 s | 1.000 mm off, 0.0 s |
| washer | 0.5 | ok, 0.146 mm, 0.0 s | ok, 0.384 mm, 0.0 s | ok, 0.415 mm, 0.0 s | ok, 0.495 mm, 0.0 s | ok, 0.495 mm, 0.0 s | ok, 1.030 mm, 0.0 s |

**opencascade** passes 92 of 92.

**ogeom** passes 74 of 92.
- cone_point at 0.0125 mm: 0.092 mm off, 0.0 s
- cone_point_vertex at 0.0125 mm: 0.092 mm off, 0.0 s
- cylinder_nurbs at 0.0125 mm: face 1 empty, 15.000 mm off
- cylinder_nurbs at 0.05 mm: face 1 empty, 15.000 mm off
- cylinder_nurbs at 0.1 mm: face 1 empty, 15.000 mm off
- cylinder_nurbs at 0.5 mm: face 1 empty, 15.000 mm off
- dome at 0.0125 mm: 20.000 mm off, 0.0 s
- dome at 0.05 mm: 20.000 mm off, 0.0 s
- dome at 0.1 mm: 20.000 mm off, 0.0 s
- dome at 0.5 mm: 20.000 mm off, 0.0 s
- dome_nurbs at 0.0125 mm: face 1 empty, 20.000 mm off
- dome_nurbs at 0.05 mm: face 1 empty, 20.000 mm off
- dome_nurbs at 0.1 mm: face 1 empty, 20.000 mm off
- dome_nurbs at 0.5 mm: face 1 empty, 20.000 mm off
- torus_ring_nurbs at 0.0125 mm: face 1 empty, 5.000 mm off
- torus_ring_nurbs at 0.05 mm: face 1 empty, 5.000 mm off
- torus_ring_nurbs at 0.1 mm: face 1 empty, 5.000 mm off
- torus_ring_nurbs at 0.5 mm: face 1 empty, 5.000 mm off

**solidcraft** passes 79 of 92.
- cone_point at 0.0125 mm: 30.000 mm off, 0.0 s
- cone_point at 0.05 mm: 30.000 mm off, 0.0 s
- cone_point at 0.1 mm: 30.000 mm off, 0.0 s
- cone_point at 0.5 mm: 30.000 mm off, 0.0 s
- cone_point_vertex at 0.0125 mm: short of the vertices by 30.0 mm on +Z, 30.000 mm off
- cone_point_vertex at 0.05 mm: short of the vertices by 29.9 mm on +Z, 30.000 mm off
- cone_point_vertex at 0.1 mm: short of the vertices by 29.9 mm on +Z, 30.000 mm off
- cone_point_vertex at 0.5 mm: short of the vertices by 29.5 mm on +Z, 30.000 mm off
- tiny_pin at 0.5 mm: outside the file by 0.2 mm on +Y, 1.164 mm off
- torus_ring at 0.0125 mm: 5.000 mm off, 0.2 s
- torus_ring at 0.05 mm: 5.000 mm off, 0.1 s
- torus_ring at 0.1 mm: 5.000 mm off, 0.0 s
- torus_ring at 0.5 mm: 11.352 mm off, 0.0 s

**truck** passes 70 of 92.
- bowl_01 at 0.0125 mm: timed out at 60 s
- bowl_01 at 0.05 mm: 53.498 mm off, 5.5 s
- bowl_01 at 0.1 mm: 53.503 mm off, 5.4 s
- bowl_01 at 0.5 mm: 53.546 mm off, 0.0 s
- sled_base at 0.1 mm: timed out at 60 s
- sled_base at 0.5 mm: 4.953 mm off, 0.1 s
- cone_point at 0.0125 mm: outside the file by 0.3 mm on +Y, 30.000 mm off
- cone_point at 0.05 mm: outside the file by 0.3 mm on +Y, 30.000 mm off
- cone_point at 0.1 mm: outside the file by 0.2 mm on +Y, 30.000 mm off
- cone_point at 0.5 mm: 30.000 mm off, 0.0 s
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

**truck-master** passes 79 of 92.
- bowl_01 at 0.0125 mm: 10.000 mm off, 0.6 s
- bowl_01 at 0.05 mm: 0.218 mm off, 0.1 s
- bowl_01 at 0.1 mm: 0.430 mm off, 0.0 s
- bowl_01 at 0.5 mm: 2.111 mm off, 0.0 s
- sled_base at 0.05 mm: 0.818 mm off, 0.1 s
- cone_point at 0.0125 mm: outside the file by 0.3 mm on +Y, 30.000 mm off
- cone_point at 0.05 mm: outside the file by 0.3 mm on +Y, 30.000 mm off
- cone_point at 0.1 mm: outside the file by 0.2 mm on +Y, 30.000 mm off
- cone_point at 0.5 mm: 30.000 mm off, 0.0 s
- cone_point_vertex at 0.0125 mm: outside the file by 0.3 mm on +Y, 30.000 mm off
- cone_point_vertex at 0.05 mm: outside the file by 0.3 mm on +Y, 30.000 mm off
- cone_point_vertex at 0.1 mm: outside the file by 0.2 mm on +Y, 30.000 mm off
- cone_point_vertex at 0.5 mm: short of the vertices by 29.5 mm on +Z, 30.000 mm off

**brepkit** passes 38 of 92.
- bowl_01 at 0.0125 mm: could not read: read_step found no solid (it reads MANIFOLD_SOLID_BREP and BREP_WITH_VOIDS only)
- bowl_01 at 0.05 mm: could not read: read_step found no solid (it reads MANIFOLD_SOLID_BREP and BREP_WITH_VOIDS only)
- bowl_01 at 0.1 mm: could not read: read_step found no solid (it reads MANIFOLD_SOLID_BREP and BREP_WITH_VOIDS only)
- bowl_01 at 0.5 mm: could not read: read_step found no solid (it reads MANIFOLD_SOLID_BREP and BREP_WITH_VOIDS only)
- sled_base at 0.0125 mm: 3.792 mm off, 0.1 s
- sled_base at 0.05 mm: 4.149 mm off, 0.0 s
- sled_base at 0.1 mm: 4.111 mm off, 0.0 s
- sled_base at 0.5 mm: face 357 empty, 5.959 mm off
- cone_point at 0.0125 mm: 14.813 mm off, 0.0 s
- cone_point at 0.05 mm: 14.813 mm off, 0.0 s
- cone_point at 0.1 mm: face 1 empty, 30.000 mm off
- cone_point at 0.5 mm: face 1 empty, 30.000 mm off
- cone_point_vertex at 0.0125 mm: could not read: parse error: failed to create wire from edge loop #44: empty wire — at least one
- cone_point_vertex at 0.05 mm: could not read: parse error: failed to create wire from edge loop #44: empty wire — at least one
- cone_point_vertex at 0.1 mm: could not read: parse error: failed to create wire from edge loop #44: empty wire — at least one
- cone_point_vertex at 0.5 mm: could not read: parse error: failed to create wire from edge loop #44: empty wire — at least one
- cylinder at 0.0125 mm: 14.469 mm off, 0.0 s
- cylinder at 0.05 mm: 14.469 mm off, 0.0 s
- cylinder at 0.1 mm: 14.469 mm off, 0.0 s
- cylinder at 0.5 mm: 14.470 mm off, 0.0 s
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
- dome_nurbs at 0.0125 mm: 3.906 mm off, 0.1 s
- dome_nurbs at 0.05 mm: 3.907 mm off, 0.0 s
- dome_nurbs at 0.1 mm: 3.913 mm off, 0.0 s
- dome_nurbs at 0.5 mm: 3.938 mm off, 0.0 s
- far_cylinder at 0.0125 mm: 14.469 mm off, 0.0 s
- far_cylinder at 0.05 mm: 14.469 mm off, 0.0 s
- far_cylinder at 0.1 mm: 14.469 mm off, 0.0 s
- far_cylinder at 0.5 mm: 14.470 mm off, 0.0 s
- sheet_disc at 0.0125 mm: 8.906 mm off, 0.1 s
- sheet_disc at 0.05 mm: 8.906 mm off, 0.0 s
- sheet_disc at 0.1 mm: 8.907 mm off, 0.0 s
- sheet_disc at 0.5 mm: 8.920 mm off, 0.0 s
- tiny_pin at 0.0125 mm: outside the file by 0.7 mm on -Z, 1.000 mm off
- tiny_pin at 0.05 mm: face 0 empty, 1000.000 mm off
- tiny_pin at 0.1 mm: face 0 empty, 1000.000 mm off
- tiny_pin at 0.5 mm: face 0 empty, 1000.000 mm off
- torus_ring_nurbs at 0.0125 mm: 0.132 mm off, 0.1 s
- torus_ring_nurbs at 0.05 mm: 0.362 mm off, 0.0 s
- torus_ring_nurbs at 0.1 mm: 0.630 mm off, 0.0 s
- washer at 0.0125 mm: 1.000 mm off, 0.0 s
- washer at 0.05 mm: 1.000 mm off, 0.0 s
- washer at 0.1 mm: 1.000 mm off, 0.0 s
