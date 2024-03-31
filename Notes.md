# Notes

This document highlights and explains the choices made by the designers of the project.

## Heart Piece

The right, top, and front sides of the pocket cube were subjectively chosen by the designers to be the ones that are permutated. So, only the back-down-left piece is left to be the heart piece.

## Permutation tables

There were two permutation table designs at hand: one where every element of the table has 6 elements (the corresponding ID's of the 6 moves), and one where every element has only the 3 basic moves' corresponding ID's. The second one was chosen to generate a small and minimal permutation table: MiniTable. This table is specifically designed for the calculations required by the paper. Each ID of the MiniTable could be converted into a 24-bit unsigned integer to optimize memory usage by 25%, but this would bring a CPU overhead that would not outperform the current algorithm since the bytes would not be aligned by four. An additinoal blank integer is added to make the entries 16-byte aligned. This makes the table's total size 56 MiB.

By using the data already present in MiniTable, a permutation table that can be used to efficiently find optimal solutions emerges: FullTable. This table could have also used 24-bit unsigned integers, but the same overhead holds. Additionally, a heuristic value that represents the shortest distance (number of moves) from the solved state is stored. In total, this makes 7 32-bit unsigned integers. For minimal CPU overhead, a blank integer is added to make the table 32-byte aligned. This makes the table's total size 112 MiB.
