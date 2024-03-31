# Implementation

## Cubic

A cubic is a corner piece of the pocket cube. There are 8 cubics in total in a standard pocket cube. They are named as:

```
FUR: front -  up  - right
FUL: front -  up  - left
FDR: front - down - right
FDL: front - down - left
BUR:  back -  up  - right
BUL:  back -  up  - left
BDR:  back - down - right
BDL:  back - down - left
```

## Facelet

A facelet is a colored face of a cubic. There are 3 facelets on a cubic and 24 in total.

## Permutation

A permutation is a move/a set of moves that can be done on a pocket cube. For example, turning the top face of the cube clockwise. In a standard pocket cube, there are 12 different moves: R, L, U, D, F, B, and their respective inverses. However, turning a side of the pocket cube clockwise is the same as turning the opposite side clockwise. This does not inflict any error on the calculations as it is the same thing as rotating the cube along the corresponding axes after the permutation. So, there are 6 possible moves: R, U, F, R', U', and F'. These moves are used to derive other permutations.

Basic moves are R, U, F, and their inverses can be achieved by repeating them three times.

## Cube State

A cube state is a specific state of the pocket cube group that can be achieved with the specified permutations. Technically, there are `8! x 3^8` (~265 million) cube state combinations. However, according to Theorem 3.5, there are `8! x 3^7` (~88 million) possible different combinations of a pocket cube. If a cubic piece is anchored (its position and rotation does not change throughout permutations), the total number of states is reduced to `7! x 3^6` (~3.7 million). This anchored cubic piece is called the **heart piece**, and it is selected to be the BDL piece.

Every cube state is technically a permutation that can be done to transform another cube state, but only the 6 basic moves are done on the cube states in the model.

## Facelets

`facelets` is a facelet array that is conventional for visualizing a cube state. It contains a sorted list of the facelets which can be used to reconstruct a physical pocket cube. The coloring of the facelets is left to the programmer. Each index of the `facelets` array represents the corresponding facelet as below:

```
.                            ┌─────────┐
.                            │  0   1  │
.                            │    U    │
(U)p      0..4               │  2   3  │
(L)eft    4..8     ┌─────────┼─────────┼─────────┬─────────┐
(F)ront   8..12    │ 4     5 │ 8     9 │ 12   13 │ 16   17 │
(R)ight  12..16    │    L    │    F    │    R    │    B    │
(B)ack   16..20    │ 6     7 │ 10   11 │ 14   15 │ 18   19 │
(D)own   20..24    └─────────┼─────────┼─────────┴─────────┘
.                            │ 20   21 │
.                            │    D    │
.                            │ 22   23 │
.                            └─────────┘
```

## **Pocket Cube** (`group::PocketCube`)

The `PocketCube` struct represents a cube state using minimal data. Any permutation done on a `PocketCube` state only accounts for the changes in the non-heart pieces, as the basic moves are designed for the BDL piece to be static.

### Default Pocket Cube

Default `PocketCube` is the cube state where every element of `PosId` and `RotId` is zero (identity). Its `PermID` is also zero.

## `PermId`

`PermId` struct is a number ranging from 0 to 3674159 (excluded)
`PosId` and `RotId`

Pos

## `PermId` is a number ranging from 0 to 3674159 (excluded)
