# Rust WHUTS
> Answering the question "Which Hypercube Unfoldings Tile Space?" posed on https://whuts.org/

This solution uses [Knuth's Algorithm X](https://en.wikipedia.org/wiki/Knuth%27s_Algorithm_X) with the dancing links technique (referred to as DLX). The actual DLX implementation used is in the following project: https://github.com/edmccard/dlx-rs.

### Output
The program first prints the coordinates of each unfolding. If it finds a suitable tiling for the unfolding, it will also print the following:
- The dimensions of the smallest wrapping space (i.e. [hypertorus or 3-torus](https://en.wikipedia.org/wiki/Torus#n-dimensional_torus)) that can be infinitely stacked to tile the entire space.
- The coordinates of each instance of the unfolding contained within the hypertorus.

For example, this is the output for the first four unfoldings:

```
====================================[ UNFOLDING #1 ]====================================
Unfolding coords:
[[0, 0, 0], [1, 0, 0], [1, 0, 1], [1, 1, 1], [1, 1, 2], [2, 1, 2], [2, 1, 3], [2, 2, 3]]

FOUND TILING!
Dimensions of the smallest wrapping space (i.e. hypertorus): [2, 2, 2]
Tiling (each line is a single instance of the unfolding):
[[0, 0, 0], [0, 0, 1], [0, 1, 0], [0, 1, 1], [1, 0, 0], [1, 0, 1], [1, 1, 0], [1, 1, 1]]

====================================[ UNFOLDING #2 ]====================================
Unfolding coords:
[[0, 1, 3], [0, 2, 3], [1, 0, 0], [1, 0, 1], [1, 1, 1], [1, 1, 2], [1, 1, 3], [2, 1, 2]]

FOUND TILING!
Dimensions of the smallest wrapping space (i.e. hypertorus): [2, 2, 4]
Tiling (each line is a single instance of the unfolding):
[[0, 0, 0], [0, 1, 0], [0, 1, 1], [1, 0, 2], [1, 0, 3], [1, 1, 0], [1, 1, 1], [1, 1, 2]]
[[0, 0, 1], [0, 0, 2], [0, 0, 3], [0, 1, 2], [0, 1, 3], [1, 0, 0], [1, 0, 1], [1, 1, 3]]

====================================[ UNFOLDING #3 ]====================================
Unfolding coords:
[[0, 0, 1], [1, 0, 1], [1, 1, 0], [1, 1, 1], [1, 1, 2], [2, 1, 2], [2, 2, 2], [2, 2, 3]]

FOUND TILING!
Dimensions of the smallest wrapping space (i.e. hypertorus): [2, 2, 4]
Tiling (each line is a single instance of the unfolding):
[[0, 0, 0], [0, 0, 1], [0, 0, 2], [0, 1, 2], [1, 0, 1], [1, 1, 1], [1, 1, 2], [1, 1, 3]]
[[0, 0, 3], [0, 1, 0], [0, 1, 1], [0, 1, 3], [1, 0, 0], [1, 0, 2], [1, 0, 3], [1, 1, 0]]

====================================[ UNFOLDING #4 ]====================================
Unfolding coords:
[[0, 3, 2], [1, 0, 1], [1, 1, 1], [1, 2, 0], [1, 2, 1], [1, 2, 2], [1, 3, 2], [2, 2, 2]]

FOUND TILING!
Dimensions of the smallest wrapping space (i.e. hypertorus): [2, 4, 4]
Tiling (each line is a single instance of the unfolding):
[[0, 0, 0], [0, 1, 0], [1, 0, 0], [1, 1, 0], [1, 1, 1], [1, 1, 2], [1, 2, 1], [1, 3, 1]]
[[0, 0, 1], [0, 0, 2], [0, 2, 1], [0, 3, 0], [0, 3, 1], [0, 3, 3], [1, 0, 1], [1, 0, 2]]
[[0, 0, 3], [0, 1, 1], [0, 1, 2], [0, 1, 3], [0, 2, 0], [0, 2, 3], [1, 2, 0], [1, 2, 3]]
[[0, 2, 2], [0, 3, 2], [1, 0, 3], [1, 1, 3], [1, 2, 2], [1, 3, 0], [1, 3, 2], [1, 3, 3]]

```