use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PuzzleState {
    board: [[u8; 4]; 4],
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

#[macro_export]
macro_rules! state {
    ($($($x:expr),*);*) => {
        {
            let matrix =  [
                $([$($x),*]),*
            ];
            PuzzleState::new(matrix)
        }
    };
}

impl PuzzleState {
    pub fn new(board: [[u8; 4]; 4]) -> PuzzleState {
        PuzzleState { board }
    }

    // TODO: More like compare with the goal state
    pub fn is_goal(&self) -> bool {
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

    pub fn get_actions(&self) -> Vec<PuzzleState> {
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

    pub fn as_string(&self) -> String {
        let mut res = "┌---------------┐\n".to_string();
        for row in 0..4 {
            res.push_str(
                format!(
                    "|{:^3}|{:^3}|{:^3}|{:^3}|\n",
                    self.board[row][0], self.board[row][1], self.board[row][2], self.board[row][3]
                )
                .as_str(),
            );
            if row < 3 {
                res += "├---+---+---+---┤\n";
            }
            // res += "├--+--+--+--┤\n";
        }
        res.push_str("└---------------┘\n");
        res.replace(" 0 ", " 🗆 ")
        // res
    }

    pub fn from_string(input: &str) -> PuzzleState {
        let mut state = PuzzleState::new([[0; 4]; 4]);
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
        for row in 0..4 {
            for col in 0..4 {
                let val = self.board[row][col];
                if val != 0 {
                    let goal_row = (val) / 4;
                    let goal_col = (val) % 4;
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
    fn test_state_macro() {
        let state = state!(
            1, 2, 3, 4;
            5, 6, 7, 8;
            9, 10, 11, 12;
            13, 14, 15, 0
        );
        assert_eq!(state.board[0][0], 1);
    }

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
        let state = PuzzleState::from_string("1 0 2 3\n4 5 6 7\n8 9 10 11\n12 13 14 15");
        assert_eq!(state.heuristic(), 1);
    }

    #[test]
    fn test_state_with_zero_in_the_first_row_has_three_options() {
        let state = PuzzleState::from_string("1 0 2 3\n4 5 6 7\n8 9 10 11\n12 13 14 15");
        let actions = state.get_actions();
        assert_eq!(actions.len(), 3);
    }

    #[test]
    fn test_state_with_zero_in_the_middle_has_four_options() {
        let state = PuzzleState::from_string("1 2 3 4\n5 0 6 7\n8 9 10 11\n12 13 14 15");
        let actions = state.get_actions();
        assert_eq!(actions.len(), 4);
    }

    #[test]
    fn test_state_with_zero_in_a_corner_has_two_options() {
        let state = PuzzleState::from_string("0 1 2 3\n4 5 6 7\n8 9 10 11\n12 13 14 15");
        let actions = state.get_actions();
        assert_eq!(actions.len(), 2);
    }

    #[test]
    fn test_state_with_zero_in_a_corner_has_specific_options() {
        let state = PuzzleState::from_string("0 1 2 3\n4 5 6 7\n8 9 10 11\n12 13 14 15");
        let actions = state.get_actions();
        assert!(actions.contains(&PuzzleState::from_string(
            "1 0 2 3\n4 5 6 7\n8 9 10 11\n12 13 14 15"
        )));
        assert!(actions.contains(&PuzzleState::from_string(
            "4 1 2 3\n0 5 6 7\n8 9 10 11\n12 13 14 15"
        )));
    }

    #[test]
    fn test_state_with_zero_in_the_middle_has_specific_options() {
        let state = PuzzleState::from_string("1 2 3 4\n5 0 6 7\n8 9 10 11\n12 13 14 15");
        let actions = state.get_actions();
        assert!(actions.contains(&PuzzleState::from_string(
            "1 0 3 4\n5 2 6 7\n8 9 10 11\n12 13 14 15"
        )));
        assert!(actions.contains(&PuzzleState::from_string(
            "1 2 3 4\n5 6 0 7\n8 9 10 11\n12 13 14 15"
        )));
        assert!(actions.contains(&PuzzleState::from_string(
            "1 2 3 4\n5 9 6 7\n8 0 10 11\n12 13 14 15"
        )));
        assert!(actions.contains(&PuzzleState::from_string(
            "1 2 3 4\n0 5 6 7\n8 9 10 11\n12 13 14 15"
        )));
    }

    #[test]
    fn test_two_states_with_the_same_boards_are_equal() {
        let state1 = PuzzleState::from_string("1 2 3 4\n5 0 6 7\n8 9 10 11\n12 13 14 15");
        let state2 = PuzzleState::from_string("1 2 3 4\n5 0 6 7\n8 9 10 11\n12 13 14 15");
        assert_eq!(state1, state2);
    }

    #[test]
    fn test_hashmap_shows_state_as_present() {
        let state1 = PuzzleState::from_string("1 2 3 4\n5 0 6 7\n8 9 10 11\n12 13 14 15");
        let state2 = PuzzleState::from_string("1 2 3 4\n5 0 6 7\n8 9 10 11\n12 13 14 15");
        let mut map = std::collections::HashMap::new();
        map.insert(state1, 1);
        assert!(map.contains_key(&state2));
    }
}
