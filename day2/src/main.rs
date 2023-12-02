use common;

const BAG: Set = Set {
    red: 12,
    green: 13,
    blue: 14,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Set {
    red: u16,
    green: u16,
    blue: u16,
}

impl Set {
    fn empty() -> Set {
        Set {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

#[derive(Debug)]
struct Game {
    id: u16,
    sets: Vec<Set>,
}

fn main() {
    let input = common::read_input_file("input.txt");

    let mut games: Vec<Game> = parse_games(&input);

    let part1_result = part1(&games);

    println!("Part 1 Result: {part1_result}");

    let part2_result = part2(&mut games);

    println!("Part 2 Result: {part2_result}");
}

fn part1(games: &Vec<Game>) -> u32 {
    let mut sum = 0;

    for game in games.iter() {        
        let mut is_set_possible = true;

        for set in game.sets.iter() {
            if set.red > BAG.red || set.green > BAG.green || set.blue > BAG.blue {
                is_set_possible = false;
            }
        }

        if is_set_possible {
            sum += game.id as u32;
        }
    }

    sum
}

fn part2(games: &mut Vec<Game>) -> u32 {
    let mut sum = 0;

    for game in games.iter_mut() {
        let minimum_set = minimum_set(&game.sets);

        sum += (minimum_set.red * minimum_set.green * minimum_set.blue) as u32;
    }

    sum
}

fn minimum_set(sets: &Vec<Set>) -> Set {
    let (mut largest_red, mut largest_green, mut largest_blue) = (0, 0, 0);

    for set in sets {
        if set.red > largest_red {
            largest_red = set.red;
        }

        if set.green > largest_green {
            largest_green = set.green;
        }

        if set.blue > largest_blue {
            largest_blue = set.blue;
        }
    }

    Set {
        red: largest_red,
        green: largest_green,
        blue: largest_blue
    }
}

fn parse_games(input: &String) -> Vec<Game> {
    let mut games = Vec::new();
    for line in input.lines() {
        let line = line.split(' ').collect::<Vec<_>>();
        games.push(parse_game(&line));
    }

    games
}

fn parse_game(line: &Vec<&str>) -> Game {
    let mut id = 0;
    let mut sets: Vec<Set> = Vec::new();

    let mut current_set = Set::empty();

    let mut iter = line.iter();

    while let Some(item) = iter.next() {
        match *item {
            "Game" => {
                let mut next = iter.next().unwrap().to_string(); 
                next = next.replace(":", "");

                id = next.parse::<u16>().unwrap();
            }
            
            _ => {
                if let Ok(item) = item.parse::<u16>() {
                    let mut peekable = iter.clone().peekable();
                    let color = peekable.peek().unwrap();

                    match color.replace(",", "").replace(";", "").as_str() {
                        "red" => current_set.red += item,
                        "green" => current_set.green += item,
                        "blue" => current_set.blue += item,
                        _ => {},
                    }

                    if color.contains(';') {
                        sets.push(current_set);
                        current_set = Set::empty();
                    }
                }
            },
        }
    };

    // Forcefully push the final set
    sets.push(current_set);

    let game = Game { id, sets };

    game
}
