use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PuzzleState {
    board: [[u8; 3]; 3],
}

// Maybe this is a better way to do it?
/*
macro_rules! state {
    ($($x:expr),*) => {
        {
        let matrix = [
            [$($x),*],
            [$($x),*],
            [$($x),*],
            [$($x),*],
        ];
        PuzzleState::new(matrix)
        }
    };
}
 */

// #[macro_export]
// macro_rules! state {
//     ($($($x:expr),*);*) => {
//         {
//             let matrix =  [
//                 $([$($x),*]),*
//             ];
//             PuzzleState::new(matrix)
//         }
//     };
// }

impl PuzzleState {
    pub fn new(board: [[u8; 3]; 3]) -> PuzzleState {
        PuzzleState { board }
    }

    // TODO: More like compare with the goal state
    pub fn is_goal(&self) -> bool {
        // For now check if the board is sorted
        let mut last = 0;
        for row in 0..3 {
            for col in 0..3 {
                if last > self.board[row][col] {
                    return false;
                }
                last = self.board[row][col];
            }
        }
        true
    }

    pub fn get_actions(&self) -> Vec<PuzzleState> {
        let mut moves: Vec<PuzzleState> = Vec::new();

        // Find the empty tile
        let mut empty_row = 0;
        let mut empty_col = 0;

        for row in 0..3 {
            for col in 0..3 {
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
        if empty_row < 2 {
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
        if empty_col < 2 {
            let mut new_state = self.clone();
            new_state.board[empty_row][empty_col] = self.board[empty_row][empty_col + 1];
            new_state.board[empty_row][empty_col + 1] = 0;
            moves.push(new_state);
        }

        moves
    }

    pub fn as_string(&self) -> String {
        let mut res = "┌-----------┐\n".to_string();
        for row in 0..3 {
            res.push_str(
                format!(
                    "|{:^3}|{:^3}|{:^3}|\n",
                    self.board[row][0], self.board[row][1], self.board[row][2]
                )
                .as_str(),
            );
            if row < 2 {
                res += "├---+---+---┤\n";
            }
            // res += "├--+--+--+--┤\n";
        }
        res.push_str("└-----------┘\n");
        res.replace(" 0 ", " 🗆 ")
        // res
    }

    pub fn from_string(input: &str) -> PuzzleState {
        let mut state = PuzzleState::new([[0; 3]; 3]);
        let mut col = 0;
        for (row, line) in input.lines().enumerate() {
            for val in line.split_whitespace() {
                state.board[row][col] = val.parse().unwrap();
                col += 1;
            }
            col = 0;
        }
        state
    }

    // The heuristic is the sum of the Manhattan distances of each tile to its goal position
    pub fn heuristic(&self) -> u32 {
        let mut h = 0;
        for row in 0..3 {
            for col in 0..3 {
                let val = self.board[row][col];
                if val != 0 {
                    let goal_row = (val) / 3;
                    let goal_col = (val) % 3;
                    h += (goal_row as i32 - row as i32).unsigned_abs();
                    h += (goal_col as i32 - col as i32).unsigned_abs();
                }
            }
        }
        h
    }
}

impl Display for PuzzleState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle_state::PuzzleState;

    #[test]
    fn test_state_with_last_zero_is_not_goal() {
        let state = PuzzleState::from_string("1 2 3\n4 5 6\n7 8 0");
        assert!(!state.is_goal());
    }

    #[test]
    fn test_state_with_leading_zero_and_sorted_is_goal() {
        let state = PuzzleState::from_string("0 1 2\n3 4 5\n6 7 8");
        assert!(state.is_goal());
    }

    #[test]
    fn test_state_with_leading_zero_and_unsorted_is_not_goal() {
        let state = PuzzleState::from_string("0 1 2\n3 4 5\n6 8 7");
        assert!(!state.is_goal());
    }

    #[test]
    fn test_goal_state_has_heuristic_of_zero() {
        let state = PuzzleState::from_string("0 1 2\n3 4 5\n6 7 8");
        assert_eq!(state.heuristic(), 0);
    }

    #[test]
    fn test_state_one_step_away_from_goal_has_heuristic_of_one() {
        let state = PuzzleState::from_string("1 0 2\n3 4 5\n6 7 8");
        assert_eq!(state.heuristic(), 1);
    }

    #[test]
    fn test_state_with_zero_in_the_first_row_has_three_options() {
        let state = PuzzleState::from_string("1 0 2\n3 4 5\n6 7 8");
        let actions = state.get_actions();
        assert_eq!(actions.len(), 3);
    }

    #[test]
    fn test_state_with_zero_in_the_middle_has_four_options() {
        let state = PuzzleState::from_string("1 2 3\n4 0 5\n6 7 8");
        let actions = state.get_actions();
        assert_eq!(actions.len(), 4);
    }

    #[test]
    fn test_state_with_zero_in_top_left_corner_has_two_options() {
        let state = PuzzleState::from_string("0 1 2\n3 4 5\n6 7 8");
        let actions = state.get_actions();
        assert_eq!(actions.len(), 2);
    }

    #[test]
    fn test_state_with_zero_in_top_right_corner_has_two_options() {
        let state = PuzzleState::from_string("1 2 0\n3 4 5\n6 7 8");
        let actions = state.get_actions();
        assert_eq!(actions.len(), 2);
    }

    #[test]
    fn test_state_with_zero_in_bottom_right_corner_has_two_options() {
        let state = PuzzleState::from_string("1 2 3\n3 4 5\n6 7 0");
        let actions = state.get_actions();
        assert_eq!(actions.len(), 2);
    }

    #[test]
    fn test_state_with_zero_in_bottom_left_corner_has_two_options() {
        let state = PuzzleState::from_string("1 2 3\n3 4 5\n0 7 8");
        let actions = state.get_actions();
        assert_eq!(actions.len(), 2);
    }

    #[test]
    fn test_state_with_zero_in_a_corner_has_specific_options() {
        let state = PuzzleState::from_string("0 1 2\n3 4 5\n6 7 8");
        let actions = state.get_actions();
        assert!(actions.contains(&PuzzleState::from_string("1 0 2\n3 4 5\n6 7 8")));
        assert!(actions.contains(&PuzzleState::from_string("3 1 2\n0 4 5\n6 7 8")));
    }

    #[test]
    fn test_state_with_zero_in_the_middle_has_specific_options() {
        let state = PuzzleState::from_string("1 2 3\n4 0 5\n6 7 8");
        let actions = state.get_actions();
        assert!(actions.contains(&PuzzleState::from_string("1 0 3\n4 2 5\n6 7 8")));
        assert!(actions.contains(&PuzzleState::from_string("1 2 3\n4 7 5\n6 0 8")));
        assert!(actions.contains(&PuzzleState::from_string("1 2 3\n0 4 5\n6 7 8")));
        assert!(actions.contains(&PuzzleState::from_string("1 2 3\n4 5 0\n6 7 8")));
    }

    #[test]
    fn test_two_states_with_the_same_boards_are_equal() {
        let state1 = PuzzleState::from_string("1 2 3\n4 5 6\n7 8 0");
        let state2 = PuzzleState::from_string("1 2 3\n4 5 6\n7 8 0");
        assert_eq!(state1, state2);
    }

    #[test]
    fn test_hashmap_shows_state_as_present() {
        let state1 = PuzzleState::from_string("1 2 3\n4 5 6\n7 8 0");
        let state2 = PuzzleState::from_string("1 2 3\n4 5 6\n7 8 0");
        let mut map = std::collections::HashMap::new();
        map.insert(state1, 1);
        assert!(map.contains_key(&state2));
    }
}
