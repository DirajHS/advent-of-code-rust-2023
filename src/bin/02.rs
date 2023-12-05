use std::cmp::max;
use std::collections::HashMap;

use regex::Regex;

advent_of_code::solution!(2);

#[derive(Debug)]
struct Game {
    id: usize,
    //(R, G, B)
    cubes_drawn: Vec<(usize, usize, usize)>,
}

impl Game {
    fn new_game(id: usize) -> Game {
        Game {
            id,
            cubes_drawn: Vec::new(),
        }
    }
    fn insert_subset(&mut self, red: usize, blue: usize, green: usize) {
        self.cubes_drawn.push((red, green, blue));
    }
}
pub fn part_one(input: &str) -> Option<usize> {
    let mut sum: usize = 0;
    //(R, G, B)
    let max_allowed: (usize, usize, usize) = (12, 13, 14);
    for line in input.lines() {
        let game = parse_line(line);
        let mut is_game_valid = true;
        for game_subset in game.cubes_drawn {
            if game_subset.0.gt(&max_allowed.0)
                || game_subset.1.gt(&max_allowed.1)
                || game_subset.2.gt(&max_allowed.2)
            {
                is_game_valid = false;
                break;
            }
        }
        if is_game_valid {
            sum += game.id;
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut sum: usize = 0;
    //(R, G, B)
    for line in input.lines() {
        let game = parse_line(line);
        let mut min_red_cubes: usize = usize::MIN;
        let mut min_green_cubes: usize = usize::MIN;
        let mut min_blue_cubes: usize = usize::MIN;
        for game_subset in game.cubes_drawn {
            min_red_cubes = max(min_red_cubes, game_subset.0);
            min_green_cubes = max(min_green_cubes, game_subset.1);
            min_blue_cubes = max(min_blue_cubes, game_subset.2);
        }
        sum += min_red_cubes * min_green_cubes * min_blue_cubes;
        //println!("min red: {}, green: {}, blue: {}, power: {}", min_red_cubes, min_green_cubes, min_blue_cubes, min_red_cubes*min_green_cubes*min_blue_cubes);
    }
    Some(sum)
}

fn parse_line(line: &str) -> Game {
    let game_id: &str = line
        .trim()
        .split(':')
        .next()
        .unwrap()
        .split(' ')
        .last()
        .unwrap();
    let cubes = line
        .trim_start()
        .split(':')
        .last()
        .unwrap()
        .split(';')
        .collect::<Vec<&str>>();
    //println!("game: {}", game_id);
    let re = Regex::new(r"(\d+) (red|green|blue)").unwrap();

    let mut game = Game::new_game(game_id.parse().unwrap());
    for cube in cubes {
        let mut red_cubes: usize = 0;
        let mut blue_cubes: usize = 0;
        let mut green_cubes: usize = 0;
        let mut color_count: HashMap<String, usize> = HashMap::new();
        for capture in re.captures_iter(cube) {
            let number: usize = capture[1].parse().unwrap();
            let color = capture[2].to_string();
            *color_count.entry(color).or_insert(0) += number;
        }
        for color in color_count {
            match color.0.as_str() {
                "green" => {
                    green_cubes += color.1;
                }
                "red" => {
                    red_cubes += color.1;
                }
                "blue" => {
                    blue_cubes += color.1;
                }
                _ => {}
            }
        }
        game.insert_subset(red_cubes, blue_cubes, green_cubes);
    }
    //println!("game: {:?}", game);
    game
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
