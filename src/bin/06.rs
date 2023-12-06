advent_of_code::solution!(6);

fn get_race_data(input: &str) -> Vec<(usize, usize)> {
    let distances = input
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap());
    let times = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap());
    let race_data: Vec<(usize, usize)> = times.into_iter().zip(distances.into_iter()).collect();
    race_data
}

fn find_min_time_hold(time: usize, max_distance: usize) -> usize {
    let mut range_start = 0;
    let mut start: usize = 0;
    let mut end: usize = time;

    while start <= end {
        let speed = (start + end) / 2;
        let remaining_time = time - speed;
        let total_distance = speed * remaining_time;
        if total_distance > max_distance {
            range_start = speed;
            end = speed - 1;
        } else {
            start = speed + 1;
        }
    }
    range_start
}

fn find_max_time_hold(time: usize, max_distance: usize) -> usize {
    let mut range_end = 0;
    let mut start: usize = 0;
    let mut end: usize = time;

    while start <= end {
        let speed = (start + end) / 2;
        let remaining_time = time - speed;
        let total_distance = speed * remaining_time;
        if total_distance > max_distance {
            range_end = speed;
            start = speed + 1;
        } else {
            end = speed - 1;
        }
    }
    range_end
}

pub fn part_one(input: &str) -> Option<usize> {
    let race_data = get_race_data(input);
    let mut result: usize = 1;
    let ways_possible: Vec<usize> = race_data
        .iter()
        .map(|race| find_max_time_hold(race.0, race.1) - find_min_time_hold(race.0, race.1) + 1)
        .collect();
    for possible_way in ways_possible {
        result *= possible_way;
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let race_data = get_race_data(input);
    let combined_time_str: String = race_data.iter().map(|race| race.0.to_string()).collect();
    let combined_time = combined_time_str.parse::<usize>().unwrap();
    let combined_distance_str: String = race_data.iter().map(|race| race.1.to_string()).collect();
    let combined_distance = combined_distance_str.parse::<usize>().unwrap();
    let ways = find_max_time_hold(combined_time, combined_distance)
        - find_min_time_hold(combined_time, combined_distance)
        + 1;
    Some(ways)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
