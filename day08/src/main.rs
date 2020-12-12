use std::collections::HashSet;
use std::env;
use std::fs;

struct Instruction<'a> {
    argument: isize,
    operation: &'a str,
}

type Program<'a> = Vec<Instruction<'a>>;

struct ProgramState<'a> {
    accumulator: isize,
    next_line: usize,
    program: &'a Program<'a>,
}

impl<'a> ProgramState<'a> {
    fn new(program: &'a Program<'a>) -> ProgramState<'a> {
        ProgramState {
            accumulator: 0,
            next_line: 0,
            program,
        }
    }

    fn step(&mut self) {
        let instruction = self.program.get(self.next_line).unwrap();
        match instruction.operation {
            "acc" => {
                self.accumulator += instruction.argument;
                self.next_line += 1;
            }
            "jmp" => {
                self.next_line = (self.next_line as isize + instruction.argument) as usize;
            }
            "nop" => { self.next_line += 1 }
            _ => panic!("Unrecognized instruction: {}", instruction.operation)
        }
    }
}

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

fn part1(program: &Program) -> isize {
    let mut state = ProgramState::new(program);
    let mut lines_visited: HashSet<usize> = HashSet::new();

    loop {
        let visited = !lines_visited.insert(state.next_line);
        if visited { break state.accumulator; }
        state.step();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");
    let program = parse_program(contents.lines());
    println!("Part 1: {}", part1(&program));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("sample");

    #[test]
    fn part1_example() {
        let program = parse_program(SAMPLE.lines());
        assert_eq!(part1(&program), 5);
    }
}
