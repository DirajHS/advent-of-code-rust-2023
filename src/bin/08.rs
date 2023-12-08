use std::collections::HashMap;

advent_of_code::solution!(8);

type Node = String;
type Left = String;
type Right = String;

fn parse_input(input: &str) -> (Vec<char>, HashMap<Node, (Left, Right)>) {
    let mut chars = Vec::new();
    let mut node_map = HashMap::<Node, (Left, Right)>::new();

    for line in input.lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.is_empty() {
            // Skip empty lines
            continue;
        }

        if tokens.len() == 1 {
            // Process the first line to Vec<char>
            chars = tokens[0].chars().collect();
        } else {
            // Process lines with the format: <string> = (<string>, <string>)
            let label = tokens[0];
            let left_label = tokens[2].trim_matches(|c| c == '(' || c == ',' || c == ')');
            let right_label = tokens[3].trim_matches(|c| c == '(' || c == ',' || c == ')');

            node_map
                .entry(label.parse().unwrap())
                .or_insert_with(|| (left_label.parse().unwrap(), right_label.parse().unwrap()));
        }
    }

    (chars, node_map)
}

fn get_next_node_from_mapping(
    direction: char,
    current_node: &Node,
    node_mapping: &HashMap<Node, (Left, Right)>,
) -> Node {
    match direction {
        'L' => node_mapping.get(current_node).unwrap().0.parse().unwrap(),
        'R' => node_mapping.get(current_node).unwrap().1.parse().unwrap(),
        _ => panic!("Invalid direction: {}", direction),
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (directions, node_mapping) = parse_input(input);
    //println!("directions: {:?}", directions);
    //println!("node_mapping: {:?}", node_mapping);
    let mut destination_found = false;
    let mut steps: usize = 0;
    let mut directions_idx: usize = 0;
    let mut current_node: String = "AAA".to_string();
    while !destination_found {
        steps += 1;
        let direction = directions[directions_idx % directions.len()];
        let next_node: String = get_next_node_from_mapping(direction, &current_node, &node_mapping);
        if next_node == "ZZZ" {
            destination_found = true;
        } else {
            current_node = next_node;
        }
        directions_idx += 1;
    }
    Some(steps)
}

fn find_nodes_ending_with_a(map: &HashMap<Node, (Left, Right)>) -> Vec<Node> {
    let mut result = Vec::new();

    for (node, (_left, _right)) in map.iter() {
        if node.ends_with('A') {
            result.push(node.clone());
        }
    }
    result
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let (directions, node_mapping) = parse_input(input);
    //println!("directions: {:?}", directions);
    //println!("node_mapping: {:?}", node_mapping);
    let current_nodes: Vec<Node> = find_nodes_ending_with_a(&node_mapping);
    let mut node_steps: Vec<usize> = Vec::new();

    //println!("starting nodes: {:?}", current_nodes);
    for node in current_nodes.iter() {
        let mut steps: usize = 0;
        let mut directions_idx: usize = 0;
        let mut current_node = node.clone();
        let mut destination_found = false;
        while !destination_found {
            steps += 1;
            let direction = directions[directions_idx % directions.len()];
            let next_node: String =
                get_next_node_from_mapping(direction, &current_node, &node_mapping);
            if next_node.ends_with('Z') {
                destination_found = true;
            } else {
                current_node = next_node;
            }
            directions_idx += 1;
        }
        node_steps.push(steps);
    }
    let mut result = node_steps[0];

    for &num in node_steps.iter().skip(1) {
        result = lcm(result, num);
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
