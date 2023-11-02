use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;

use schiebepuzzle_solver::{self, PuzzleState};

fn main() {
    let start_state = match read_from_command_line() {
        Ok(state) => state,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    let _ = clearscreen::clear();

    print!(
        "Starting solution schiebepuzzle_solver with state:\n{}",
        start_state
    );

    println!("Solving...(this may take a long while and a lot of memory)");

    let solution = schiebepuzzle_solver::solver(start_state);

    match solution {
        Ok(solution) => {
            if let Err(e) = handle_solution(&solution) {
                println!("Error: {}", e);
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

fn read_from_command_line() -> anyhow::Result<PuzzleState> {
    println!("Enter the puzzle state line by line, separated by spaces. Use 0 for the empty tile.");

    let mut input = String::new();

    let mut lines = 0;
    while lines < 4 {
        let mut line: String = promptly::prompt("Enter line")?;
        if let Err(e) = validate_line(&line) {
            println!("Error: {}", e);
            continue;
        }
        line.push('\n');
        input.push_str(&line);
        lines += 1;
    }

    validate_input(&input)?;

    let state = PuzzleState::from_string(&input);
    Ok(state)
}

fn validate_line(line: &String) -> anyhow::Result<&String> {
    let mut numbers: Vec<u32> = Vec::new();
    for number in line.split_whitespace() {
        numbers.push(number.parse()?);
    }

    if numbers.len() != 4 {
        Err(anyhow::anyhow!("Line must contain 4 numbers"))?;
    }

    for number in numbers {
        if number > 15 {
            Err(anyhow::anyhow!("Number must be between 0 and 15"))?;
        }
    }

    Ok(line)
}

fn validate_input(input: &String) -> anyhow::Result<&String> {
    let lines = input.lines();
    let mut numbers: Vec<u32> = Vec::new();

    if lines.count() != 4 {
        Err(anyhow::anyhow!("Input must contain 4 lines"))?;
    }

    for line in input.lines() {
        for number in line.split_whitespace() {
            numbers.push(number.parse()?);
        }
    }

    if numbers.len() != 16 {
        Err(anyhow::anyhow!("Input must contain 16 numbers"))?;
    }

    for number in &numbers {
        if number > &15 {
            Err(anyhow::anyhow!("Number must be between 0 and 15"))?;
        }
    }

    if numbers.into_iter().unique().count() != 16 {
        Err(anyhow::anyhow!("Numbers must be unique"))?;
    }

    Ok(input)
}

fn handle_solution(solution: &Vec<PuzzleState>) -> anyhow::Result<()> {
    let _ = clearscreen::clear();
    println!("Solution found for \n{}!", solution[0]);
    println!("Solution length: {}", solution.len());
    let solution_file = "solution.txt";

    let mut file = File::create(solution_file).unwrap();
    for step in solution {
        file.write_all(format!("{}\n", step).as_bytes())?;
    }
    println!("Solution written to {}", solution_file);

    solution_step_by_step(solution)?;

    println!("Done!");

    Ok(())
}

fn solution_step_by_step(solution: &Vec<PuzzleState>) -> anyhow::Result<()> {
    for step in solution {
        let cont = promptly::prompt_default("Do you want to continue?", true)?;
        if !cont {
            Err(anyhow::anyhow!("Aborted by user"))?;
        }
        println!("{}", step);
    }
    Ok(())
}
