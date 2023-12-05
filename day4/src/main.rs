use std::collections::HashMap;

use common;

struct Card {
    winning_numbers: Vec<u16>,
    numbers: Vec<u16>,
}

fn main() {
    let input = common::read_input_file("input.txt");
    let cards = get_cards(&input);

    let part1 = part1(&cards);
    println!("Part 1 Result: {part1}");

    let instances = get_card_instances(&cards);

    let part2 = part2(&instances);
    println!("Part 2 Result: {part2}");
}

fn part1(cards: &Vec<Card>) -> u16 {
    cards.iter().fold(0, |acc, card| {
        acc + card.numbers.iter().fold(0, |acc, num| {
            if acc == 0 {
                if card.winning_numbers.contains(num) {
                    return 1;
                }
            }

            if card.winning_numbers.contains(num) {
                return acc * 2;
            }

            acc
        })
    })
}

fn part2(instances: &HashMap<usize, usize>) -> usize {
    instances.values().sum()
}

fn get_cards(input: &String) -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();

    input.lines().for_each(|line| {
        let line = line.split(' ').skip(2).collect::<Vec<&str>>();
        let separator_index = line.iter().position(|sym| sym == &"|").unwrap(); // The separator will appear in every line, so we can unwrap safely

        let winning_numbers = line[..separator_index]
            .iter()
            .filter_map(|num| num.parse::<u16>().ok())
            .collect::<Vec<u16>>();

        let numbers = line[separator_index..]
            .iter()
            .filter_map(|num| num.parse::<u16>().ok())
            .collect::<Vec<u16>>();

        cards.push(Card {
            winning_numbers,
            numbers,
        });
    });

    cards
}

fn get_card_instances(cards: &Vec<Card>) -> HashMap<usize, usize> {
    let mut card_instances: HashMap<usize, usize> = HashMap::new();

    cards.iter().enumerate().for_each(|(index, card)| {
        let matching = card.numbers.iter().fold(0, |acc, num| {
            if card.winning_numbers.contains(num) {
                return acc + 1;
            }
            acc
        });
        if card_instances.get(&index).is_none() {
            card_instances.insert(index, 1);
        }

        (1..=matching).for_each(|i| {
            let current_instances = card_instances.get(&index).unwrap();

            let instances = if let Some(instance) = card_instances.get(&(index + i)) {
                *instance
            } else {
                1
            };

            card_instances.insert(index + i, instances + current_instances);
        });
    });

    card_instances
}
