use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Copy, Clone, Debug)]
struct Instruction<'a> {
    argument: isize,
    operation: &'a str,
}

#[derive(Debug)]
enum ProgramError {
    InfiniteLoop(isize, HashSet<usize>),
    InvalidLine,
}

type Program<'a> = Vec<Instruction<'a>>;

struct ProgramState<'a> {
    accumulator: isize,
    next_line: usize,
    program: &'a Program<'a>,
}

impl<'a> ProgramState<'a> {
    /// Create a new "run" of a given Program
    fn new(program: &'a Program<'a>) -> ProgramState<'a> {
        ProgramState {
            accumulator: 0,
            next_line: 0,
            program,
        }
    }

    /// Steps forward in the program, returning true if next_line pointed to an instruction
    fn step(&mut self) -> bool {
        if let Some(instruction) = self.program.get(self.next_line) {
            match instruction.operation {
                "acc" => {
                    self.accumulator += instruction.argument;
                    self.next_line += 1;
                }
                "jmp" => {
                    self.next_line = (self.next_line as isize + instruction.argument) as usize;
                }
                "nop" => { self.next_line += 1; }
                _ => panic!("Unrecognized instruction: {}", instruction.operation)
            };
            true
        } else { false }
    }
}

/// Parse each item from a string slice iterator as an Instruction
fn parse_program<'a>(instructions: impl Iterator<Item=&'a str>) -> Vec<Instruction<'a>> {
    instructions.map(|instruction| {
        let mut split = instruction.split(' ');
        let operation = split.next().unwrap();
        let argument_str = split.next().unwrap();
        Instruction {
            argument: argument_str.parse().unwrap(),
            operation,
        }
    }).collect()
}

/// Runs a program, returning an error and the lines run if a loop is detected
fn run_program(program: &Program) -> Result<isize, ProgramError> {
    let mut state = ProgramState::new(program);
    let mut lines_visited: HashSet<usize> = HashSet::new();

    loop {
        let visited = !lines_visited.insert(state.next_line);
        if visited {
            break Err(ProgramError::InfiniteLoop(state.accumulator, lines_visited));
        }
        // End of program?
        if state.next_line == program.len() { break Ok(state.accumulator); }
        if !state.step() {
            break Err(ProgramError::InvalidLine);
        }
    }
}

fn part1(program: &Program) -> (isize, HashSet<usize>) {
    match run_program(program) {
        Err(ProgramError::InfiniteLoop(acc, lines)) => (acc, lines),
        Err(e) => panic!("Unexpected program error {:?}", e),
        Ok(_) => panic!("Should have found infinite loop"),
    }
}

fn part2(program: &Program, candidates: HashSet<usize>) -> isize {
    candidates.iter().find_map(|candidate| {
        let candidate_instruction = program.get(*candidate).unwrap();
        if candidate_instruction.operation == "acc" {
            return None
        } else {
            let mut program_fix = program.clone();
            let mut instruction = program_fix.get_mut(*candidate).unwrap();
            match instruction.operation {
                "jmp" => { instruction.operation = "nop"; }
                "nop" => { instruction.operation = "jmp"; }
                _ => panic!("Invalid candidate {:?}", instruction)
            };
            run_program(&program_fix).ok()
        }
    }).unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");
    let program = parse_program(contents.lines());
    let (result, lines) = part1(&program);
    println!("Part 1: {}", result);
    println!("Part 2: {}", part2(&program, lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("sample");

    #[test]
    fn example() {
        let program = parse_program(SAMPLE.lines());
        let (result, lines) = part1(&program);
        assert_eq!(result, 5);
        assert_eq!(part2(&program, lines), 8);
    }
}
