use std::str::FromStr;

use regex::Regex;

advent_of_code::solution!(4);

#[derive(Clone)]
struct ParsedCard {
    card_number: u32,
    winning_numbers: Vec<u32>,
    current_numbers: Vec<u32>,
}

impl ParsedCard {
    fn count_wins(&self) -> usize {
        self.current_numbers
            .iter()
            .filter(|x| self.winning_numbers.contains(x))
            .count()
    }
}

impl FromStr for ParsedCard {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(|c| c == ':' || c == '|').collect();

        if parts.len() != 3 {
            return Err("Invalid input format");
        }

        let re = Regex::new(r"Card (\d+)").unwrap();
        let card_number: u32 = if let Some(captures) = re.captures(parts[0]) {
            if let Some(number_str) = captures.get(1) {
                number_str.as_str().parse::<u32>().unwrap_or_default()
            } else {
                0
            }
        } else {
            0
        };
        let numbers_before_pipe: Vec<u32> = parts[1]
            .split_whitespace()
            .map(|num| num.parse().map_err(|_| "Invalid number in the first list"))
            .collect::<Result<_, _>>()?;
        let numbers_after_pipe: Vec<u32> = parts[2]
            .split_whitespace()
            .map(|num| num.parse().map_err(|_| "Invalid number in the second list"))
            .collect::<Result<_, _>>()?;

        Ok(ParsedCard {
            card_number,
            winning_numbers: numbers_before_pipe,
            current_numbers: numbers_after_pipe,
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut points: u32 = 0;
    for line in input.lines() {
        let mut current_card_point: u32 = 0;
        match line.parse::<ParsedCard>() {
            Ok(parsed_card) => {
                println!("Card Number: {}", parsed_card.card_number);
                println!("Numbers before pipe: {:?}", parsed_card.winning_numbers);
                println!("Numbers after pipe: {:?}", parsed_card.current_numbers);
                let winning_numbers: Vec<u32> = parsed_card
                    .winning_numbers
                    .iter()
                    .filter(|number| parsed_card.current_numbers.contains(number))
                    .cloned()
                    .collect();
                points += if winning_numbers.len() == 1 {
                    current_card_point = 1;
                    1
                } else if !winning_numbers.is_empty() {
                    for number in 0..winning_numbers.len() {
                        if number == 0 {
                            current_card_point = 1;
                        } else {
                            current_card_point *= 2;
                        }
                    }
                    current_card_point
                } else {
                    0
                };

                println!("points: {points}");
            }
            Err(err) => {
                eprintln!("Error parsing input: {}", err);
            }
        }
    }
    Some(points)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cards = Vec::new();
    for line in input.lines() {
        let cc = line.parse::<ParsedCard>().unwrap();
        cards.push(cc.clone());
    }
    let mut cards_count = vec![1; cards.len()];

    for (idx, card) in cards.iter().enumerate() {
        let winning_numbers_count = card
            .current_numbers
            .iter()
            .filter(|number| card.winning_numbers.contains(number))
            .cloned()
            .count();

        for card_number in (idx + 1)..=(winning_numbers_count + idx) {
            cards_count[card_number] += cards_count[idx];
        }
    }
    let total_cards: usize = cards_count.iter().sum::<usize>();
    Some(total_cards as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
