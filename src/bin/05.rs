advent_of_code::solution!(5);

type SeedSoilMapping = Vec<(isize, isize, isize)>;
type SoilFertilizerMapping = Vec<(isize, isize, isize)>;
type FertilizerWaterMapping = Vec<(isize, isize, isize)>;
type WaterLightMapping = Vec<(isize, isize, isize)>;
type LightTempMapping = Vec<(isize, isize, isize)>;
type TempHumidityMapping = Vec<(isize, isize, isize)>;
type HumidityLocationMapping = Vec<(isize, isize, isize)>;

fn parse_mapping(lines: &mut dyn Iterator<Item = &str>) -> Vec<(isize, isize, isize)> {
    lines
        .skip(1) // Skip the header line
        .take_while(|&line| !line.trim().is_empty()) // Stop parsing when an empty line is encountered
        .filter_map(|line| {
            let values: Vec<isize> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap_or_default())
                .collect();
            if values.len() == 3 {
                Some((values[0], values[1], values[2]))
            } else {
                None
            }
        })
        .collect()
}

fn parse_input(
    input: &str,
) -> (
    Vec<isize>,
    SeedSoilMapping,
    SoilFertilizerMapping,
    FertilizerWaterMapping,
    WaterLightMapping,
    LightTempMapping,
    TempHumidityMapping,
    HumidityLocationMapping,
) {
    let mut lines = input.lines();

    // Parse the first line
    let seeds: Vec<isize> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    // Parse seed-to-soil map
    let seed_to_soil = parse_mapping(&mut lines);

    // Parse soil-to-fertilizer map
    let soil_to_fertilizer = parse_mapping(&mut lines);

    // Parse fertilizer-to-water map
    let fertilizer_to_water = parse_mapping(&mut lines);

    // Parse water-to-light map
    let water_to_light = parse_mapping(&mut lines);

    // Parse light-to-temperature map
    let light_to_temp = parse_mapping(&mut lines);

    // Parse temperature-to-humidity map
    let temp_to_humidity = parse_mapping(&mut lines);

    // Parse humidity-to-location map
    let humidity_to_location = parse_mapping(&mut lines);

    (
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temp,
        temp_to_humidity,
        humidity_to_location,
    )
}

fn get_category_dst(category_src: &isize, mapping: &[(isize, isize, isize)]) -> isize {
    let mut category_dst: isize = -1;
    for ss_mapping in mapping.iter() {
        if ss_mapping.1 <= *category_src && ss_mapping.1 + ss_mapping.2 >= *category_src {
            category_dst = ss_mapping.0 + (category_src - ss_mapping.1);
            break;
        }
    }
    if category_dst == -1 {
        category_dst = *category_src;
    }
    //print!("{}: {} ", category_dst_str, category_dst);
    category_dst
}

pub fn part_one(input: &str) -> Option<u32> {
    let (
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temp,
        temp_to_humidity,
        humidity_to_location,
    ) = parse_input(input);

    let mut locations: Vec<isize> = Vec::new();
    get_min_location(
        seeds,
        &seed_to_soil,
        &soil_to_fertilizer,
        &fertilizer_to_water,
        &water_to_light,
        &light_to_temp,
        &temp_to_humidity,
        &humidity_to_location,
        &mut locations,
    );
    Some(*locations.iter().min().unwrap() as u32)
}

#[allow(clippy::too_many_arguments)]
fn get_min_location(
    seeds: Vec<isize>,
    seed_to_soil: &SeedSoilMapping,
    soil_to_fertilizer: &SoilFertilizerMapping,
    fertilizer_to_water: &FertilizerWaterMapping,
    water_to_light: &WaterLightMapping,
    light_to_temp: &LightTempMapping,
    temp_to_humidity: &TempHumidityMapping,
    humidity_to_location: &HumidityLocationMapping,
    locations: &mut Vec<isize>,
) {
    for seed in seeds.iter() {
        //print!("seed: {} ", *seed);
        let soil = get_category_dst(seed, seed_to_soil);
        let fertlizer = get_category_dst(&soil, soil_to_fertilizer);
        let water = get_category_dst(&fertlizer, fertilizer_to_water);
        let light = get_category_dst(&water, water_to_light);
        let temp = get_category_dst(&light, light_to_temp);
        let humidity = get_category_dst(&temp, temp_to_humidity);
        let location = get_category_dst(&humidity, humidity_to_location);
        locations.push(location);
        //println!()
    }
}

fn expand_pairs(input: Vec<isize>) -> Vec<isize> {
    let mut result: Vec<isize> = Vec::new();

    // Iterate over pairs of start and range values
    for i in (0..input.len()).step_by(2) {
        if i + 1 < input.len() {
            let start = input[i];
            let range = input[i + 1];

            // Generate the sequence of values and add them to the result vector
            result.extend((start..start + range).collect::<Vec<isize>>());
        }
    }

    result
}

pub fn part_two(input: &str) -> Option<u32> {
    let (
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temp,
        temp_to_humidity,
        humidity_to_location,
    ) = parse_input(input);

    let updated_seeds_list = expand_pairs(seeds);
    //println!("inc in seeds:{}", updated_seeds_list.len() - seeds.len());
    let mut locations: Vec<isize> = Vec::new();
    get_min_location(
        updated_seeds_list,
        &seed_to_soil,
        &soil_to_fertilizer,
        &fertilizer_to_water,
        &water_to_light,
        &light_to_temp,
        &temp_to_humidity,
        &humidity_to_location,
        &mut locations,
    );
    Some(*locations.iter().min().unwrap() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
