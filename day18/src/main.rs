use std::{env, fs};

struct TokensFromEnd<'a> {
    split: Vec<&'a str>,
}

#[derive(Debug)]
enum Token {
    Number(usize),
    Operator(char),
    OpenParen,
    CloseParen,
}

use Token::*;

impl<'a> From<&'a str> for TokensFromEnd<'a> {
    fn from(expr: &'a str) -> Self {
        Self { split: expr.split(' ').collect() }
    }
}

impl Iterator for TokensFromEnd<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(token) = self.split.pop() {
            if let Some(next_token) = token.strip_suffix(')') {
                self.split.push(next_token);
                Some(CloseParen)
            } else if token == "(" {
                Some(OpenParen)
            } else if let Some(next_token) = token.strip_prefix('(') {
                self.split.push("(");
                self.split.push(next_token);
                self.next()
            } else if let Ok(num) = token.parse() {
                Some(Number(num))
            } else if let Some(op) = token.chars().nth(0) {
                Some(Operator(op))
            } else {
                self.next()
            }
        } else {
            None
        }
    }
}

fn perform_op(op: &char, n1: usize, n2: usize) -> usize {
    match op {
        '+' => n1 + n2,
        '*' => n1 * n2,
        _ => panic!("Operation {} not supported", op),
    }
}

/// Process tokens without precedence
fn calculate(token_stack: &mut Vec<Token>) -> usize {
    let mut acc = 0;
    let mut op = '+';
    while let Some(token) = token_stack.pop() {
        match token {
            Number(num) => acc = perform_op(&op, acc, num),
            Operator(new_op) => op = new_op,
            OpenParen => acc = perform_op(&op, acc, calculate(token_stack)),
            CloseParen => break,
        };
    }
    acc
}

fn part1(mut token_stacks: Vec<Vec<Token>>) -> usize {
    token_stacks
        .iter_mut()
        .map(|token_stack| calculate(token_stack))
        .sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");
    let token_stacks: Vec<Vec<Token>> = contents
        .lines()
        .map(|line| {
            let token_iter: TokensFromEnd = line.into();
            token_iter.collect()
        })
        .collect();
    println!("Part 1: {}", part1(token_stacks));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_examples() {
        let equations = vec![
            "1 + (2 * 3) + (4 * (5 + 6))",
            "2 * 3 + (4 * 5)",
            "5 + (8 * 3 + 9 + 3 * 4 * 3)",
            "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))",
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
        ];

        let answers = vec![
            51, 26, 437, 12240, 13632
        ];

        let token_stacks: Vec<Vec<Token>> = equations
            .into_iter()
            .map(|eq| {
                let token_iter: TokensFromEnd = eq.into();
                token_iter.collect()
            })
            .collect();

        assert_eq!(part1(token_stacks), answers.iter().sum());
    }
}
