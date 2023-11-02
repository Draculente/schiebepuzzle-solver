## Schiebepuzzle Solver

This is a solver for the [Schiebepuzzle](https://en.wikipedia.org/wiki/Sliding_puzzle) game. It uses the [A* algorithm](https://en.wikipedia.org/wiki/A*_search_algorithm) to find the shortest path to the solution.

The only goal of this project is to fiddle around with the A* algorithm.

As there not always exists a solution, the solver may run forever (or until the memory is full).

In the current implementation the only possible goal state is the following (the empty field is in the top left corner):

```
┌---------------┐
| 🗆 | 1 | 2 | 3 |
├---+---+---+---┤
| 4 | 5 | 6 | 7 |
├---+---+---+---┤
| 8 | 9 |10 |11 |
├---+---+---+---┤
|12 |13 |14 |15 |
└---------------┘
```
