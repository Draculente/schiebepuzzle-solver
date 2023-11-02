## Schiebepuzzle Solver

This is a solver for the [Schiebepuzzle](https://en.wikipedia.org/wiki/Sliding_puzzle) game. It uses the [A* algorithm](https://en.wikipedia.org/wiki/A*_search_algorithm) to find the shortest amount of steps to the solution.

The only goal of this project is to fiddle around with the A* algorithm.

In the current implementation the only possible target state is the following (the empty field is in the top left corner):

```
┌-----------┐
| 🗆 | 1 | 2 |
├---+---+---┤
| 3 | 4 | 5 |
├---+---+---┤
| 6 | 7 | 8 |
└-----------┘
```

The solver only works for 3x3 puzzles, as this makes for a total of `9! = 362880` possible states. This is still manageable.  
For larger puzzles the number of possible states grows exponentially and the solver would take too long to find a solution (or come to the conclusion that there is none).  
I implemented a [4x4 solver](https://github.com/Draculente/schiebepuzzle-solver/tree/four-by-four), but it runs out of memory before it can conclude if there is a solution (even though it _can_ find a solution in a reasonable amount of time, if can't definitely say if there is none).