// TODO: FINISH THIS
use common;

fn main() {
    let input = common::read_input_file("input.txt");

    let part1 = part1(&input);
    println!("Hello, world!");
}

fn part1(input: &String) -> u32 {
    let sum: u32 = 0;

    let mut x: i32 = 0;
    let mut y: i32 = 0;

    let parts = input
        .chars()
        .map(|c| {
            x += 1;
            if c == '\n' {
                y += 1;
                x = -1;
            }

            (x, y, c)
        })
        .filter(|(x, _, c)| x > &-1 && c != &'.')
        .filter(|(x, y, c)| {
            let mut is_adjacent = false;

            let lines = input.lines().collect::<Vec<&str>>();

            if c.is_numeric() {
                let (num) = input.split_once(|c: char| !c.is_numeric());
            }

            is_adjacent
        })
        .filter(|(_, _, c)| c.is_numeric())
        .collect::<Vec<(i32, i32, char)>>();

    println!("{:?}", parts);

    sum
}
