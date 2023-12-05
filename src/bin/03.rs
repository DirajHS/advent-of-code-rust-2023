use regex::Regex;

use advent_of_code::point::{Point, DIAGONALS};

advent_of_code::solution!(3);

pub struct Schematic {
    gears: Vec<(char, Point)>,
    symbols: Vec<(char, Point)>,
    numbers: Vec<(String, Point)>,
}

fn parse_engine_schematic(input: &str) -> Option<Schematic> {
    let mut numbers: Vec<(String, Point)> = vec![];
    let num_re = Regex::new(r"\d+").unwrap();
    for (row, line) in input.lines().enumerate() {
        for num_match in num_re.find_iter(line) {
            numbers.push((
                num_match.as_str().parse().unwrap(),
                Point::new(row as i32, num_match.start() as i32),
            ));
        }
    }
    let mut symbols: Vec<(char, Point)> = vec![];
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if !ch.is_ascii_digit() && ch != '.' {
                symbols.push((ch, Point::new(row as i32, col as i32)));
            }
        }
    }
    let mut gears: Vec<(char, Point)> = vec![];
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '*' {
                gears.push((ch, Point::new(row as i32, col as i32)));
            }
        }
    }
    Some(Schematic {
        gears,
        symbols,
        numbers,
    })
}

pub fn part_one(input: &str) -> Option<usize> {
    let Schematic {
        numbers, symbols, ..
    } = parse_engine_schematic(input).unwrap();
    let mut part_numbers: usize = 0;
    for (number, starting_point) in numbers {
        let num_end_pos = Point::new(
            starting_point.x,
            starting_point.y + number.to_string().len() as i32 - 1,
        );
        for (_, symbol_pos) in symbols.iter() {
            let neighbors = DIAGONALS
                .into_iter()
                .map(|diagonal| diagonal + *symbol_pos)
                .collect::<Vec<Point>>();
            for neighbor in neighbors.iter() {
                if neighbor.x == starting_point.x
                    && neighbor.y >= starting_point.y
                    && neighbor.y <= num_end_pos.y
                {
                    part_numbers += number.parse::<usize>().unwrap_or_default();
                    break;
                }
            }
        }
    }
    Some(part_numbers)
}

pub fn part_two(input: &str) -> Option<u32> {
    let Schematic {
        numbers,
        symbols: _symbols,
        gears,
    } = parse_engine_schematic(input).unwrap();
    let mut gear_ratios: Vec<u32> = vec![];
    for (_, gear_pos) in gears.iter() {
        let neighbors = DIAGONALS
            .into_iter()
            .map(|diagonal| diagonal + *gear_pos)
            .collect::<Vec<Point>>();
        let mut adjacent_nums: Vec<u32> = vec![];
        for (num_str, num_start_pos) in numbers.iter() {
            let num_end_pos =
                Point::new(num_start_pos.x, num_start_pos.y + num_str.len() as i32 - 1);
            for neighbor in neighbors.iter() {
                if neighbor.x == num_start_pos.x
                    && neighbor.y >= num_start_pos.y
                    && neighbor.y <= num_end_pos.y
                {
                    adjacent_nums.push(num_str.parse::<u32>().unwrap());
                    break;
                }
            }
        }
        if adjacent_nums.len() == 2 {
            gear_ratios.push(adjacent_nums.iter().product());
        }
    }
    Some(gear_ratios.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
