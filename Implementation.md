# Implementation

## Cubic

A cubic is a corner piece of the pocket cube. There are 8 cubics in total in a standard pocket cube.

## Facelet

A facelet is a colored face of a cubic. There are 3 facelets on a cubic and 24 in total.

## Cube State

A cube state is a specific state of the pocket cube group that can be achieved with the specified permutations. Technically, there are `24!` (6.2e23) possible cube state combinations, but due to some invalid states, there are `8! x 3^7` (~88 million) different combinations.

## Flat Cube State

Flat cube state is a struct that is conventional for storing, visualizing, or loading a specific pocket cube state. It contains a sorted list of the facelets which can be used to reconstruct a pocket cube. Number of combinations is the same as a cube state.

In the case of visualization, the coloring of the facelets is left to the user of the library.

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

## **Ref Cube State**

A ref cube state may only represent a subgroup of a cube state, since it does not include unnecessary details like the pocket cube's rotation and direction. At the beginning of the computations, an arbitrary (ref)erence cubic is selected. This cubic may be counted as **the heart piece** of the cube as it is always kept at the same position, BDL (Back-Down-Left). Any permutation done on a ref cube state only accounts for the changes in other cubics, as the necessary permutations are designed for the BDL piece to be static.

### Initial Cube State

Initial cube state is the ref cube state where every element of `positions` and `rotations` is zero (identity). Its `ID` is also zero.

### Permutations

There are 6 modifying operations (U, D, L, R, F, and B) that can be done on a ref cube state. The initial cube state is regarded as the identity element, and does not inflict any changes.

U, R, and F operations do not change the position and rotation of the heart piece, so they are straightforward. D, L, and B operations do change the positions and/or rotation of the heart piece, so they are modified to be respectively U', R', and F'. This does not inflict any error on the calculations as it is the same thing as rotating the cube along the corresponding axes after the permutation.
