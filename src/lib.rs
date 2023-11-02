use std::{collections::HashMap, sync::Arc};

use priority_queue::PriorityQueue;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct PuzzleState {
    board: [[u8; 4]; 4],
}

impl PuzzleState {
    fn new() -> PuzzleState {
        PuzzleState { board: [[0; 4]; 4] }
    }

    fn is_goal(&self) -> bool {
        // For now check if the board is sorted
        let mut last = 0;
        for row in 0..4 {
            for col in 0..4 {
                if last > self.board[row][col] {
                    return false;
                }
                last = self.board[row][col];
            }
        }
        true
    }

    fn get_actions(&self) -> Vec<PuzzleState> {
        let mut moves: Vec<PuzzleState> = Vec::new();

        // Find the empty tile
        let mut empty_row = 0;
        let mut empty_col = 0;

        for row in 0..4 {
            for col in 0..4 {
                if self.board[row][col] == 0 {
                    empty_row = row;
                    empty_col = col;
                }
            }
        }

        // Check if we can move up
        if empty_row > 0 {
            let mut new_state = self.clone();
            new_state.board[empty_row][empty_col] = self.board[empty_row - 1][empty_col];
            new_state.board[empty_row - 1][empty_col] = 0;
            moves.push(new_state);
        }

        // Check if we can move down
        if empty_row < 3 {
            let mut new_state = self.clone();
            new_state.board[empty_row][empty_col] = self.board[empty_row + 1][empty_col];
            new_state.board[empty_row + 1][empty_col] = 0;
            moves.push(new_state);
        }

        // Check if we can move left
        if empty_col > 0 {
            let mut new_state = self.clone();
            new_state.board[empty_row][empty_col] = self.board[empty_row][empty_col - 1];
            new_state.board[empty_row][empty_col - 1] = 0;
            moves.push(new_state);
        }

        // Check if we can move right
        if empty_col < 3 {
            let mut new_state = self.clone();
            new_state.board[empty_row][empty_col] = self.board[empty_row][empty_col + 1];
            new_state.board[empty_row][empty_col + 1] = 0;
            moves.push(new_state);
        }

        moves
    }

    fn print(&self) {
        for row in 0..4 {
            for col in 0..4 {
                print!("{} ", self.board[row][col]);
            }
            println!();
        }
    }

    fn from_string(input: &str) -> PuzzleState {
        let mut state = PuzzleState::new();
        let mut row = 0;
        let mut col = 0;
        for line in input.lines() {
            for val in line.split_whitespace() {
                state.board[row][col] = val.parse().unwrap();
                col += 1;
            }
            row += 1;
            col = 0;
        }
        state
    }

    fn heuristic(&self) -> u32 {
        let mut h = 0;
        for row in 0..4 {
            for col in 0..4 {
                let val = self.board[row][col];
                if val != 0 {
                    let goal_row = (val) / 4;
                    let goal_col = (val) % 4;
                    h += (goal_row as i32 - row as i32).abs() as u32;
                    h += (goal_col as i32 - col as i32).abs() as u32;
                }
            }
        }
        h
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Node {
    state: PuzzleState,
    parent: Option<Arc<Node>>,
    cost: u32,
}

impl Node {
    fn new(state: PuzzleState, parent: Option<Arc<Node>>, path_cost: u32) -> Node {
        Node {
            state,
            parent,
            cost: path_cost,
        }
    }

    fn get_steps(&self) -> Vec<PuzzleState> {
        let mut solution: Vec<PuzzleState> = Vec::new();
        let mut node = self;
        while let Some(parent) = &node.parent {
            solution.push(node.state.clone());
            node = parent;
        }
        solution.push(node.state.clone());
        solution.reverse();
        solution
    }

    fn f(&self) -> u32 {
        self.cost + self.state.heuristic()
    }
}

fn solver(state: PuzzleState) -> anyhow::Result<Vec<PuzzleState>> {
    let first_node = Arc::new(Node::new(state, None, 0));
    let mut frontier: PriorityQueue<Arc<Node>, u32> = PriorityQueue::new();
    let mut reached: HashMap<PuzzleState, Arc<Node>> = HashMap::new();

    let f = first_node.f();
    frontier.push(first_node, f);

    while !frontier.is_empty() {
        let (new_node, _) = frontier.pop().unwrap();
        if new_node.state.is_goal() {
            return Ok(new_node.get_steps());
        }
        for action in new_node.state.get_actions() {
            let child = Arc::new(Node::new(action, Some(new_node.clone()), new_node.cost + 1));
            if !reached.contains_key(&child.state) || child.cost < reached[&child.state].cost {
                reached.insert(child.state.clone(), child.clone());
                frontier.push(child.clone(), child.cost);
            }
        }
    }

    Err(anyhow::anyhow!("No solution found"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_state_with_last_zero_is_not_goal() {
        let state = PuzzleState::from_string("1 2 3 4\n5 6 7 8\n9 10 11 12\n13 14 15 0");
        assert!(!state.is_goal());
    }

    #[test]
    fn test_state_with_leading_zero_and_sorted_is_goal() {
        let state = PuzzleState::from_string("0 1 2 3\n4 5 6 7\n8 9 10 11\n12 13 14 15");
        assert!(state.is_goal());
    }

    #[test]
    fn test_state_with_leading_zero_and_unsorted_is_not_goal() {
        let state = PuzzleState::from_string("0 1 2 3\n4 5 6 7\n8 9 10 11\n12 13 15 14");
        assert!(!state.is_goal());
    }

    #[test]
    fn test_goal_state_has_heuristic_of_zero() {
        let state = PuzzleState::from_string("0 1 2 3\n4 5 6 7\n8 9 10 11\n12 13 14 15");
        assert_eq!(state.heuristic(), 0);
    }

    #[test]
    fn test_state_one_step_away_from_goal_has_heuristic_of_one() {
        let state = PuzzleState::from_string("1 0 2 3\n4 5 6 7\n8 9 10 11\n12 13 15 14");
        assert_eq!(state.heuristic(), 1);
    }
}
