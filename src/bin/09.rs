advent_of_code::solution!(9);

fn get_pairwise_diff(mut data: Vec<isize>) -> Vec<Vec<isize>> {
    let mut diff_history = Vec::new();
    diff_history.push(data.clone());

    while data.iter().any(|&x| x != 0) {
        let mut current_diff = Vec::new();
        for i in 0..data.len() - 1 {
            let diff = data[i + 1] - data[i];
            current_diff.push(diff);
            data[i] = diff;
        }
        data.pop();
        diff_history.push(current_diff.clone());
        //println!("pairwise diff: {:?}", current_diff);
    }
    diff_history
}

fn parse_input(input: &str) -> Vec<Vec<isize>> {
    return input
        .lines()
        .map(|line| {
            //println!("line: {}", &line);
            line.split_whitespace()
                .map(|reading| reading.parse::<isize>().unwrap())
                .collect()
        })
        .collect();
}

pub fn part_one(input: &str) -> Option<isize> {
    let sensor_data: Vec<Vec<isize>> = parse_input(input);
    let prediction: isize = sensor_data
        .iter()
        .map(|data| get_pairwise_diff(data.clone()))
        .map(|diff_history| {
            diff_history
                .iter()
                .rev()
                .filter_map(|diff| diff.last().cloned())
                .sum::<isize>()
        })
        .sum();
    Some(prediction)
}

pub fn part_two(input: &str) -> Option<isize> {
    let sensor_data: Vec<Vec<isize>> = parse_input(input);
    let prediction: isize = sensor_data
        .iter()
        .map(|data| get_pairwise_diff(data.clone()))
        .map(|diff_history| {
            diff_history
                .iter()
                .rev()
                .filter_map(|diff| diff.first().cloned())
                .fold(0, |acc, x| x - acc)
        })
        .sum();
    Some(prediction)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
