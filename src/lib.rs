use std::{cmp::Reverse, collections::HashMap, sync::Arc};

use priority_queue::PriorityQueue;
pub use puzzle_state::PuzzleState;

mod puzzle_state;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Node {
    state: Arc<PuzzleState>,
    parent: Option<Arc<Node>>,
    cost: u32,
}

impl Node {
    fn new(state: PuzzleState, parent: Option<Arc<Node>>, path_cost: u32) -> Node {
        Node {
            state: Arc::new(state),
            parent,
            cost: path_cost,
        }
    }

    fn get_steps(&self) -> Vec<PuzzleState> {
        let mut solution: Vec<PuzzleState> = Vec::new();
        let mut node = self;
        while let Some(parent) = &node.parent {
            solution.push((*node.state).clone());
            node = parent;
        }
        solution.push((*node.state).clone());
        solution.reverse();
        solution
    }

    fn has_self_in_path(&self) -> bool {
        let mut node = self;
        while let Some(parent) = &node.parent {
            if node.state == parent.state {
                return true;
            }
            node = parent;
        }
        false
    }

    fn f(&self) -> u32 {
        self.cost + self.state.heuristic()
    }
}

enum IterationResult {
    Solved(Vec<PuzzleState>),
    MinEvaluated(u32),
}

// Try to solve the puzzle using IDA* search (https://en.wikipedia.org/wiki/Iterative_deepening_A*)
pub fn solver(state: PuzzleState) -> anyhow::Result<Vec<PuzzleState>> {
    let mut max_f = 0;
    let mut min_evaluated = 1;

    while min_evaluated > max_f {
        let new_max_f = min_evaluated;
        println!("Starting iteration with maximum f of: {}", new_max_f);
        match solver_iteration(state.clone(), new_max_f) {
            IterationResult::Solved(solution) => return Ok(solution),
            IterationResult::MinEvaluated(min) => {
                min_evaluated = min;
                max_f = new_max_f;
            }
        }
    }

    Err(anyhow::anyhow!("There is no solution"))
}

fn solver_iteration(state: PuzzleState, max_f: u32) -> IterationResult {
    let mut min_evaluated = std::u32::MAX;

    let mut frontier: Vec<Arc<Node>> = Vec::new();

    let first_node = Arc::new(Node::new(state, None, 0));

    frontier.push(first_node);

    while !frontier.is_empty() {
        let node = frontier.pop().unwrap();
        if node.state.is_target() {
            return IterationResult::Solved(node.get_steps());
        }
        for action in node.state.get_actions() {
            let child = Arc::new(Node::new(action, Some(node.clone()), node.cost + 1));
            if child.f() > max_f {
                min_evaluated = min_evaluated.min(child.f());
                continue;
            }
            if !child.has_self_in_path() {
                frontier.push(child.clone());
            }
        }
    }

    IterationResult::MinEvaluated(min_evaluated)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solver_solves_in_one_step() {
        let state = PuzzleState::from_string("1 0 2 3\n4 5 6 7\n8 9 10 11\n12 13 14 15");
        let solution = solver(state).unwrap();
        assert_eq!(solution.len(), 2);
        assert_eq!(
            solution[solution.len() - 1],
            PuzzleState::from_string("0 1 2 3\n4 5 6 7\n8 9 10 11\n12 13 14 15")
        );
    }
}
