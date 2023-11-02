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

    fn f(&self) -> u32 {
        self.cost + self.state.heuristic()
    }
}

// Try to solve the puzzle using A* search
pub fn solver(state: PuzzleState) -> anyhow::Result<Vec<PuzzleState>> {
    let first_node = Arc::new(Node::new(state, None, 0));
    let mut frontier: PriorityQueue<Arc<Node>, Reverse<u32>> = PriorityQueue::new();
    let mut reached: HashMap<Arc<PuzzleState>, Arc<Node>> = HashMap::new();

    let f = first_node.f();
    frontier.push(first_node, Reverse(f));

    while !frontier.is_empty() {
        let (node, _) = frontier.pop().ok_or(anyhow::anyhow!("Frontier is empty"))?;
        if node.state.is_goal() {
            return Ok(node.get_steps());
        }
        for action in node.state.get_actions() {
            let child = Arc::new(Node::new(action, Some(node.clone()), node.cost + 1));
            if !reached.contains_key(&child.state) {
                reached.insert(child.state.clone(), child.clone());
                frontier.push(child.clone(), Reverse(child.f()));
            } else if child.cost < reached[&child.state].cost {
                // Remove the old (worse) node
                frontier.remove(&reached[&child.state]);
                reached.insert(child.state.clone(), child.clone());
                frontier.push(child.clone(), Reverse(child.f()));
            }
        }
    }

    Err(anyhow::anyhow!("No solution found"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solver_solves_in_one_step() {
        let state = PuzzleState::from_string("1 0 2\n3 4 5\n6 7 8");
        let solution = solver(state).unwrap();
        assert_eq!(solution.len(), 2);
        assert_eq!(
            solution[solution.len() - 1],
            PuzzleState::from_string("0 1 2\n3 4 5\n6 7 8")
        );
    }
}
